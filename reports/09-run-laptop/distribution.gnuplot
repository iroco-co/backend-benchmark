set terminal pngcairo size 800,600 enhanced font 'Arial,10'
set output "rust_dist.png"
set boxwidth 0.7 absolute
set style fill solid 1.0 noborder
set title "request distribution time"
set grid y
set xlabel "request latency (ms)"
set ylabel "number of requests"

bin_width = 1;

bin_number(x) = floor(x/bin_width)

rounded(x) = bin_width * ( bin_number(x) )

plot 'rust.tsv' using (rounded($9)):(1) smooth frequency with boxes title "requests"
