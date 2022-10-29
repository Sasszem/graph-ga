x = [logspace(log10(300000), log10(2500000), 15), logspace(log10(4800000), log10(20000000), 15)];

bp_att = -46;
bp_rippl = 3;
bp_pen1 = 100;
bp_pen2 = 10000;

for k = x
disp(sprintf("(%f, %f, %f, %f, %f),", k, bp_att, bp_pen1, bp_pen2, bp_rippl));
end