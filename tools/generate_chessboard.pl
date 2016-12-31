#!/usr/bin/perl

# generate_chessboard.pl
# 
# Dumps a chessboard table with the offsets for each square.
#

use strict;
use warnings;
use feature 'say';
use Text::ASCIITable;

my @ranks = map { 8 - $_ } ( 0.. 7 );

my @files = ( 'A'..'H' );

my $table = Text::ASCIITable->new({ drawRowLine => 1 });

$table->setCols('', @files);

for my $row ( 0..7 ) {
    my $rank = $ranks[$row];
    my @row = ( $rank );
    for my $col ( 0..7 ) {
        my $file = $files[$col];
        push @row, offset( $row, $col );
    }
    $table->addRow( @row );
}

print $table;

sub offset {
    my ( $row, $col ) = @_;
    return ( ( $row << 3 ) | $col );
}




