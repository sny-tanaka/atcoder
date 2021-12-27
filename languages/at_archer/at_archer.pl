#!/usr/bin/perl
use strict;
use warnings;
use POSIX 'floor';

sub get_stdin_split {
    my $line=<STDIN>;
    chomp($line);
    return split(/ /, $line);
}
sub main {
    my ($n, $m, $d) = get_stdin_split();
    my @r = get_stdin_split();
    my @s = get_stdin_split();
    my $answer = solve($n, $m, $d, \@r, \@s);
    print "$answer\n"
}

sub solve {
    # get arguments
    my ($n, $m, $d, $r, $s) = @_;

    # process
    my $a = -floor($n * $d / 2);
    my @tmp_list = ((map { -$_ } (reverse @$r))[0..($m - 1)], (map { $_ + 1 } @$r)[1..$m]);
    my @l = @tmp_list[0..2 * ($m - 1)];
    my @r_list = @tmp_list[1..2 * $m - 1];
    my @s_list = ((reverse @$s), @$s[1..$m]);
    my @imos = (0) x ($d + 1);
    for(my $i = 0; $i < 2 * $m - 1; $i++) {
		if($l[$i] - $a >= $n * $d) {
			@imos[0] -= $n * @s_list[$i];
			@imos[$d] += $n * @s_list[$i];
		}
		elsif($l[$i] - $a >= 1) {
			@imos[0] -= floor(($l[$i] - $a + $d) / $d) * @s_list[$i];
			@imos[($l[$i] - $a) % $d] += @s_list[$i];
			@imos[$d] += floor(($l[$i] - $a) / $d) * @s_list[$i];
		}
		if(@r_list[$i] - $a >= $n * $d) {
			@imos[0] += $n * @s_list[$i];
			@imos[$d] -= $n * @s_list[$i];
		}
		elsif(@r_list[$i] - $a >= 1) {
			@imos[0] += floor((@r_list[$i] - $a + $d) / $d) * @s_list[$i];
			@imos[(@r_list[$i] - $a) % $d] -= @s_list[$i];
			@imos[$d] -= floor((@r_list[$i] - $a) / $d) * @s_list[$i];
		}
    }
    my $val = 0;
    my $score = 0;
    for(my $i = 0; $i < $d; $i++) {
        $val += @imos[$i];
        if($val > $score) {
            $score = $val;
        }
    }
    return $score;
}

main();
