hexdump data/BATTLE1.LFD > dump1
hexdump data/BATTLE1.LFD_extract > dump2
diff --suppress-common-lines --report-identical-files -y dump1 dump2
rm dump1
rm dump2
