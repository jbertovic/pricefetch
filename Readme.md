Following along with a project on LiveProject.manning.com (Building a stock trading cli with async-streams in rust).

This is my code while participating in the project.

Finished Milestone 2 - Mar 14, 2021 - added intervals and actors to retrieve quotes and procss data.
Finished Milestone 3 - Mar 22, 2021 - split into workspaces (cli / library) and added benchmarks.
Finished Milestone 4 - Mar 27, 2021 - added csv output, added server component and refactored a datastamp struct


```
pricetracker 0.4
Jas B. <jas@bertovic.net>
LiveProject on Manning.com - simple tracker - Milestone 4.0

USAGE:
    pricefetch [FLAGS] [OPTIONS] --from <from> --sym <symbols>...

FLAGS:
    -h, --help       Prints help information
    -x, --server     starts server on localhost port 8080
    -V, --version    Prints version information

OPTIONS:
    -c, --csv <csv>           name of file to output csv format
    -f, --from <from>         from date specified as yyyy-mm-dd
    -p, --pool <pool>         number of downloader actors [default: 5]
    -s, --sym <symbols>...    symbols to fetch price data
```

## Example run using cargo

```
>cargo run --  -f 2021-01-01 -s AAPL MSFT GOOG INTC SPY       
period start,symbol,price,change %,min,max,30d avg
2021-01-01T00:00:00+00:00,AAPL,$121.42,0.06%,$120.13,$143.16,$131.73
2021-01-01T00:00:00+00:00,MSFT,$231.60,-0.06%,$212.25,$244.99,$236.99
2021-01-01T00:00:00+00:00,GOOG,$2108.54,-0.22%,$1728.24,$2128.31,$2029.59
2021-01-01T00:00:00+00:00,INTC,$60.74,-0.22%,$49.67,$63.19,$59.31
2021-01-01T00:00:00+00:00,SPY,$383.63,-0.04%,$368.79,$392.64,$385.11
```

## Example run starting server and outputing csv file

```
>cargo run -- -x -c test.csv -f 2021-01-01 -s TRP SPY VNQ LIT QQQ VZ INTC
period start,symbol,price,change %,min,max,30d avg
2021-01-01T00:00:00+00:00,LIT,$58.32,-12.04%,$55.15,$74.31,$62.03
2021-01-01T00:00:00+00:00,VNQ,$92.84,12.99%,$81.97,$92.98,$89.71
2021-01-01T00:00:00+00:00,QQQ,$316.00,2.16%,$299.94,$336.45,$318.20
2021-01-01T00:00:00+00:00,TRP,$47.73,16.87%,$40.84,$47.73,$45.22
2021-01-01T00:00:00+00:00,SPY,$395.98,7.37%,$368.79,$397.26,$389.23
2021-01-01T00:00:00+00:00,VZ,$58.18,-1.14%,$54.15,$59.29,$56.15
2021-01-01T00:00:00+00:00,INTC,$64.87,30.60%,$49.67,$65.78,$62.36
2021-01-01T00:00:00+00:00,LIT,$58.32,-12.04%,$55.15,$74.31,$62.03
2021-01-01T00:00:00+00:00,TRP,$47.73,16.87%,$40.84,$47.73,$45.22
```

## Output of csv file

```
>cat test.csv
period start,symbol,price,change %,min,max,30d avg
2021-01-01T00:00:00+00:00,VNQ,$92.84,12.99%,$81.97,$92.98,$89.71
2021-01-01T00:00:00+00:00,LIT,$58.32,-12.04%,$55.15,$74.31,$62.03
2021-01-01T00:00:00+00:00,QQQ,$316.00,2.16%,$299.94,$336.45,$318.20
2021-01-01T00:00:00+00:00,TRP,$47.73,16.87%,$40.84,$47.73,$45.22
2021-01-01T00:00:00+00:00,SPY,$395.98,7.37%,$368.79,$397.26,$389.23
2021-01-01T00:00:00+00:00,VZ,$58.18,-1.14%,$54.15,$59.29,$56.15
2021-01-01T00:00:00+00:00,INTC,$64.87,30.60%,$49.67,$65.78,$62.36

```

## Running curl to test server

```
>curl localhost:8080/text/5
period start,symbol,price,change %,min,max,30d avg
2021-01-01T00:00:00+00:00,VZ,$58.18,-1.14%,$54.15,$59.29,$56.15
2021-01-01T00:00:00+00:00,INTC,$64.87,30.60%,$49.67,$65.78,$62.36
2021-01-01T00:00:00+00:00,TRP,$47.73,16.87%,$40.84,$47.73,$45.22
2021-01-01T00:00:00+00:00,QQQ,$316.00,2.16%,$299.94,$336.45,$318.20
2021-01-01T00:00:00+00:00,SPY,$395.98,7.37%,$368.79,$397.26,$389.23

>curl localhost:8080/text/20?sym=VZ
period start,symbol,price,change %,min,max,30d avg
2021-01-01T00:00:00+00:00,VZ,$58.18,-1.14%,$54.15,$59.29,$56.15
2021-01-01T00:00:00+00:00,VZ,$58.18,-1.14%,$54.15,$59.29,$56.15
2021-01-01T00:00:00+00:00,VZ,$58.18,-1.14%,$54.15,$59.29,$56.15

>curl localhost:8080/tail/2
[{"from":"2021-01-01T00:00:00+00:00","symbol":"INTC","last":64.87,"change":30.6,"min":49.67,"max":65.78,"sma_30":62.36},{"from":"2021-01-01T00:00:00+00:00","symbol":"VZ","last":58.18,"change":-1.14,"min":54.15,"max":59.29,"sma_30":56.15}]

>curl localhost:8080/tail/20?sym=VZ
[{"from":"2021-01-01T00:00:00+00:00","symbol":"VZ","last":58.18,"change":-1.14,"min":54.15,"max":59.29,"sma_30":56.15},{"from":"2021-01-01T00:00:00+00:00","symbol":"VZ","last":58.18,"change":-1.14,"min":54.15,"max":59.29,"sma_30":56.15},{"from":"2021-01-01T00:00:00+00:00","symbol":"VZ","last":58.18,"change":-1.14,"min":54.15,"max":59.29,"sma_30":56.15}]

```
