#![no_std]
#![no_main]

extern crate avr_std_stub;

use avr_delay::delay_cycles;

#[no_mangle]
fn main() {}

/*

Interesting corner case values. They are picked to be right around the switch between lower to
higher number of bits for the dealy counter.

#!/bin/bash
while read i; do
    echo "#[inline(never)] #[no_mangle] pub fn test_${i}() { delay_cycles::<${i}>(); }"
done <<EOF
0
1
2
3
4
5
6
7
8
9
11
12
767
768
769
770
771
772
773
774
262_144
262_145
262_146
262_147
262_148
262_149
262_150
262_151
262_152
262_153
262_154
262_155
83_886_081
83_886_082
83_886_083
83_886_084
83_886_085
83_886_086
83_886_087
83_886_088
83_886_089
83_886_090
83_886_091
83_886_092
83_886_093
83_886_094
83_886_095
25_769_803_778
25_769_803_779
25_769_803_780
25_769_803_783
25_769_803_784
EOF

*/

#[inline(never)] #[no_mangle] pub fn test_0() { delay_cycles::<0>(); }
#[inline(never)] #[no_mangle] pub fn test_1() { delay_cycles::<1>(); }
#[inline(never)] #[no_mangle] pub fn test_2() { delay_cycles::<2>(); }
#[inline(never)] #[no_mangle] pub fn test_3() { delay_cycles::<3>(); }
#[inline(never)] #[no_mangle] pub fn test_4() { delay_cycles::<4>(); }
#[inline(never)] #[no_mangle] pub fn test_5() { delay_cycles::<5>(); }
#[inline(never)] #[no_mangle] pub fn test_6() { delay_cycles::<6>(); }
#[inline(never)] #[no_mangle] pub fn test_7() { delay_cycles::<7>(); }
#[inline(never)] #[no_mangle] pub fn test_8() { delay_cycles::<8>(); }
#[inline(never)] #[no_mangle] pub fn test_9() { delay_cycles::<9>(); }
#[inline(never)] #[no_mangle] pub fn test_11() { delay_cycles::<11>(); }
#[inline(never)] #[no_mangle] pub fn test_12() { delay_cycles::<12>(); }
#[inline(never)] #[no_mangle] pub fn test_767() { delay_cycles::<767>(); }
#[inline(never)] #[no_mangle] pub fn test_768() { delay_cycles::<768>(); }
#[inline(never)] #[no_mangle] pub fn test_769() { delay_cycles::<769>(); }
#[inline(never)] #[no_mangle] pub fn test_770() { delay_cycles::<770>(); }
#[inline(never)] #[no_mangle] pub fn test_771() { delay_cycles::<771>(); }
#[inline(never)] #[no_mangle] pub fn test_772() { delay_cycles::<772>(); }
#[inline(never)] #[no_mangle] pub fn test_773() { delay_cycles::<773>(); }
#[inline(never)] #[no_mangle] pub fn test_774() { delay_cycles::<774>(); }
#[inline(never)] #[no_mangle] pub fn test_262_144() { delay_cycles::<262_144>(); }
#[inline(never)] #[no_mangle] pub fn test_262_145() { delay_cycles::<262_145>(); }
#[inline(never)] #[no_mangle] pub fn test_262_146() { delay_cycles::<262_146>(); }
#[inline(never)] #[no_mangle] pub fn test_262_147() { delay_cycles::<262_147>(); }
#[inline(never)] #[no_mangle] pub fn test_262_148() { delay_cycles::<262_148>(); }
#[inline(never)] #[no_mangle] pub fn test_262_149() { delay_cycles::<262_149>(); }
#[inline(never)] #[no_mangle] pub fn test_262_150() { delay_cycles::<262_150>(); }
#[inline(never)] #[no_mangle] pub fn test_262_151() { delay_cycles::<262_151>(); }
#[inline(never)] #[no_mangle] pub fn test_262_152() { delay_cycles::<262_152>(); }
#[inline(never)] #[no_mangle] pub fn test_262_153() { delay_cycles::<262_153>(); }
#[inline(never)] #[no_mangle] pub fn test_262_154() { delay_cycles::<262_154>(); }
#[inline(never)] #[no_mangle] pub fn test_262_155() { delay_cycles::<262_155>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_081() { delay_cycles::<83_886_081>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_082() { delay_cycles::<83_886_082>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_083() { delay_cycles::<83_886_083>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_084() { delay_cycles::<83_886_084>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_085() { delay_cycles::<83_886_085>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_086() { delay_cycles::<83_886_086>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_087() { delay_cycles::<83_886_087>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_088() { delay_cycles::<83_886_088>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_089() { delay_cycles::<83_886_089>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_090() { delay_cycles::<83_886_090>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_091() { delay_cycles::<83_886_091>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_092() { delay_cycles::<83_886_092>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_093() { delay_cycles::<83_886_093>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_094() { delay_cycles::<83_886_094>(); }
#[inline(never)] #[no_mangle] pub fn test_83_886_095() { delay_cycles::<83_886_095>(); }
#[inline(never)] #[no_mangle] pub fn test_25_769_803_778() { delay_cycles::<25_769_803_778>(); }
#[inline(never)] #[no_mangle] pub fn test_25_769_803_779() { delay_cycles::<25_769_803_779>(); }
#[inline(never)] #[no_mangle] pub fn test_25_769_803_780() { delay_cycles::<25_769_803_780>(); }
#[inline(never)] #[no_mangle] pub fn test_25_769_803_783() { delay_cycles::<25_769_803_783>(); }
#[inline(never)] #[no_mangle] pub fn test_25_769_803_784() { delay_cycles::<25_769_803_784>(); }

// This shouldn't compile, but we don't have static assertion yet. The code produced is still
// correct. But it costs more than calling delay_cycles twice.
#[inline(never)] #[no_mangle] pub fn test_25_769_803_785_bad() { delay_cycles::<25_769_803_785>(); }

// This shouldn't compile, but we don't have static assertion yet. The code produced is still
// correct. But it costs more than calling delay_cycles twice.
#[inline(never)] #[no_mangle] pub fn test_25_853_952_778_bad() { delay_cycles::<25_853_952_778>(); }

// This shouldn't compile, but we don't have static assertion yet. The code produced is still
// correct. But it costs more than calling delay_cycles twice. This is the absolute limit.
#[inline(never)] #[no_mangle] pub fn test_25_853_952_779_bad() { delay_cycles::<25_853_952_779>(); }

// This shouldn't compile, but we don't have static assertion yet. This does overflow and should
// produces the same function as 25_769_803_778.
#[inline(never)] #[no_mangle] pub fn test_25_853_952_780_overflow() { delay_cycles::<25_853_952_780>(); }
