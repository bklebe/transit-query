{
	description = "Querying MBTA GTFS data with https://github.com/obi1kenobi/trustfall";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

		crane = {
			url = "github:ipetkov/crane";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		flake-utils.url = "github:numtide/flake-utils";

		advisory-db = {
			url = "github:rustsec/advisory-db";
			flake = false;
		};
	};

	outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, ... }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs {
					inherit system;
				};

				inherit (pkgs) lib;

				craneLib = crane.lib.${system};
				src = let
					# Only keeps markdown files
					graphQLFilter = path: _type: builtins.match ".*/src/.*\.gql$" path != null;
					testData = path: _type: builtins.match ".*/test_data/.*$" path != null;
					qglOrCargo = path: type:
						builtins.any (fn: fn path type) [
							graphQLFilter
							testData
							craneLib.filterCargoSources
						];
				in lib.cleanSourceWith {
					src = craneLib.path ./.; # The original, unfiltered source
					filter = qglOrCargo;
				};

				# Common arguments can be set here to avoid repeating them later
				commonArgs = {
					inherit src;
					strictDeps = true;

					buildInputs = [
						# Add additional build inputs here
					] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
						# Additional darwin specific inputs can be set here
						pkgs.libiconv
						CoreFoundation
						Security
						SystemConfiguration
					]);
				};

				craneLibLLvmTools = craneLib.overrideToolchain
					(fenix.packages.${system}.complete.withComponents [
						"cargo"
						"llvm-tools"
						"rustc"
					]);

				# Build *just* the cargo dependencies, so we can reuse
				# all of that work (e.g. via cachix) when running in CI
				cargoArtifacts = craneLib.buildDepsOnly commonArgs;

				# Build the actual crate itself, reusing the dependency
				# artifacts from above.
				transit-query = craneLib.buildPackage (commonArgs // {
					inherit cargoArtifacts;
					doCheck = false;
					# GTFS_STATIC = self.packages.${system}.mbta-gtfs;
				});
			in
			{
				checks = {
					# Build the crate as part of `nix flake check` for convenience
					inherit transit-query;

					# Run clippy (and deny all warnings) on the crate source,
					# again, resuing the dependency artifacts from above.
					#
					# Note that this is done as a separate derivation so that
					# we can block the CI if there are issues here, but not
					# prevent downstream consumers from building our crate by itself.
					transit-query-clippy = craneLib.cargoClippy (commonArgs // {
						inherit cargoArtifacts;
						cargoClippyExtraArgs = "--all-targets -- --deny warnings";
					});

					transit-query-doc = craneLib.cargoDoc (commonArgs // {
						inherit cargoArtifacts;
					});

					# Check formatting
					transit-query-fmt = craneLib.cargoFmt {
						inherit src;
					};

					# Audit dependencies
					transit-query-audit = craneLib.cargoAudit {
						inherit src advisory-db;
					};

					# Run tests with cargo-nextest
					# Consider setting `doCheck = false` on `transit-query` if you do not want
					# the tests to run twice
					transit-query-nextest = craneLib.cargoNextest (commonArgs // {
						inherit cargoArtifacts;
						partitions = 1;
						partitionType = "count";
					});
				};

				packages = {
					inherit transit-query;
					default = transit-query;

					# transit-query-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
					# 	inherit cargoArtifacts;
					# });

					mbta-gtfs = pkgs.fetchzip {
						name = "MBTA GTFS Static";
						url = "https://cdn.mbta.com/MBTA_GTFS.zip";
						stripRoot = false;
						hash = "sha256-WB4q+DqB99ruj76wuU3VfBYmIZWFaBBGMaNFa6Z/QqI=";
					};
				};

				apps.default = flake-utils.lib.mkApp {
					drv = transit-query;
				};

				devShells.default = craneLib.devShell {
					# Inherit inputs from checks.
					checks = self.checks.${system};

					inputsFrom = [ self.packages.${system}.default ];

					# Additional dev-shell environment variables can be set directly
					# MY_CUSTOM_DEVELOPMENT_VAR = "something else";
					GTFS_STATIC = self.packages.${system}.mbta-gtfs;

					# Extra inputs can be added here; cargo and rustc are provided by default.
					packages = [
						pkgs.rust-analyzer
					];
				};
			});
}
