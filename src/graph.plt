set xrange [-100:100]
set yrange [-100:100]
set zrange [-100:100]

unset output
stats datafile
unset walls
#set xyplane 0
#animate=1
#ps variable pt 6

if(animate==1){
    set term gif animate delay 0
    set output outputfile
    do for [i=1:STATS_blocks] {
        splot datafile index i using 1:2:3:4 with points ps variable pt 7
    }
}
if(animate==0){
    set term qt
    splot for[i=1:STATS_blocks] datafile index i using 1:2:3:4 with dots title ''
}