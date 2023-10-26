# Visualization of timings for rdfpipe vs rdfpipe-rs
# tidyverse>=1.1.3 is the only dependency

library(tidyverse)

timings <- read_csv("timings.csv")

bench <- timings %>%
    rename(
        tool = command,
        thousand_lines = parameter_N,
        fmt = parameter_FMT
    ) %>%
    mutate(tool = case_when(
        str_detect(tool, "rdfpipe-rs") ~ "rdfpipe-rs",
        TRUE ~ "rdfpipe"
    )) %>%
    select(tool, mean, fmt, stddev, thousand_lines) %>%
    arrange(thousand_lines, tool)

ggplot(bench, aes(x = thousand_lines, y = log10(mean), color = tool)) +
    geom_line() +
    geom_ribbon(
        aes(
            y = log10(mean),
            ymin = log10(mean - stddev),
            ymax = log10(mean + stddev),
            fill = "lightgrey"
        ),
        alpha = .2
    ) +
    xlab("Thousands of lines parsed") +
    ylab("Log10 time (seconds)") +
    theme_bw(base_size = 18) +
    coord_fixed(ratio = 10) +
    facet_grid(~fmt, labeller = labeller(
        fmt = c(
            "ttl" = "ntriples -> turtle",
            "xml" = "ntriples -> xml"
        )
    ))
