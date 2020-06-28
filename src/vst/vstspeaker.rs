use crate::vst::{Speaker, SpeakerArrangement};
pub const kSpeakerL: Speaker = 1;
pub const kSpeakerR: Speaker = 2;
pub const kSpeakerC: Speaker = 4;
pub const kSpeakerLfe: Speaker = 8;
pub const kSpeakerLs: Speaker = 16;
pub const kSpeakerRs: Speaker = 32;
pub const kSpeakerLc: Speaker = 64;
pub const kSpeakerRc: Speaker = 128;
pub const kSpeakerS: Speaker = 256;
pub const kSpeakerCs: Speaker = 256;
pub const kSpeakerSl: Speaker = 512;
pub const kSpeakerSr: Speaker = 1024;
pub const kSpeakerTc: Speaker = 2048;
pub const kSpeakerTfl: Speaker = 4096;
pub const kSpeakerTfc: Speaker = 8192;
pub const kSpeakerTfr: Speaker = 16384;
pub const kSpeakerTrl: Speaker = 32768;
pub const kSpeakerTrc: Speaker = 65536;
pub const kSpeakerTrr: Speaker = 131072;
pub const kSpeakerLfe2: Speaker = 262144;
pub const kSpeakerM: Speaker = 524288;
pub const kSpeakerACN0: Speaker = 1048576;
pub const kSpeakerACN1: Speaker = 2097152;
pub const kSpeakerACN2: Speaker = 4194304;
pub const kSpeakerACN3: Speaker = 8388608;
pub const kSpeakerACN4: Speaker = 274877906944;
pub const kSpeakerACN5: Speaker = 549755813888;
pub const kSpeakerACN6: Speaker = 1099511627776;
pub const kSpeakerACN7: Speaker = 2199023255552;
pub const kSpeakerACN8: Speaker = 4398046511104;
pub const kSpeakerACN9: Speaker = 8796093022208;
pub const kSpeakerACN10: Speaker = 17592186044416;
pub const kSpeakerACN11: Speaker = 35184372088832;
pub const kSpeakerACN12: Speaker = 70368744177664;
pub const kSpeakerACN13: Speaker = 140737488355328;
pub const kSpeakerACN14: Speaker = 281474976710656;
pub const kSpeakerACN15: Speaker = 562949953421312;
pub const kSpeakerTsl: Speaker = 16777216;
pub const kSpeakerTsr: Speaker = 33554432;
pub const kSpeakerLcs: Speaker = 67108864;
pub const kSpeakerRcs: Speaker = 134217728;
pub const kSpeakerBfl: Speaker = 268435456;
pub const kSpeakerBfc: Speaker = 536870912;
pub const kSpeakerBfr: Speaker = 1073741824;
pub const kSpeakerPl: Speaker = 2147483648;
pub const kSpeakerPr: Speaker = 4294967296;
pub const kSpeakerBsl: Speaker = 8589934592;
pub const kSpeakerBsr: Speaker = 17179869184;
pub const kSpeakerBrl: Speaker = 34359738368;
pub const kSpeakerBrc: Speaker = 68719476736;
pub const kSpeakerBrr: Speaker = 137438953472;
pub const kEmpty: SpeakerArrangement = 0;
pub const kMono: SpeakerArrangement = 524288;
pub const kStereo: SpeakerArrangement = 3;
pub const kStereoSurround: SpeakerArrangement = 48;
pub const kStereoCenter: SpeakerArrangement = 192;
pub const kStereoSide: SpeakerArrangement = 1536;
pub const kStereoCLfe: SpeakerArrangement = 12;
pub const kStereoTF: SpeakerArrangement = 20480;
pub const kStereoTS: SpeakerArrangement = 50331648;
pub const kStereoTR: SpeakerArrangement = 163840;
pub const kStereoBF: SpeakerArrangement = 1342177280;
pub const k30Cine: SpeakerArrangement = 7;
pub const k30Music: SpeakerArrangement = 259;
pub const k31Cine: SpeakerArrangement = 15;
pub const k31Music: SpeakerArrangement = 267;
pub const k40Cine: SpeakerArrangement = 263;
pub const k40Music: SpeakerArrangement = 51;
pub const k41Cine: SpeakerArrangement = 271;
pub const k41Music: SpeakerArrangement = 59;
pub const k50: SpeakerArrangement = 55;
pub const k51: SpeakerArrangement = 63;
pub const k60Cine: SpeakerArrangement = 311;
pub const k60Music: SpeakerArrangement = 1587;
pub const k61Cine: SpeakerArrangement = 319;
pub const k61Music: SpeakerArrangement = 1595;
pub const k70Cine: SpeakerArrangement = 247;
pub const k70Music: SpeakerArrangement = 1591;
pub const k71Cine: SpeakerArrangement = 255;
pub const k71CineFullFront: SpeakerArrangement = 255;
pub const k71CineFullRear: SpeakerArrangement = 201326655;
pub const k71Music: SpeakerArrangement = 1599;
pub const k71CineSideFill: SpeakerArrangement = 1599;
pub const k71Proximity: SpeakerArrangement = 6442451007;
pub const k80Cine: SpeakerArrangement = 503;
pub const k80Music: SpeakerArrangement = 1847;
pub const k81Cine: SpeakerArrangement = 511;
pub const k81Music: SpeakerArrangement = 1855;
pub const kAmbi1stOrderACN: SpeakerArrangement = 15728640;
pub const kAmbi2cdOrderACN: SpeakerArrangement = 8521230843904;
pub const kAmbi3rdOrderACN: SpeakerArrangement = 1125625044664320;
pub const k80Cube: SpeakerArrangement = 184371;
pub const k71CineTopCenter: SpeakerArrangement = 2367;
pub const k71CineCenterHigh: SpeakerArrangement = 8511;
pub const k71CineFrontHigh: SpeakerArrangement = 20543;
pub const k71MPEG3D: SpeakerArrangement = 20543;
pub const k71CineSideHigh: SpeakerArrangement = 50331711;
pub const k81MPEG3D: SpeakerArrangement = 536899643;
pub const k90: SpeakerArrangement = 184375;
pub const k50_4: SpeakerArrangement = 184375;
pub const k91: SpeakerArrangement = 184383;
pub const k51_4: SpeakerArrangement = 184383;
pub const k70_2: SpeakerArrangement = 50333239;
pub const k71_2: SpeakerArrangement = 50333247;
pub const k91Atmos: SpeakerArrangement = 50333247;
pub const k70_4: SpeakerArrangement = 185911;
pub const k71_4: SpeakerArrangement = 185919;
pub const k111MPEG3D: SpeakerArrangement = 185919;
pub const k70_6: SpeakerArrangement = 50517559;
pub const k71_6: SpeakerArrangement = 50517567;
pub const k100: SpeakerArrangement = 186423;
pub const k101: SpeakerArrangement = 186431;
pub const k101MPEG3D: SpeakerArrangement = 186431;
pub const k102: SpeakerArrangement = 454719;
pub const k110: SpeakerArrangement = 194615;
pub const k111: SpeakerArrangement = 194623;
pub const k122: SpeakerArrangement = 454911;
pub const k130: SpeakerArrangement = 196151;
pub const k131: SpeakerArrangement = 196159;
pub const k140: SpeakerArrangement = 173141055027;
pub const k222: SpeakerArrangement = 1929904127;
pub const kStringEmpty: &[u8; 1usize] = b"\0";
pub const kStringMono: &[u8; 5usize] = b"Mono\0";
pub const kStringStereo: &[u8; 7usize] = b"Stereo\0";
pub const kStringStereoR: &[u8; 15usize] = b"Stereo (Ls Rs)\0";
pub const kStringStereoC: &[u8; 15usize] = b"Stereo (Lc Rc)\0";
pub const kStringStereoSide: &[u8; 15usize] = b"Stereo (Sl Sr)\0";
pub const kStringStereoCLfe: &[u8; 15usize] = b"Stereo (C LFE)\0";
pub const kStringStereoTF: &[u8; 17usize] = b"Stereo (Tfl Tfr)\0";
pub const kStringStereoTS: &[u8; 17usize] = b"Stereo (Tsl Tsr)\0";
pub const kStringStereoTR: &[u8; 17usize] = b"Stereo (Trl Trr)\0";
pub const kStringStereoBF: &[u8; 17usize] = b"Stereo (Bfl Bfr)\0";
pub const kString30Cine: &[u8; 4usize] = b"LRC\0";
pub const kString30Music: &[u8; 4usize] = b"LRS\0";
pub const kString31Cine: &[u8; 8usize] = b"LRC+LFE\0";
pub const kString31Music: &[u8; 8usize] = b"LRS+LFE\0";
pub const kString40Cine: &[u8; 5usize] = b"LRCS\0";
pub const kString40Music: &[u8; 7usize] = b"Quadro\0";
pub const kString41Cine: &[u8; 9usize] = b"LRCS+LFE\0";
pub const kString41Music: &[u8; 11usize] = b"Quadro+LFE\0";
pub const kString50: &[u8; 4usize] = b"5.0\0";
pub const kString51: &[u8; 4usize] = b"5.1\0";
pub const kString60Cine: &[u8; 9usize] = b"6.0 Cine\0";
pub const kString60Music: &[u8; 10usize] = b"6.0 Music\0";
pub const kString61Cine: &[u8; 9usize] = b"6.1 Cine\0";
pub const kString61Music: &[u8; 10usize] = b"6.1 Music\0";
pub const kString70Cine: &[u8; 16usize] = b"7.0 Cine (SDDS)\0";
pub const kString70Music: &[u8; 18usize] = b"7.0 Music (Dolby)\0";
pub const kString71Cine: &[u8; 16usize] = b"7.1 Cine (SDDS)\0";
pub const kString71Music: &[u8; 18usize] = b"7.1 Music (Dolby)\0";
pub const kString71CineTopCenter: &[u8; 20usize] = b"7.1 Cine Top Center\0";
pub const kString71CineCenterHigh: &[u8; 21usize] = b"7.1 Cine Center High\0";
pub const kString71CineFrontHigh: &[u8; 20usize] = b"7.1 Cine Front High\0";
pub const kString71CineSideHigh: &[u8; 19usize] = b"7.1 Cine Side High\0";
pub const kString71CineFullRear: &[u8; 19usize] = b"7.1 Cine Full Rear\0";
pub const kString71Proximity: &[u8; 14usize] = b"7.1 Proximity\0";
pub const kString80Cine: &[u8; 9usize] = b"8.0 Cine\0";
pub const kString80Music: &[u8; 10usize] = b"8.0 Music\0";
pub const kString80Cube: &[u8; 9usize] = b"8.0 Cube\0";
pub const kString81Cine: &[u8; 9usize] = b"8.1 Cine\0";
pub const kString81Music: &[u8; 10usize] = b"8.1 Music\0";
pub const kString102: &[u8; 18usize] = b"10.2 Experimental\0";
pub const kString122: &[u8; 5usize] = b"12.2\0";
pub const kString50_4: &[u8; 6usize] = b"5.0.4\0";
pub const kString51_4: &[u8; 6usize] = b"5.1.4\0";
pub const kString70_2: &[u8; 6usize] = b"7.0.2\0";
pub const kString71_2: &[u8; 6usize] = b"7.1.2\0";
pub const kString70_4: &[u8; 6usize] = b"7.0.4\0";
pub const kString71_4: &[u8; 6usize] = b"7.1.4\0";
pub const kString70_6: &[u8; 6usize] = b"7.0.6\0";
pub const kString71_6: &[u8; 6usize] = b"7.1.6\0";
pub const kString100: &[u8; 13usize] = b"10.0 Auro-3D\0";
pub const kString101: &[u8; 13usize] = b"10.1 Auro-3D\0";
pub const kString110: &[u8; 13usize] = b"11.0 Auro-3D\0";
pub const kString111: &[u8; 13usize] = b"11.1 Auro-3D\0";
pub const kString130: &[u8; 13usize] = b"13.0 Auro-3D\0";
pub const kString131: &[u8; 13usize] = b"13.1 Auro-3D\0";
pub const kString81MPEG: &[u8; 9usize] = b"8.1 MPEG\0";
pub const kString140: &[u8; 5usize] = b"14.0\0";
pub const kString222: &[u8; 5usize] = b"22.2\0";
pub const kStringAmbi1stOrder: &[u8; 21usize] = b"1st Order Ambisonics\0";
pub const kStringAmbi2cdOrder: &[u8; 21usize] = b"2nd Order Ambisonics\0";
pub const kStringAmbi3rdOrder: &[u8; 21usize] = b"3rd Order Ambisonics\0";
pub const kStringMonoS: &[u8; 2usize] = b"M\0";
pub const kStringStereoS: &[u8; 4usize] = b"L R\0";
pub const kStringStereoRS: &[u8; 6usize] = b"Ls Rs\0";
pub const kStringStereoCS: &[u8; 6usize] = b"Lc Rc\0";
pub const kStringStereoSS: &[u8; 6usize] = b"Sl Sr\0";
pub const kStringStereoCLfeS: &[u8; 6usize] = b"C LFE\0";
pub const kStringStereoTFS: &[u8; 8usize] = b"Tfl Tfr\0";
pub const kStringStereoTSS: &[u8; 8usize] = b"Tsl Tsr\0";
pub const kStringStereoTRS: &[u8; 8usize] = b"Trl Trr\0";
pub const kStringStereoBFS: &[u8; 8usize] = b"Bfl Bfr\0";
pub const kString30CineS: &[u8; 6usize] = b"L R C\0";
pub const kString30MusicS: &[u8; 6usize] = b"L R S\0";
pub const kString31CineS: &[u8; 10usize] = b"L R C LFE\0";
pub const kString31MusicS: &[u8; 10usize] = b"L R LFE S\0";
pub const kString40CineS: &[u8; 8usize] = b"L R C S\0";
pub const kString40MusicS: &[u8; 10usize] = b"L R Ls Rs\0";
pub const kString41CineS: &[u8; 12usize] = b"L R C LFE S\0";
pub const kString41MusicS: &[u8; 14usize] = b"L R LFE Ls Rs\0";
pub const kString50S: &[u8; 12usize] = b"L R C Ls Rs\0";
pub const kString51S: &[u8; 16usize] = b"L R C LFE Ls Rs\0";
pub const kString60CineS: &[u8; 15usize] = b"L R C Ls Rs Cs\0";
pub const kString60MusicS: &[u8; 16usize] = b"L R Ls Rs Sl Sr\0";
pub const kString61CineS: &[u8; 19usize] = b"L R C LFE Ls Rs Cs\0";
pub const kString61MusicS: &[u8; 20usize] = b"L R LFE Ls Rs Sl Sr\0";
pub const kString70CineS: &[u8; 18usize] = b"L R C Ls Rs Lc Rc\0";
pub const kString70MusicS: &[u8; 18usize] = b"L R C Ls Rs Sl Sr\0";
pub const kString71CineS: &[u8; 22usize] = b"L R C LFE Ls Rs Lc Rc\0";
pub const kString71MusicS: &[u8; 22usize] = b"L R C LFE Ls Rs Sl Sr\0";
pub const kString80CineS: &[u8; 21usize] = b"L R C Ls Rs Lc Rc Cs\0";
pub const kString80MusicS: &[u8; 21usize] = b"L R C Ls Rs Cs Sl Sr\0";
pub const kString81CineS: &[u8; 25usize] = b"L R C LFE Ls Rs Lc Rc Cs\0";
pub const kString81MusicS: &[u8; 25usize] = b"L R C LFE Ls Rs Cs Sl Sr\0";
pub const kString80CubeS: &[u8; 26usize] = b"L R Ls Rs Tfl Tfr Trl Trr\0";
pub const kString71CineTopCenterS: &[u8; 22usize] = b"L R C LFE Ls Rs Cs Tc\0";
pub const kString71CineCenterHighS: &[u8; 23usize] = b"L R C LFE Ls Rs Cs Tfc\0";
pub const kString71CineFrontHighS: &[u8; 24usize] = b"L R C LFE Ls Rs Tfl Tfl\0";
pub const kString71CineSideHighS: &[u8; 24usize] = b"L R C LFE Ls Rs Tsl Tsl\0";
pub const kString71CineFullRearS: &[u8; 24usize] = b"L R C LFE Ls Rs Lcs Rcs\0";
pub const kString71ProximityS: &[u8; 22usize] = b"L R C LFE Ls Rs Pl Pr\0";
pub const kString50_4S: &[u8; 28usize] = b"L R C Ls Rs Tfl Tfr Trl Trr\0";
pub const kString51_4S: &[u8; 32usize] = b"L R C LFE Ls Rs Tfl Tfr Trl Trr\0";
pub const kString70_2S: &[u8; 26usize] = b"L R C Ls Rs Sl Sr Tsl Tsr\0";
pub const kString71_2S: &[u8; 30usize] = b"L R C LFE Ls Rs Sl Sr Tsl Tsr\0";
pub const kString70_4S: &[u8; 34usize] = b"L R C Ls Rs Sl Sr Tfl Tfr Trl Trr\0";
pub const kString71_4S: &[u8; 38usize] = b"L R C LFE Ls Rs Sl Sr Tfl Tfr Trl Trr\0";
pub const kString70_6S: &[u8; 42usize] = b"L R C Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr\0";
pub const kString71_6S: &[u8; 46usize] = b"L R C LFE Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr\0";
pub const kString100S: &[u8; 31usize] = b"L R C Ls Rs Tc Tfl Tfr Trl Trr\0";
pub const kString101S: &[u8; 35usize] = b"L R C LFE Ls Rs Tc Tfl Tfr Trl Trr\0";
pub const kString110S: &[u8; 35usize] = b"L R C Ls Rs Tc Tfl Tfc Tfr Trl Trr\0";
pub const kString111S: &[u8; 39usize] = b"L R C LFE Ls Rs Tc Tfl Tfc Tfr Trl Trr\0";
pub const kString130S: &[u8; 41usize] = b"L R C Ls Rs Sl Sr Tc Tfl Tfc Tfr Trl Trr\0";
pub const kString131S: &[u8; 45usize] = b"L R C LFE Ls Rs Sl Sr Tc Tfl Tfc Tfr Trl Trr\0";
pub const kString102S: &[u8; 41usize] = b"L R C LFE Ls Rs Tfl Tfc Tfr Trl Trr LFE2\0";
pub const kString122S: &[u8; 47usize] = b"L R C LFE Ls Rs Lc Rc Tfl Tfc Tfr Trl Trr LFE2\0";
pub const kString81MPEGS: &[u8; 30usize] = b"L R LFE Ls Rs Tfl Tfc Tfr Bfc\0";
pub const kString140S: &[u8; 48usize] = b"L R Ls Rs Sl Sr Tfl Tfr Trl Trr Bfl Bfr Brl Brr\0";
pub const kString222S: &[u8; 83usize] =
    b"L R C LFE Ls Rs Lc Rc Cs Sl Sr Tc Tfl Tfc Tfr Trl Trc Trr LFE2 Tsl Tsr Bfl Bfc Bfr\0";
pub const kStringAmbi1stOrderS: &[u8; 8usize] = b"0 1 2 3\0";
pub const kStringAmbi2cdOrderS: &[u8; 18usize] = b"0 1 2 3 4 5 6 7 8\0";
pub const kStringAmbi3rdOrderS: &[u8; 38usize] = b"0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15\0";
