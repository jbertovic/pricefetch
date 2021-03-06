Following along with a project on LiveProject.manning.com (Building a stock trading cli with async-streams in rust).

This is my code while participating in the project.

```
Usage: pricefetch.exe <from> [<symbols...>]

Struct to contain CLI arguments and configuration Fetch close history from Yahoo Finance API

Options:
  --help            display usage information
```

Example run using cargo

```
>cargo run --  2021-01-01 AAPL MSFT GOOG INTC SPY       
period start,symbol,price,change %,min,max,30d avg
2021-01-01T00:00:00+00:00,AAPL,$121.42,0.06%,$120.13,$143.16,$131.73
2021-01-01T00:00:00+00:00,MSFT,$231.60,-0.06%,$212.25,$244.99,$236.99
2021-01-01T00:00:00+00:00,GOOG,$2108.54,-0.22%,$1728.24,$2128.31,$2029.59
2021-01-01T00:00:00+00:00,INTC,$60.74,-0.22%,$49.67,$63.19,$59.31
2021-01-01T00:00:00+00:00,SPY,$383.63,-0.04%,$368.79,$392.64,$385.11
```