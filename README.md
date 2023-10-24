# transit-query
`cargo run`
## e.g. get crowding information on every vehicle and their lat/long
```graphql
{
  Vehicle {
    lead_car: label @output
    latitude @output
    longitude @output

    trip_descriptor {
      route {
        route: long_name @output
      }
    }

    multi_carriage_details {
      label @output
      occupancy_percentage @output
      occupancy_status @output
    }
  }
}

```