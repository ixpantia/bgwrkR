# bgwrkR

This is an R package for running scheduled R jobs in the background.

## Usage

This package uses a "cron-like" syntax for scheduling. The main difference
with regular cron syntax is the addition of seconds. 

### Example

This example will run `script.R` according to the schedule given.

```r
#           sec  min   hour   day of month   month   day of week   year
schedule <- "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2"
bgwrkR::BackgroundWorker$new("script.R", schedule)
```

In the following example `script.R` will run every 10 seconds.

```r
# Notice that `year` is optional.
# Every schedule must have at least 6 arguments.
#
#            sec   min hour day  month day of week
schedule <- "*/10   *   *    *    *         *"
bgwrkR::BackgroundWorker$new("script.R", schedule)
```

