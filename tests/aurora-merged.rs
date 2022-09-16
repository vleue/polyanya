use glam::Vec2;
use polyanya::Mesh;

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x.unwrap().length;
        if !((val - $y).abs() < 0.001) {
            assert_eq!(val, $y);
        }
    };
}

fn aurora_mesh() -> Mesh {
    Mesh::from_file("meshes/aurora-merged.mesh".into())
}

#[test]
fn aurora_merged() {
    let aurora = aurora_mesh();
    assert_delta!(
        aurora.path(Vec2::new(749.0, 97.0), Vec2::new(749.0, 104.0)),
        7.0
    );

    assert_delta!(
        aurora.path(Vec2::new(8.0, 646.0), Vec2::new(11.0, 642.0)),
        5.0
    );

    assert_delta!(
        aurora.path(Vec2::new(271.0, 287.0), Vec2::new(276.0, 283.0)),
        6.40312
    );

    assert_delta!(
        aurora.path(Vec2::new(814.0, 570.0), Vec2::new(810.0, 571.0)),
        4.12311
    );

    assert_delta!(
        aurora.path(Vec2::new(579.0, 169.0), Vec2::new(575.0, 163.0)),
        7.2111
    );

    assert_delta!(
        aurora.path(Vec2::new(548.0, 325.0), Vec2::new(551.0, 321.0)),
        5.0
    );

    assert_delta!(
        aurora.path(Vec2::new(89.0, 124.0), Vec2::new(95.0, 127.0)),
        6.7082
    );

    assert_delta!(
        aurora.path(Vec2::new(195.0, 114.0), Vec2::new(192.0, 120.0)),
        6.7082
    );

    assert_delta!(
        aurora.path(Vec2::new(146.0, 290.0), Vec2::new(142.0, 287.0)),
        5.0
    );

    assert_delta!(
        aurora.path(Vec2::new(114.0, 437.0), Vec2::new(117.0, 433.0)),
        5.0
    );

    assert_delta!(
        aurora.path(Vec2::new(358.0, 666.0), Vec2::new(368.0, 669.0)),
        10.4403
    );

    assert_delta!(
        aurora.path(Vec2::new(840.0, 252.0), Vec2::new(847.0, 248.0)),
        8.06226
    );

    assert_delta!(
        aurora.path(Vec2::new(428.0, 300.0), Vec2::new(418.0, 301.0)),
        10.0499
    );

    assert_delta!(
        aurora.path(Vec2::new(895.0, 646.0), Vec2::new(903.0, 641.0)),
        9.53663
    );

    assert_delta!(
        aurora.path(Vec2::new(940.0, 471.0), Vec2::new(929.0, 473.0)),
        11.1803
    );

    assert_delta!(
        aurora.path(Vec2::new(612.0, 654.0), Vec2::new(607.0, 645.0)),
        10.2956
    );

    assert_delta!(
        aurora.path(Vec2::new(610.0, 451.0), Vec2::new(617.0, 454.0)),
        7.61577
    );

    assert_delta!(
        aurora.path(Vec2::new(300.0, 358.0), Vec2::new(291.0, 351.0)),
        11.4018
    );

    assert_delta!(
        aurora.path(Vec2::new(10.0, 655.0), Vec2::new(18.0, 663.0)),
        11.3137
    );

    assert_delta!(
        aurora.path(Vec2::new(490.0, 682.0), Vec2::new(500.0, 680.0)),
        10.198
    );

    assert_delta!(
        aurora.path(Vec2::new(70.0, 554.0), Vec2::new(73.0, 540.0)),
        14.3178
    );

    assert_delta!(
        aurora.path(Vec2::new(717.0, 172.0), Vec2::new(729.0, 170.0)),
        12.1655
    );

    assert_delta!(
        aurora.path(Vec2::new(521.0, 706.0), Vec2::new(508.0, 702.0)),
        13.6015
    );

    assert_delta!(
        aurora.path(Vec2::new(717.0, 683.0), Vec2::new(731.0, 681.0)),
        14.1421
    );

    assert_delta!(
        aurora.path(Vec2::new(392.0, 601.0), Vec2::new(405.0, 606.0)),
        13.9284
    );

    assert_delta!(
        aurora.path(Vec2::new(347.0, 272.0), Vec2::new(348.0, 285.0)),
        13.0384
    );

    assert_delta!(
        aurora.path(Vec2::new(229.0, 639.0), Vec2::new(240.0, 631.0)),
        13.6015
    );

    assert_delta!(
        aurora.path(Vec2::new(749.0, 423.0), Vec2::new(739.0, 412.0)),
        14.8661
    );

    assert_delta!(
        aurora.path(Vec2::new(485.0, 297.0), Vec2::new(472.0, 300.0)),
        13.3417
    );

    assert_delta!(
        aurora.path(Vec2::new(628.0, 302.0), Vec2::new(638.0, 309.0)),
        12.2066
    );

    assert_delta!(
        aurora.path(Vec2::new(386.0, 468.0), Vec2::new(374.0, 483.0)),
        19.2094
    );

    assert_delta!(
        aurora.path(Vec2::new(483.0, 499.0), Vec2::new(468.0, 487.0)),
        19.2094
    );

    assert_delta!(
        aurora.path(Vec2::new(869.0, 424.0), Vec2::new(854.0, 416.0)),
        17.0
    );

    assert_delta!(
        aurora.path(Vec2::new(514.0, 346.0), Vec2::new(499.0, 337.0)),
        17.4929
    );

    assert_delta!(
        aurora.path(Vec2::new(208.0, 251.0), Vec2::new(218.0, 238.0)),
        16.4012
    );

    assert_delta!(
        aurora.path(Vec2::new(676.0, 100.0), Vec2::new(669.0, 115.0)),
        16.5529
    );

    assert_delta!(
        aurora.path(Vec2::new(881.0, 382.0), Vec2::new(897.0, 375.0)),
        17.4642
    );

    assert_delta!(
        aurora.path(Vec2::new(927.0, 418.0), Vec2::new(932.0, 404.0)),
        14.8661
    );

    assert_delta!(
        aurora.path(Vec2::new(323.0, 99.0), Vec2::new(309.0, 90.0)),
        16.6433
    );

    assert_delta!(
        aurora.path(Vec2::new(354.0, 118.0), Vec2::new(337.0, 112.0)),
        18.0278
    );

    assert_delta!(
        aurora.path(Vec2::new(719.0, 111.0), Vec2::new(700.0, 114.0)),
        19.2354
    );

    assert_delta!(
        aurora.path(Vec2::new(863.0, 142.0), Vec2::new(884.0, 147.0)),
        21.587
    );

    assert_delta!(
        aurora.path(Vec2::new(442.0, 456.0), Vec2::new(447.0, 477.0)),
        21.587
    );

    assert_delta!(
        aurora.path(Vec2::new(903.0, 419.0), Vec2::new(911.0, 436.0)),
        18.7883
    );

    assert_delta!(
        aurora.path(Vec2::new(813.0, 242.0), Vec2::new(790.0, 244.0)),
        23.0868
    );

    assert_delta!(
        aurora.path(Vec2::new(604.0, 197.0), Vec2::new(619.0, 214.0)),
        22.6716
    );

    assert_delta!(
        aurora.path(Vec2::new(274.0, 273.0), Vec2::new(258.0, 257.0)),
        22.6274
    );

    assert_delta!(
        aurora.path(Vec2::new(313.0, 547.0), Vec2::new(317.0, 526.0)),
        21.3776
    );

    assert_delta!(
        aurora.path(Vec2::new(134.0, 674.0), Vec2::new(116.0, 665.0)),
        20.1246
    );

    assert_delta!(
        aurora.path(Vec2::new(464.0, 326.0), Vec2::new(474.0, 343.0)),
        19.7231
    );

    assert_delta!(
        aurora.path(Vec2::new(58.0, 404.0), Vec2::new(45.0, 385.0)),
        23.0217
    );

    assert_delta!(
        aurora.path(Vec2::new(55.0, 166.0), Vec2::new(76.0, 182.0)),
        26.4008
    );

    assert_delta!(
        aurora.path(Vec2::new(14.0, 387.0), Vec2::new(40.0, 390.0)),
        26.1725
    );

    assert_delta!(
        aurora.path(Vec2::new(493.0, 444.0), Vec2::new(470.0, 436.0)),
        24.3516
    );

    assert_delta!(
        aurora.path(Vec2::new(653.0, 394.0), Vec2::new(674.0, 407.0)),
        24.6982
    );

    assert_delta!(
        aurora.path(Vec2::new(836.0, 122.0), Vec2::new(857.0, 111.0)),
        23.7065
    );

    assert_delta!(
        aurora.path(Vec2::new(27.0, 591.0), Vec2::new(42.0, 573.0)),
        23.455
    );

    assert_delta!(
        aurora.path(Vec2::new(761.0, 368.0), Vec2::new(740.0, 358.0)),
        23.2594
    );

    assert_delta!(
        aurora.path(Vec2::new(693.0, 312.0), Vec2::new(682.0, 298.0)),
        22.1812
    );

    assert_delta!(
        aurora.path(Vec2::new(126.0, 183.0), Vec2::new(103.0, 189.0)),
        23.7697
    );

    assert_delta!(
        aurora.path(Vec2::new(545.0, 503.0), Vec2::new(517.0, 507.0)),
        28.2843
    );

    assert_delta!(
        aurora.path(Vec2::new(812.0, 550.0), Vec2::new(809.0, 577.0)),
        27.1662
    );

    assert_delta!(
        aurora.path(Vec2::new(885.0, 286.0), Vec2::new(860.0, 299.0)),
        28.178
    );

    assert_delta!(
        aurora.path(Vec2::new(818.0, 292.0), Vec2::new(795.0, 306.0)),
        26.9258
    );

    assert_delta!(
        aurora.path(Vec2::new(467.0, 205.0), Vec2::new(446.0, 185.0)),
        29.0
    );

    assert_delta!(
        aurora.path(Vec2::new(114.0, 464.0), Vec2::new(143.0, 469.0)),
        29.4279
    );

    assert_delta!(
        aurora.path(Vec2::new(717.0, 368.0), Vec2::new(741.0, 381.0)),
        27.2947
    );

    assert_delta!(
        aurora.path(Vec2::new(432.0, 259.0), Vec2::new(462.0, 258.0)),
        30.0167
    );

    assert_delta!(
        aurora.path(Vec2::new(134.0, 434.0), Vec2::new(136.0, 405.0)),
        29.0689
    );

    assert_delta!(
        aurora.path(Vec2::new(305.0, 683.0), Vec2::new(275.0, 681.0)),
        30.0666
    );

    assert_delta!(
        aurora.path(Vec2::new(336.0, 321.0), Vec2::new(336.0, 353.0)),
        32.0
    );

    assert_delta!(
        aurora.path(Vec2::new(646.0, 410.0), Vec2::new(642.0, 444.0)),
        34.2345
    );

    assert_delta!(
        aurora.path(Vec2::new(255.0, 382.0), Vec2::new(281.0, 362.0)),
        32.8024
    );

    assert_delta!(
        aurora.path(Vec2::new(326.0, 509.0), Vec2::new(298.0, 520.0)),
        30.313
    );

    assert_delta!(
        aurora.path(Vec2::new(980.0, 447.0), Vec2::new(965.0, 418.0)),
        32.6497
    );

    assert_delta!(
        aurora.path(Vec2::new(645.0, 591.0), Vec2::new(619.0, 573.0)),
        31.6969
    );

    assert_delta!(
        aurora.path(Vec2::new(123.0, 465.0), Vec2::new(142.0, 492.0)),
        33.0151
    );

    assert_delta!(
        aurora.path(Vec2::new(387.0, 227.0), Vec2::new(358.0, 239.0)),
        31.3847
    );

    assert_delta!(
        aurora.path(Vec2::new(383.0, 340.0), Vec2::new(390.0, 370.0)),
        32.8629
    );

    assert_delta!(
        aurora.path(Vec2::new(100.0, 290.0), Vec2::new(129.0, 303.0)),
        33.7275
    );

    assert_delta!(
        aurora.path(Vec2::new(555.0, 646.0), Vec2::new(528.0, 624.0)),
        34.8281
    );

    assert_delta!(
        aurora.path(Vec2::new(457.0, 673.0), Vec2::new(493.0, 670.0)),
        36.1248
    );

    assert_delta!(
        aurora.path(Vec2::new(13.0, 622.0), Vec2::new(28.0, 604.0)),
        37.4046
    );

    assert_delta!(
        aurora.path(Vec2::new(278.0, 92.0), Vec2::new(314.0, 98.0)),
        36.4966
    );

    assert_delta!(
        aurora.path(Vec2::new(527.0, 541.0), Vec2::new(520.0, 506.0)),
        35.6931
    );

    assert_delta!(
        aurora.path(Vec2::new(774.0, 216.0), Vec2::new(805.0, 216.0)),
        35.4222
    );

    assert_delta!(
        aurora.path(Vec2::new(229.0, 698.0), Vec2::new(199.0, 714.0)),
        34.0
    );

    assert_delta!(
        aurora.path(Vec2::new(592.0, 177.0), Vec2::new(607.0, 146.0)),
        34.4384
    );

    assert_delta!(
        aurora.path(Vec2::new(889.0, 512.0), Vec2::new(874.0, 482.0)),
        33.541
    );

    assert_delta!(
        aurora.path(Vec2::new(251.0, 98.0), Vec2::new(222.0, 115.0)),
        33.6155
    );

    assert_delta!(
        aurora.path(Vec2::new(903.0, 380.0), Vec2::new(935.0, 352.0)),
        42.5206
    );

    assert_delta!(
        aurora.path(Vec2::new(537.0, 193.0), Vec2::new(567.0, 219.0)),
        39.6989
    );

    assert_delta!(
        aurora.path(Vec2::new(230.0, 281.0), Vec2::new(263.0, 261.0)),
        39.0
    );

    assert_delta!(
        aurora.path(Vec2::new(636.0, 85.0), Vec2::new(603.0, 106.0)),
        39.5876
    );

    assert_delta!(
        aurora.path(Vec2::new(620.0, 235.0), Vec2::new(641.0, 268.0)),
        39.2753
    );

    assert_delta!(
        aurora.path(Vec2::new(167.0, 235.0), Vec2::new(201.0, 255.0)),
        39.4462
    );

    assert_delta!(
        aurora.path(Vec2::new(315.0, 338.0), Vec2::new(337.0, 307.0)),
        38.0132
    );

    assert_delta!(
        aurora.path(Vec2::new(72.0, 402.0), Vec2::new(101.0, 431.0)),
        41.0122
    );

    assert_delta!(
        aurora.path(Vec2::new(632.0, 742.0), Vec2::new(623.0, 704.0)),
        39.108
    );

    assert_delta!(
        aurora.path(Vec2::new(709.0, 584.0), Vec2::new(741.0, 599.0)),
        39.4709
    );

    assert_delta!(
        aurora.path(Vec2::new(908.0, 292.0), Vec2::new(911.0, 257.0)),
        42.4083
    );

    assert_delta!(
        aurora.path(Vec2::new(104.0, 564.0), Vec2::new(145.0, 566.0)),
        42.1045
    );

    assert_delta!(
        aurora.path(Vec2::new(272.0, 92.0), Vec2::new(233.0, 106.0)),
        41.4367
    );

    assert_delta!(
        aurora.path(Vec2::new(322.0, 296.0), Vec2::new(283.0, 277.0)),
        43.382
    );

    assert_delta!(
        aurora.path(Vec2::new(442.0, 460.0), Vec2::new(398.0, 467.0)),
        44.5533
    );

    assert_delta!(
        aurora.path(Vec2::new(774.0, 350.0), Vec2::new(731.0, 362.0)),
        44.643
    );

    assert_delta!(
        aurora.path(Vec2::new(118.0, 721.0), Vec2::new(95.0, 686.0)),
        41.8808
    );

    assert_delta!(
        aurora.path(Vec2::new(569.0, 163.0), Vec2::new(607.0, 183.0)),
        42.9418
    );

    assert_delta!(
        aurora.path(Vec2::new(87.0, 632.0), Vec2::new(122.0, 610.0)),
        41.3401
    );

    assert_delta!(
        aurora.path(Vec2::new(214.0, 529.0), Vec2::new(176.0, 551.0)),
        43.909
    );

    assert_delta!(
        aurora.path(Vec2::new(75.0, 148.0), Vec2::new(113.0, 141.0)),
        46.5038
    );

    assert_delta!(
        aurora.path(Vec2::new(572.0, 486.0), Vec2::new(533.0, 464.0)),
        44.7772
    );

    assert_delta!(
        aurora.path(Vec2::new(456.0, 589.0), Vec2::new(492.0, 618.0)),
        46.2277
    );

    assert_delta!(
        aurora.path(Vec2::new(488.0, 493.0), Vec2::new(454.0, 456.0)),
        50.2769
    );

    assert_delta!(
        aurora.path(Vec2::new(835.0, 549.0), Vec2::new(788.0, 555.0)),
        47.3814
    );

    assert_delta!(
        aurora.path(Vec2::new(191.0, 701.0), Vec2::new(237.0, 714.0)),
        47.8017
    );

    assert_delta!(
        aurora.path(Vec2::new(909.0, 261.0), Vec2::new(897.0, 279.0)),
        47.9312
    );

    assert_delta!(
        aurora.path(Vec2::new(614.0, 608.0), Vec2::new(650.0, 572.0)),
        50.9117
    );

    assert_delta!(
        aurora.path(Vec2::new(487.0, 539.0), Vec2::new(452.0, 574.0)),
        49.4975
    );

    assert_delta!(
        aurora.path(Vec2::new(806.0, 370.0), Vec2::new(765.0, 348.0)),
        46.5296
    );

    assert_delta!(
        aurora.path(Vec2::new(905.0, 592.0), Vec2::new(859.0, 570.0)),
        50.9902
    );

    assert_delta!(
        aurora.path(Vec2::new(713.0, 657.0), Vec2::new(711.0, 604.0)),
        53.0377
    );

    assert_delta!(
        aurora.path(Vec2::new(468.0, 288.0), Vec2::new(428.0, 324.0)),
        53.8145
    );

    assert_delta!(
        aurora.path(Vec2::new(567.0, 158.0), Vec2::new(544.0, 140.0)),
        51.2681
    );

    assert_delta!(
        aurora.path(Vec2::new(488.0, 138.0), Vec2::new(467.0, 156.0)),
        47.5703
    );

    assert_delta!(
        aurora.path(Vec2::new(631.0, 402.0), Vec2::new(620.0, 353.0)),
        50.3406
    );

    assert_delta!(
        aurora.path(Vec2::new(681.0, 216.0), Vec2::new(707.0, 178.0)),
        52.9495
    );

    assert_delta!(
        aurora.path(Vec2::new(801.0, 543.0), Vec2::new(851.0, 551.0)),
        50.636
    );

    assert_delta!(
        aurora.path(Vec2::new(549.0, 600.0), Vec2::new(503.0, 621.0)),
        50.5668
    );

    assert_delta!(
        aurora.path(Vec2::new(976.0, 624.0), Vec2::new(923.0, 629.0)),
        53.2353
    );

    assert_delta!(
        aurora.path(Vec2::new(965.0, 415.0), Vec2::new(1023.0, 412.0)),
        58.0775
    );

    assert_delta!(
        aurora.path(Vec2::new(902.0, 343.0), Vec2::new(923.0, 295.0)),
        52.3927
    );

    assert_delta!(
        aurora.path(Vec2::new(228.0, 212.0), Vec2::new(182.0, 239.0)),
        53.3385
    );

    assert_delta!(
        aurora.path(Vec2::new(795.0, 680.0), Vec2::new(747.0, 700.0)),
        52.0
    );

    assert_delta!(
        aurora.path(Vec2::new(710.0, 398.0), Vec2::new(686.0, 434.0)),
        56.8737
    );

    assert_delta!(
        aurora.path(Vec2::new(318.0, 166.0), Vec2::new(294.0, 209.0)),
        55.5646
    );

    assert_delta!(
        aurora.path(Vec2::new(166.0, 303.0), Vec2::new(209.0, 269.0)),
        54.8759
    );

    assert_delta!(
        aurora.path(Vec2::new(117.0, 105.0), Vec2::new(70.0, 133.0)),
        54.7083
    );

    assert_delta!(
        aurora.path(Vec2::new(544.0, 302.0), Vec2::new(593.0, 319.0)),
        51.8652
    );

    assert_delta!(
        aurora.path(Vec2::new(658.0, 344.0), Vec2::new(704.0, 319.0)),
        52.3546
    );

    assert_delta!(
        aurora.path(Vec2::new(426.0, 140.0), Vec2::new(406.0, 109.0)),
        56.8809
    );

    assert_delta!(
        aurora.path(Vec2::new(245.0, 594.0), Vec2::new(236.0, 631.0)),
        58.3086
    );

    assert_delta!(
        aurora.path(Vec2::new(210.0, 530.0), Vec2::new(155.0, 525.0)),
        56.8212
    );

    assert_delta!(
        aurora.path(Vec2::new(255.0, 572.0), Vec2::new(210.0, 531.0)),
        60.8769
    );

    assert_delta!(
        aurora.path(Vec2::new(21.0, 585.0), Vec2::new(61.0, 618.0)),
        57.9557
    );

    assert_delta!(
        aurora.path(Vec2::new(114.0, 237.0), Vec2::new(117.0, 281.0)),
        56.7178
    );

    assert_delta!(
        aurora.path(Vec2::new(1021.0, 340.0), Vec2::new(967.0, 346.0)),
        57.1555
    );

    assert_delta!(
        aurora.path(Vec2::new(644.0, 340.0), Vec2::new(598.0, 375.0)),
        57.8014
    );

    assert_delta!(
        aurora.path(Vec2::new(123.0, 157.0), Vec2::new(68.0, 178.0)),
        58.8727
    );

    assert_delta!(
        aurora.path(Vec2::new(536.0, 261.0), Vec2::new(562.0, 307.0)),
        56.3745
    );

    assert_delta!(
        aurora.path(Vec2::new(672.0, 72.0), Vec2::new(724.0, 79.0)),
        60.6906
    );

    assert_delta!(
        aurora.path(Vec2::new(508.0, 318.0), Vec2::new(494.0, 295.0)),
        59.7026
    );

    assert_delta!(
        aurora.path(Vec2::new(676.0, 115.0), Vec2::new(644.0, 94.0)),
        59.7014
    );

    assert_delta!(
        aurora.path(Vec2::new(127.0, 281.0), Vec2::new(102.0, 337.0)),
        63.9275
    );

    assert_delta!(
        aurora.path(Vec2::new(629.0, 627.0), Vec2::new(569.0, 644.0)),
        65.0856
    );

    assert_delta!(
        aurora.path(Vec2::new(472.0, 544.0), Vec2::new(531.0, 530.0)),
        60.7235
    );

    assert_delta!(
        aurora.path(Vec2::new(223.0, 646.0), Vec2::new(249.0, 674.0)),
        62.6869
    );

    assert_delta!(
        aurora.path(Vec2::new(447.0, 458.0), Vec2::new(425.0, 415.0)),
        61.8827
    );

    assert_delta!(
        aurora.path(Vec2::new(147.0, 390.0), Vec2::new(86.0, 376.0)),
        62.5859
    );

    assert_delta!(
        aurora.path(Vec2::new(382.0, 542.0), Vec2::new(432.0, 506.0)),
        61.6117
    );

    assert_delta!(
        aurora.path(Vec2::new(470.0, 547.0), Vec2::new(541.0, 548.0)),
        71.007
    );

    assert_delta!(
        aurora.path(Vec2::new(771.0, 393.0), Vec2::new(832.0, 372.0)),
        66.3305
    );

    assert_delta!(
        aurora.path(Vec2::new(226.0, 502.0), Vec2::new(194.0, 452.0)),
        65.6878
    );

    assert_delta!(
        aurora.path(Vec2::new(668.0, 535.0), Vec2::new(695.0, 595.0)),
        69.0304
    );

    assert_delta!(
        aurora.path(Vec2::new(577.0, 669.0), Vec2::new(523.0, 635.0)),
        63.8122
    );

    assert_delta!(
        aurora.path(Vec2::new(454.0, 341.0), Vec2::new(494.0, 396.0)),
        69.223
    );

    assert_delta!(
        aurora.path(Vec2::new(6.0, 410.0), Vec2::new(56.0, 369.0)),
        67.448
    );

    assert_delta!(
        aurora.path(Vec2::new(499.0, 552.0), Vec2::new(525.0, 491.0)),
        66.3099
    );

    assert_delta!(
        aurora.path(Vec2::new(334.0, 566.0), Vec2::new(390.0, 603.0)),
        67.1193
    );

    assert_delta!(
        aurora.path(Vec2::new(720.0, 180.0), Vec2::new(731.0, 226.0)),
        63.0995
    );

    assert_delta!(
        aurora.path(Vec2::new(201.0, 399.0), Vec2::new(265.0, 375.0)),
        68.352
    );

    assert_delta!(
        aurora.path(Vec2::new(809.0, 595.0), Vec2::new(741.0, 611.0)),
        69.857
    );

    assert_delta!(
        aurora.path(Vec2::new(697.0, 576.0), Vec2::new(762.0, 559.0)),
        67.1863
    );

    assert_delta!(
        aurora.path(Vec2::new(633.0, 584.0), Vec2::new(691.0, 544.0)),
        70.4557
    );

    assert_delta!(
        aurora.path(Vec2::new(58.0, 653.0), Vec2::new(123.0, 630.0)),
        71.0863
    );

    assert_delta!(
        aurora.path(Vec2::new(659.0, 165.0), Vec2::new(731.0, 161.0)),
        72.111
    );

    assert_delta!(
        aurora.path(Vec2::new(769.0, 647.0), Vec2::new(758.0, 666.0)),
        70.1669
    );

    assert_delta!(
        aurora.path(Vec2::new(615.0, 651.0), Vec2::new(543.0, 643.0)),
        72.4431
    );

    assert_delta!(
        aurora.path(Vec2::new(215.0, 111.0), Vec2::new(191.0, 74.0)),
        71.0889
    );

    assert_delta!(
        aurora.path(Vec2::new(442.0, 123.0), Vec2::new(382.0, 131.0)),
        69.5767
    );

    assert_delta!(
        aurora.path(Vec2::new(129.0, 209.0), Vec2::new(177.0, 245.0)),
        71.5466
    );

    assert_delta!(
        aurora.path(Vec2::new(174.0, 372.0), Vec2::new(184.0, 299.0)),
        73.6817
    );

    assert_delta!(
        aurora.path(Vec2::new(518.0, 354.0), Vec2::new(513.0, 281.0)),
        75.4362
    );

    assert_delta!(
        aurora.path(Vec2::new(664.0, 269.0), Vec2::new(729.0, 305.0)),
        74.3034
    );

    assert_delta!(
        aurora.path(Vec2::new(965.0, 154.0), Vec2::new(905.0, 192.0)),
        71.4347
    );

    assert_delta!(
        aurora.path(Vec2::new(307.0, 617.0), Vec2::new(283.0, 687.0)),
        74.0999
    );

    assert_delta!(
        aurora.path(Vec2::new(270.0, 200.0), Vec2::new(324.0, 250.0)),
        75.4775
    );

    assert_delta!(
        aurora.path(Vec2::new(652.0, 519.0), Vec2::new(629.0, 580.0)),
        74.2269
    );

    assert_delta!(
        aurora.path(Vec2::new(770.0, 119.0), Vec2::new(711.0, 76.0)),
        73.1276
    );

    assert_delta!(
        aurora.path(Vec2::new(201.0, 286.0), Vec2::new(163.0, 333.0)),
        70.0244
    );

    assert_delta!(
        aurora.path(Vec2::new(664.0, 633.0), Vec2::new(591.0, 607.0)),
        77.4919
    );

    assert_delta!(
        aurora.path(Vec2::new(706.0, 537.0), Vec2::new(706.0, 609.0)),
        77.2939
    );

    assert_delta!(
        aurora.path(Vec2::new(494.0, 134.0), Vec2::new(528.0, 179.0)),
        79.5731
    );

    assert_delta!(
        aurora.path(Vec2::new(453.0, 520.0), Vec2::new(436.0, 534.0)),
        73.7515
    );

    assert_delta!(
        aurora.path(Vec2::new(182.0, 354.0), Vec2::new(216.0, 322.0)),
        74.9783
    );

    assert_delta!(
        aurora.path(Vec2::new(543.0, 643.0), Vec2::new(498.0, 593.0)),
        76.8788
    );

    assert_delta!(
        aurora.path(Vec2::new(278.0, 489.0), Vec2::new(336.0, 514.0)),
        76.9914
    );

    assert_delta!(
        aurora.path(Vec2::new(596.0, 151.0), Vec2::new(526.0, 164.0)),
        74.2301
    );

    assert_delta!(
        aurora.path(Vec2::new(719.0, 606.0), Vec2::new(766.0, 543.0)),
        81.5821
    );

    assert_delta!(
        aurora.path(Vec2::new(367.0, 719.0), Vec2::new(299.0, 752.0)),
        75.5844
    );

    assert_delta!(
        aurora.path(Vec2::new(706.0, 524.0), Vec2::new(778.0, 491.0)),
        79.2023
    );

    assert_delta!(
        aurora.path(Vec2::new(913.0, 482.0), Vec2::new(842.0, 453.0)),
        84.5886
    );

    assert_delta!(
        aurora.path(Vec2::new(451.0, 146.0), Vec2::new(436.0, 80.0)),
        81.8474
    );

    assert_delta!(
        aurora.path(Vec2::new(252.0, 560.0), Vec2::new(322.0, 602.0)),
        81.6333
    );

    assert_delta!(
        aurora.path(Vec2::new(930.0, 540.0), Vec2::new(911.0, 492.0)),
        80.5801
    );

    assert_delta!(
        aurora.path(Vec2::new(545.0, 172.0), Vec2::new(620.0, 201.0)),
        80.4114
    );

    assert_delta!(
        aurora.path(Vec2::new(839.0, 525.0), Vec2::new(894.0, 569.0)),
        81.5683
    );

    assert_delta!(
        aurora.path(Vec2::new(528.0, 461.0), Vec2::new(491.0, 514.0)),
        82.9281
    );

    assert_delta!(
        aurora.path(Vec2::new(678.0, 368.0), Vec2::new(648.0, 425.0)),
        79.0596
    );

    assert_delta!(
        aurora.path(Vec2::new(527.0, 325.0), Vec2::new(451.0, 350.0)),
        80.0062
    );

    assert_delta!(
        aurora.path(Vec2::new(878.0, 245.0), Vec2::new(932.0, 302.0)),
        86.1604
    );

    assert_delta!(
        aurora.path(Vec2::new(41.0, 509.0), Vec2::new(118.0, 525.0)),
        82.2626
    );

    assert_delta!(
        aurora.path(Vec2::new(798.0, 442.0), Vec2::new(869.0, 484.0)),
        83.0759
    );

    assert_delta!(
        aurora.path(Vec2::new(820.0, 400.0), Vec2::new(795.0, 334.0)),
        83.4117
    );

    assert_delta!(
        aurora.path(Vec2::new(94.0, 406.0), Vec2::new(143.0, 476.0)),
        85.4459
    );

    assert_delta!(
        aurora.path(Vec2::new(102.0, 375.0), Vec2::new(24.0, 405.0)),
        84.9267
    );

    assert_delta!(
        aurora.path(Vec2::new(169.0, 179.0), Vec2::new(138.0, 195.0)),
        86.0704
    );

    assert_delta!(
        aurora.path(Vec2::new(624.0, 290.0), Vec2::new(597.0, 233.0)),
        84.8778
    );

    assert_delta!(
        aurora.path(Vec2::new(744.0, 158.0), Vec2::new(712.0, 80.0)),
        87.0808
    );

    assert_delta!(
        aurora.path(Vec2::new(526.0, 394.0), Vec2::new(521.0, 332.0)),
        80.4314
    );

    assert_delta!(
        aurora.path(Vec2::new(764.0, 636.0), Vec2::new(845.0, 667.0)),
        87.3999
    );

    assert_delta!(
        aurora.path(Vec2::new(524.0, 587.0), Vec2::new(445.0, 625.0)),
        88.3247
    );

    assert_delta!(
        aurora.path(Vec2::new(592.0, 241.0), Vec2::new(518.0, 189.0)),
        90.4434
    );

    assert_delta!(
        aurora.path(Vec2::new(527.0, 565.0), Vec2::new(606.0, 533.0)),
        88.2341
    );

    assert_delta!(
        aurora.path(Vec2::new(514.0, 320.0), Vec2::new(555.0, 244.0)),
        86.3539
    );

    assert_delta!(
        aurora.path(Vec2::new(320.0, 660.0), Vec2::new(256.0, 603.0)),
        89.4251
    );

    assert_delta!(
        aurora.path(Vec2::new(266.0, 645.0), Vec2::new(267.0, 561.0)),
        86.1849
    );

    assert_delta!(
        aurora.path(Vec2::new(708.0, 567.0), Vec2::new(796.0, 557.0)),
        88.5664
    );

    assert_delta!(
        aurora.path(Vec2::new(487.0, 167.0), Vec2::new(570.0, 136.0)),
        88.6002
    );

    assert_delta!(
        aurora.path(Vec2::new(674.0, 378.0), Vec2::new(746.0, 372.0)),
        84.3871
    );

    assert_delta!(
        aurora.path(Vec2::new(609.0, 609.0), Vec2::new(657.0, 532.0)),
        95.4461
    );

    assert_delta!(
        aurora.path(Vec2::new(13.0, 326.0), Vec2::new(28.0, 413.0)),
        93.7099
    );

    assert_delta!(
        aurora.path(Vec2::new(591.0, 609.0), Vec2::new(530.0, 678.0)),
        96.2778
    );

    assert_delta!(
        aurora.path(Vec2::new(75.0, 144.0), Vec2::new(167.0, 155.0)),
        93.1209
    );

    assert_delta!(
        aurora.path(Vec2::new(244.0, 617.0), Vec2::new(218.0, 530.0)),
        92.9498
    );

    assert_delta!(
        aurora.path(Vec2::new(820.0, 495.0), Vec2::new(823.0, 587.0)),
        94.1679
    );

    assert_delta!(
        aurora.path(Vec2::new(321.0, 624.0), Vec2::new(228.0, 608.0)),
        94.6241
    );

    assert_delta!(
        aurora.path(Vec2::new(444.0, 121.0), Vec2::new(392.0, 163.0)),
        93.1583
    );

    assert_delta!(
        aurora.path(Vec2::new(524.0, 640.0), Vec2::new(469.0, 566.0)),
        93.3364
    );

    assert_delta!(
        aurora.path(Vec2::new(684.0, 309.0), Vec2::new(701.0, 333.0)),
        91.1695
    );

    assert_delta!(
        aurora.path(Vec2::new(909.0, 548.0), Vec2::new(969.0, 614.0)),
        96.8366
    );

    assert_delta!(
        aurora.path(Vec2::new(473.0, 245.0), Vec2::new(562.0, 253.0)),
        95.3451
    );

    assert_delta!(
        aurora.path(Vec2::new(543.0, 309.0), Vec2::new(498.0, 231.0)),
        97.0411
    );

    assert_delta!(
        aurora.path(Vec2::new(889.0, 615.0), Vec2::new(808.0, 567.0)),
        94.2606
    );

    assert_delta!(
        aurora.path(Vec2::new(722.0, 228.0), Vec2::new(729.0, 144.0)),
        92.2824
    );

    assert_delta!(
        aurora.path(Vec2::new(165.0, 263.0), Vec2::new(249.0, 307.0)),
        94.8262
    );

    assert_delta!(
        aurora.path(Vec2::new(862.0, 631.0), Vec2::new(952.0, 605.0)),
        93.6803
    );

    assert_delta!(
        aurora.path(Vec2::new(356.0, 124.0), Vec2::new(452.0, 112.0)),
        97.6132
    );

    assert_delta!(
        aurora.path(Vec2::new(169.0, 531.0), Vec2::new(258.0, 509.0)),
        93.672
    );

    assert_delta!(
        aurora.path(Vec2::new(357.0, 722.0), Vec2::new(449.0, 695.0)),
        95.8801
    );

    assert_delta!(
        aurora.path(Vec2::new(651.0, 529.0), Vec2::new(678.0, 624.0)),
        101.655
    );

    assert_delta!(
        aurora.path(Vec2::new(170.0, 648.0), Vec2::new(253.0, 659.0)),
        100.206
    );

    assert_delta!(
        aurora.path(Vec2::new(122.0, 383.0), Vec2::new(211.0, 346.0)),
        96.3846
    );

    assert_delta!(
        aurora.path(Vec2::new(355.0, 488.0), Vec2::new(436.0, 515.0)),
        101.788
    );

    assert_delta!(
        aurora.path(Vec2::new(828.0, 95.0), Vec2::new(780.0, 130.0)),
        100.879
    );

    assert_delta!(
        aurora.path(Vec2::new(67.0, 376.0), Vec2::new(138.0, 434.0)),
        97.9025
    );

    assert_delta!(
        aurora.path(Vec2::new(625.0, 130.0), Vec2::new(716.0, 90.0)),
        99.917
    );

    assert_delta!(
        aurora.path(Vec2::new(482.0, 50.0), Vec2::new(433.0, 122.0)),
        99.7744
    );

    assert_delta!(
        aurora.path(Vec2::new(150.0, 589.0), Vec2::new(224.0, 516.0)),
        103.948
    );

    assert_delta!(
        aurora.path(Vec2::new(526.0, 313.0), Vec2::new(437.0, 351.0)),
        96.7729
    );

    assert_delta!(
        aurora.path(Vec2::new(250.0, 656.0), Vec2::new(203.0, 576.0)),
        107.144
    );

    assert_delta!(
        aurora.path(Vec2::new(824.0, 607.0), Vec2::new(836.0, 512.0)),
        103.089
    );

    assert_delta!(
        aurora.path(Vec2::new(859.0, 219.0), Vec2::new(943.0, 278.0)),
        102.65
    );

    assert_delta!(
        aurora.path(Vec2::new(429.0, 487.0), Vec2::new(527.0, 456.0)),
        102.793
    );

    assert_delta!(
        aurora.path(Vec2::new(827.0, 661.0), Vec2::new(765.0, 726.0)),
        106.58
    );

    assert_delta!(
        aurora.path(Vec2::new(979.0, 312.0), Vec2::new(949.0, 266.0)),
        100.064
    );

    assert_delta!(
        aurora.path(Vec2::new(702.0, 392.0), Vec2::new(729.0, 388.0)),
        100.328
    );

    assert_delta!(
        aurora.path(Vec2::new(1023.0, 360.0), Vec2::new(962.0, 280.0)),
        106.438
    );

    assert_delta!(
        aurora.path(Vec2::new(516.0, 203.0), Vec2::new(449.0, 261.0)),
        103.808
    );

    assert_delta!(
        aurora.path(Vec2::new(546.0, 601.0), Vec2::new(466.0, 575.0)),
        102.607
    );

    assert_delta!(
        aurora.path(Vec2::new(715.0, 702.0), Vec2::new(632.0, 654.0)),
        109.622
    );

    assert_delta!(
        aurora.path(Vec2::new(740.0, 244.0), Vec2::new(737.0, 158.0)),
        105.764
    );

    assert_delta!(
        aurora.path(Vec2::new(78.0, 194.0), Vec2::new(173.0, 150.0)),
        104.695
    );

    assert_delta!(
        aurora.path(Vec2::new(134.0, 666.0), Vec2::new(187.0, 574.0)),
        106.195
    );

    assert_delta!(
        aurora.path(Vec2::new(256.0, 719.0), Vec2::new(365.0, 706.0)),
        110.25
    );

    assert_delta!(
        aurora.path(Vec2::new(852.0, 307.0), Vec2::new(835.0, 239.0)),
        105.822
    );

    assert_delta!(
        aurora.path(Vec2::new(177.0, 420.0), Vec2::new(180.0, 449.0)),
        107.705
    );

    assert_delta!(
        aurora.path(Vec2::new(458.0, 269.0), Vec2::new(518.0, 324.0)),
        109.626
    );

    assert_delta!(
        aurora.path(Vec2::new(185.0, 186.0), Vec2::new(239.0, 96.0)),
        105.766
    );

    assert_delta!(
        aurora.path(Vec2::new(115.0, 583.0), Vec2::new(229.0, 584.0)),
        114.005
    );

    assert_delta!(
        aurora.path(Vec2::new(166.0, 631.0), Vec2::new(186.0, 718.0)),
        108.15
    );

    assert_delta!(
        aurora.path(Vec2::new(607.0, 367.0), Vec2::new(501.0, 384.0)),
        109.317
    );

    assert_delta!(
        aurora.path(Vec2::new(876.0, 314.0), Vec2::new(775.0, 307.0)),
        109.238
    );

    assert_delta!(
        aurora.path(Vec2::new(692.0, 269.0), Vec2::new(622.0, 335.0)),
        107.923
    );

    assert_delta!(
        aurora.path(Vec2::new(828.0, 515.0), Vec2::new(766.0, 609.0)),
        112.606
    );

    assert_delta!(
        aurora.path(Vec2::new(773.0, 354.0), Vec2::new(861.0, 289.0)),
        112.932
    );

    assert_delta!(
        aurora.path(Vec2::new(99.0, 387.0), Vec2::new(62.0, 436.0)),
        110.422
    );

    assert_delta!(
        aurora.path(Vec2::new(176.0, 639.0), Vec2::new(294.0, 639.0)),
        118.0
    );

    assert_delta!(
        aurora.path(Vec2::new(458.0, 405.0), Vec2::new(549.0, 340.0)),
        111.844
    );

    assert_delta!(
        aurora.path(Vec2::new(308.0, 226.0), Vec2::new(195.0, 231.0)),
        113.579
    );

    assert_delta!(
        aurora.path(Vec2::new(827.0, 456.0), Vec2::new(921.0, 434.0)),
        117.226
    );

    assert_delta!(
        aurora.path(Vec2::new(469.0, 434.0), Vec2::new(455.0, 539.0)),
        112.804
    );

    assert_delta!(
        aurora.path(Vec2::new(135.0, 656.0), Vec2::new(124.0, 613.0)),
        111.949
    );

    assert_delta!(
        aurora.path(Vec2::new(508.0, 504.0), Vec2::new(393.0, 503.0)),
        116.462
    );

    assert_delta!(
        aurora.path(Vec2::new(515.0, 427.0), Vec2::new(516.0, 316.0)),
        113.893
    );

    assert_delta!(
        aurora.path(Vec2::new(221.0, 260.0), Vec2::new(124.0, 321.0)),
        114.586
    );

    assert_delta!(
        aurora.path(Vec2::new(492.0, 553.0), Vec2::new(480.0, 447.0)),
        115.111
    );

    assert_delta!(
        aurora.path(Vec2::new(265.0, 319.0), Vec2::new(195.0, 357.0)),
        115.028
    );

    assert_delta!(
        aurora.path(Vec2::new(550.0, 201.0), Vec2::new(451.0, 261.0)),
        115.763
    );

    assert_delta!(
        aurora.path(Vec2::new(323.0, 128.0), Vec2::new(337.0, 243.0)),
        115.849
    );

    assert_delta!(
        aurora.path(Vec2::new(429.0, 627.0), Vec2::new(555.0, 628.0)),
        126.004
    );

    assert_delta!(
        aurora.path(Vec2::new(734.0, 569.0), Vec2::new(850.0, 543.0)),
        118.878
    );

    assert_delta!(
        aurora.path(Vec2::new(700.0, 595.0), Vec2::new(820.0, 612.0)),
        121.277
    );

    assert_delta!(
        aurora.path(Vec2::new(841.0, 523.0), Vec2::new(837.0, 620.0)),
        118.67
    );

    assert_delta!(
        aurora.path(Vec2::new(422.0, 523.0), Vec2::new(484.0, 548.0)),
        119.616
    );

    assert_delta!(
        aurora.path(Vec2::new(556.0, 541.0), Vec2::new(524.0, 626.0)),
        124.426
    );

    assert_delta!(
        aurora.path(Vec2::new(159.0, 396.0), Vec2::new(186.0, 489.0)),
        120.74
    );

    assert_delta!(
        aurora.path(Vec2::new(816.0, 262.0), Vec2::new(918.0, 265.0)),
        118.635
    );

    assert_delta!(
        aurora.path(Vec2::new(340.0, 103.0), Vec2::new(445.0, 151.0)),
        115.451
    );

    assert_delta!(
        aurora.path(Vec2::new(755.0, 164.0), Vec2::new(852.0, 111.0)),
        116.292
    );

    assert_delta!(
        aurora.path(Vec2::new(810.0, 524.0), Vec2::new(894.0, 468.0)),
        120.924
    );

    assert_delta!(
        aurora.path(Vec2::new(605.0, 561.0), Vec2::new(689.0, 575.0)),
        123.012
    );

    assert_delta!(
        aurora.path(Vec2::new(371.0, 179.0), Vec2::new(263.0, 125.0)),
        120.748
    );

    assert_delta!(
        aurora.path(Vec2::new(298.0, 741.0), Vec2::new(181.0, 708.0)),
        121.644
    );

    assert_delta!(
        aurora.path(Vec2::new(556.0, 247.0), Vec2::new(531.0, 215.0)),
        123.026
    );

    assert_delta!(
        aurora.path(Vec2::new(858.0, 121.0), Vec2::new(760.0, 200.0)),
        126.749
    );

    assert_delta!(
        aurora.path(Vec2::new(259.0, 107.0), Vec2::new(333.0, 187.0)),
        119.329
    );

    assert_delta!(
        aurora.path(Vec2::new(449.0, 626.0), Vec2::new(549.0, 553.0)),
        124.803
    );

    assert_delta!(
        aurora.path(Vec2::new(422.0, 620.0), Vec2::new(315.0, 562.0)),
        121.709
    );

    assert_delta!(
        aurora.path(Vec2::new(736.0, 174.0), Vec2::new(623.0, 139.0)),
        122.716
    );

    assert_delta!(
        aurora.path(Vec2::new(571.0, 598.0), Vec2::new(450.0, 628.0)),
        124.733
    );

    assert_delta!(
        aurora.path(Vec2::new(494.0, 599.0), Vec2::new(604.0, 608.0)),
        129.666
    );

    assert_delta!(
        aurora.path(Vec2::new(323.0, 681.0), Vec2::new(236.0, 706.0)),
        121.637
    );

    assert_delta!(
        aurora.path(Vec2::new(873.0, 564.0), Vec2::new(977.0, 633.0)),
        124.808
    );

    assert_delta!(
        aurora.path(Vec2::new(232.0, 79.0), Vec2::new(117.0, 122.0)),
        123.911
    );

    assert_delta!(
        aurora.path(Vec2::new(35.0, 481.0), Vec2::new(40.0, 404.0)),
        128.897
    );

    assert_delta!(
        aurora.path(Vec2::new(416.0, 451.0), Vec2::new(296.0, 484.0)),
        125.304
    );

    assert_delta!(
        aurora.path(Vec2::new(456.0, 630.0), Vec2::new(563.0, 561.0)),
        127.318
    );

    assert_delta!(
        aurora.path(Vec2::new(287.0, 538.0), Vec2::new(407.0, 570.0)),
        124.193
    );

    assert_delta!(
        aurora.path(Vec2::new(433.0, 666.0), Vec2::new(318.0, 709.0)),
        122.776
    );

    assert_delta!(
        aurora.path(Vec2::new(523.0, 573.0), Vec2::new(497.0, 453.0)),
        127.196
    );

    assert_delta!(
        aurora.path(Vec2::new(139.0, 270.0), Vec2::new(183.0, 376.0)),
        130.02
    );

    assert_delta!(
        aurora.path(Vec2::new(229.0, 222.0), Vec2::new(126.0, 139.0)),
        132.28
    );

    assert_delta!(
        aurora.path(Vec2::new(609.0, 299.0), Vec2::new(508.0, 364.0)),
        128.118
    );

    assert_delta!(
        aurora.path(Vec2::new(345.0, 344.0), Vec2::new(438.0, 349.0)),
        126.209
    );

    assert_delta!(
        aurora.path(Vec2::new(432.0, 110.0), Vec2::new(552.0, 149.0)),
        126.5
    );

    assert_delta!(
        aurora.path(Vec2::new(201.0, 373.0), Vec2::new(64.0, 368.0)),
        137.091
    );

    assert_delta!(
        aurora.path(Vec2::new(469.0, 405.0), Vec2::new(564.0, 307.0)),
        136.488
    );

    assert_delta!(
        aurora.path(Vec2::new(765.0, 374.0), Vec2::new(894.0, 350.0)),
        131.334
    );

    assert_delta!(
        aurora.path(Vec2::new(567.0, 416.0), Vec2::new(623.0, 347.0)),
        128.501
    );

    assert_delta!(
        aurora.path(Vec2::new(982.0, 359.0), Vec2::new(859.0, 311.0)),
        133.355
    );

    assert_delta!(
        aurora.path(Vec2::new(825.0, 254.0), Vec2::new(712.0, 187.0)),
        132.362
    );

    assert_delta!(
        aurora.path(Vec2::new(177.0, 581.0), Vec2::new(271.0, 556.0)),
        136.399
    );

    assert_delta!(
        aurora.path(Vec2::new(611.0, 477.0), Vec2::new(705.0, 567.0)),
        136.083
    );

    assert_delta!(
        aurora.path(Vec2::new(467.0, 676.0), Vec2::new(598.0, 661.0)),
        133.475
    );

    assert_delta!(
        aurora.path(Vec2::new(260.0, 502.0), Vec2::new(291.0, 597.0)),
        136.976
    );

    assert_delta!(
        aurora.path(Vec2::new(805.0, 727.0), Vec2::new(758.0, 635.0)),
        134.216
    );

    assert_delta!(
        aurora.path(Vec2::new(429.0, 687.0), Vec2::new(342.0, 701.0)),
        132.358
    );

    assert_delta!(
        aurora.path(Vec2::new(607.0, 40.0), Vec2::new(730.0, 42.0)),
        133.171
    );

    assert_delta!(
        aurora.path(Vec2::new(262.0, 499.0), Vec2::new(376.0, 562.0)),
        130.287
    );

    assert_delta!(
        aurora.path(Vec2::new(473.0, 285.0), Vec2::new(604.0, 326.0)),
        139.231
    );

    assert_delta!(
        aurora.path(Vec2::new(528.0, 292.0), Vec2::new(572.0, 225.0)),
        137.677
    );

    assert_delta!(
        aurora.path(Vec2::new(800.0, 249.0), Vec2::new(720.0, 152.0)),
        138.227
    );

    assert_delta!(
        aurora.path(Vec2::new(554.0, 686.0), Vec2::new(425.0, 686.0)),
        135.514
    );

    assert_delta!(
        aurora.path(Vec2::new(156.0, 258.0), Vec2::new(293.0, 281.0)),
        140.492
    );

    assert_delta!(
        aurora.path(Vec2::new(404.0, 92.0), Vec2::new(525.0, 154.0)),
        135.96
    );

    assert_delta!(
        aurora.path(Vec2::new(682.0, 532.0), Vec2::new(654.0, 649.0)),
        134.929
    );

    assert_delta!(
        aurora.path(Vec2::new(340.0, 697.0), Vec2::new(306.0, 660.0)),
        136.33
    );

    assert_delta!(
        aurora.path(Vec2::new(688.0, 355.0), Vec2::new(817.0, 324.0)),
        135.82
    );

    assert_delta!(
        aurora.path(Vec2::new(524.0, 300.0), Vec2::new(547.0, 375.0)),
        134.456
    );

    assert_delta!(
        aurora.path(Vec2::new(427.0, 269.0), Vec2::new(551.0, 206.0)),
        139.086
    );

    assert_delta!(
        aurora.path(Vec2::new(449.0, 208.0), Vec2::new(348.0, 98.0)),
        149.416
    );

    assert_delta!(
        aurora.path(Vec2::new(457.0, 483.0), Vec2::new(366.0, 386.0)),
        142.206
    );

    assert_delta!(
        aurora.path(Vec2::new(850.0, 365.0), Vec2::new(957.0, 460.0)),
        144.053
    );

    assert_delta!(
        aurora.path(Vec2::new(766.0, 149.0), Vec2::new(907.0, 129.0)),
        142.585
    );

    assert_delta!(
        aurora.path(Vec2::new(541.0, 247.0), Vec2::new(413.0, 301.0)),
        140.008
    );

    assert_delta!(
        aurora.path(Vec2::new(825.0, 378.0), Vec2::new(918.0, 469.0)),
        141.696
    );

    assert_delta!(
        aurora.path(Vec2::new(711.0, 346.0), Vec2::new(831.0, 278.0)),
        137.928
    );

    assert_delta!(
        aurora.path(Vec2::new(880.0, 500.0), Vec2::new(837.0, 547.0)),
        138.679
    );

    assert_delta!(
        aurora.path(Vec2::new(686.0, 581.0), Vec2::new(812.0, 639.0)),
        138.763
    );

    assert_delta!(
        aurora.path(Vec2::new(121.0, 241.0), Vec2::new(39.0, 151.0)),
        142.817
    );

    assert_delta!(
        aurora.path(Vec2::new(334.0, 349.0), Vec2::new(202.0, 375.0)),
        142.821
    );

    assert_delta!(
        aurora.path(Vec2::new(935.0, 610.0), Vec2::new(833.0, 524.0)),
        143.02
    );

    assert_delta!(
        aurora.path(Vec2::new(455.0, 532.0), Vec2::new(406.0, 409.0)),
        149.544
    );

    assert_delta!(
        aurora.path(Vec2::new(812.0, 191.0), Vec2::new(672.0, 227.0)),
        144.617
    );

    assert_delta!(
        aurora.path(Vec2::new(683.0, 406.0), Vec2::new(667.0, 317.0)),
        144.737
    );

    assert_delta!(
        aurora.path(Vec2::new(486.0, 683.0), Vec2::new(628.0, 681.0)),
        145.697
    );

    assert_delta!(
        aurora.path(Vec2::new(399.0, 261.0), Vec2::new(308.0, 150.0)),
        146.233
    );

    assert_delta!(
        aurora.path(Vec2::new(773.0, 504.0), Vec2::new(903.0, 464.0)),
        141.756
    );

    assert_delta!(
        aurora.path(Vec2::new(321.0, 522.0), Vec2::new(215.0, 581.0)),
        144.722
    );

    assert_delta!(
        aurora.path(Vec2::new(790.0, 142.0), Vec2::new(810.0, 256.0)),
        147.918
    );

    assert_delta!(
        aurora.path(Vec2::new(797.0, 248.0), Vec2::new(933.0, 276.0)),
        147.208
    );

    assert_delta!(
        aurora.path(Vec2::new(769.0, 200.0), Vec2::new(900.0, 138.0)),
        144.931
    );

    assert_delta!(
        aurora.path(Vec2::new(588.0, 367.0), Vec2::new(497.0, 325.0)),
        148.565
    );

    assert_delta!(
        aurora.path(Vec2::new(277.0, 389.0), Vec2::new(386.0, 300.0)),
        148.544
    );

    assert_delta!(
        aurora.path(Vec2::new(119.0, 646.0), Vec2::new(233.0, 723.0)),
        146.179
    );

    assert_delta!(
        aurora.path(Vec2::new(579.0, 135.0), Vec2::new(672.0, 236.0)),
        147.476
    );

    assert_delta!(
        aurora.path(Vec2::new(237.0, 54.0), Vec2::new(203.0, 197.0)),
        149.81
    );

    assert_delta!(
        aurora.path(Vec2::new(185.0, 152.0), Vec2::new(33.0, 158.0)),
        153.783
    );

    assert_delta!(
        aurora.path(Vec2::new(920.0, 444.0), Vec2::new(789.0, 429.0)),
        149.376
    );

    assert_delta!(
        aurora.path(Vec2::new(594.0, 182.0), Vec2::new(742.0, 187.0)),
        150.47
    );

    assert_delta!(
        aurora.path(Vec2::new(443.0, 480.0), Vec2::new(389.0, 588.0)),
        152.957
    );

    assert_delta!(
        aurora.path(Vec2::new(882.0, 490.0), Vec2::new(869.0, 369.0)),
        155.027
    );

    assert_delta!(
        aurora.path(Vec2::new(258.0, 125.0), Vec2::new(340.0, 252.0)),
        152.023
    );

    assert_delta!(
        aurora.path(Vec2::new(236.0, 480.0), Vec2::new(88.0, 454.0)),
        152.556
    );

    assert_delta!(
        aurora.path(Vec2::new(766.0, 715.0), Vec2::new(772.0, 609.0)),
        155.232
    );

    assert_delta!(
        aurora.path(Vec2::new(820.0, 274.0), Vec2::new(950.0, 349.0)),
        150.106
    );

    assert_delta!(
        aurora.path(Vec2::new(355.0, 563.0), Vec2::new(503.0, 546.0)),
        152.626
    );

    assert_delta!(
        aurora.path(Vec2::new(794.0, 674.0), Vec2::new(928.0, 604.0)),
        151.182
    );

    assert_delta!(
        aurora.path(Vec2::new(462.0, 255.0), Vec2::new(487.0, 147.0)),
        151.721
    );

    assert_delta!(
        aurora.path(Vec2::new(272.0, 577.0), Vec2::new(113.0, 582.0)),
        160.231
    );

    assert_delta!(
        aurora.path(Vec2::new(222.0, 644.0), Vec2::new(373.0, 677.0)),
        160.811
    );

    assert_delta!(
        aurora.path(Vec2::new(335.0, 158.0), Vec2::new(481.0, 202.0)),
        153.226
    );

    assert_delta!(
        aurora.path(Vec2::new(260.0, 312.0), Vec2::new(185.0, 416.0)),
        158.572
    );

    assert_delta!(
        aurora.path(Vec2::new(582.0, 598.0), Vec2::new(473.0, 673.0)),
        153.715
    );

    assert_delta!(
        aurora.path(Vec2::new(633.0, 195.0), Vec2::new(708.0, 70.0)),
        158.24
    );

    assert_delta!(
        aurora.path(Vec2::new(184.0, 541.0), Vec2::new(53.0, 559.0)),
        154.085
    );

    assert_delta!(
        aurora.path(Vec2::new(121.0, 603.0), Vec2::new(273.0, 567.0)),
        156.453
    );

    assert_delta!(
        aurora.path(Vec2::new(617.0, 500.0), Vec2::new(478.0, 556.0)),
        157.818
    );

    assert_delta!(
        aurora.path(Vec2::new(926.0, 461.0), Vec2::new(989.0, 330.0)),
        158.092
    );

    assert_delta!(
        aurora.path(Vec2::new(557.0, 132.0), Vec2::new(412.0, 193.0)),
        165.026
    );

    assert_delta!(
        aurora.path(Vec2::new(934.0, 627.0), Vec2::new(770.0, 616.0)),
        164.585
    );

    assert_delta!(
        aurora.path(Vec2::new(305.0, 697.0), Vec2::new(187.0, 711.0)),
        159.321
    );

    assert_delta!(
        aurora.path(Vec2::new(316.0, 373.0), Vec2::new(410.0, 372.0)),
        157.286
    );

    assert_delta!(
        aurora.path(Vec2::new(258.0, 627.0), Vec2::new(146.0, 530.0)),
        157.567
    );

    assert_delta!(
        aurora.path(Vec2::new(836.0, 651.0), Vec2::new(815.0, 492.0)),
        161.868
    );

    assert_delta!(
        aurora.path(Vec2::new(351.0, 368.0), Vec2::new(348.0, 462.0)),
        159.461
    );

    assert_delta!(
        aurora.path(Vec2::new(206.0, 469.0), Vec2::new(330.0, 573.0)),
        163.413
    );

    assert_delta!(
        aurora.path(Vec2::new(692.0, 544.0), Vec2::new(835.0, 475.0)),
        158.777
    );

    assert_delta!(
        aurora.path(Vec2::new(265.0, 149.0), Vec2::new(131.0, 201.0)),
        161.894
    );

    assert_delta!(
        aurora.path(Vec2::new(338.0, 474.0), Vec2::new(213.0, 434.0)),
        163.459
    );

    assert_delta!(
        aurora.path(Vec2::new(979.0, 430.0), Vec2::new(850.0, 441.0)),
        161.263
    );

    assert_delta!(
        aurora.path(Vec2::new(329.0, 193.0), Vec2::new(233.0, 281.0)),
        159.323
    );

    assert_delta!(
        aurora.path(Vec2::new(306.0, 315.0), Vec2::new(421.0, 436.0)),
        169.07
    );

    assert_delta!(
        aurora.path(Vec2::new(142.0, 660.0), Vec2::new(232.0, 524.0)),
        168.439
    );

    assert_delta!(
        aurora.path(Vec2::new(358.0, 115.0), Vec2::new(212.0, 165.0)),
        162.386
    );

    assert_delta!(
        aurora.path(Vec2::new(308.0, 387.0), Vec2::new(177.0, 478.0)),
        161.827
    );

    assert_delta!(
        aurora.path(Vec2::new(577.0, 373.0), Vec2::new(724.0, 416.0)),
        159.53
    );

    assert_delta!(
        aurora.path(Vec2::new(373.0, 124.0), Vec2::new(292.0, 210.0)),
        164.841
    );

    assert_delta!(
        aurora.path(Vec2::new(588.0, 113.0), Vec2::new(451.0, 204.0)),
        166.026
    );

    assert_delta!(
        aurora.path(Vec2::new(511.0, 192.0), Vec2::new(357.0, 153.0)),
        164.475
    );

    assert_delta!(
        aurora.path(Vec2::new(86.0, 220.0), Vec2::new(140.0, 338.0)),
        167.778
    );

    assert_delta!(
        aurora.path(Vec2::new(283.0, 146.0), Vec2::new(271.0, 245.0)),
        166.83
    );

    assert_delta!(
        aurora.path(Vec2::new(752.0, 123.0), Vec2::new(592.0, 151.0)),
        167.203
    );

    assert_delta!(
        aurora.path(Vec2::new(249.0, 473.0), Vec2::new(177.0, 616.0)),
        172.179
    );

    assert_delta!(
        aurora.path(Vec2::new(750.0, 343.0), Vec2::new(906.0, 343.0)),
        164.868
    );

    assert_delta!(
        aurora.path(Vec2::new(281.0, 680.0), Vec2::new(155.0, 673.0)),
        164.082
    );

    assert_delta!(
        aurora.path(Vec2::new(404.0, 581.0), Vec2::new(373.0, 483.0)),
        167.047
    );

    assert_delta!(
        aurora.path(Vec2::new(855.0, 365.0), Vec2::new(870.0, 480.0)),
        170.752
    );

    assert_delta!(
        aurora.path(Vec2::new(858.0, 461.0), Vec2::new(806.0, 610.0)),
        169.693
    );

    assert_delta!(
        aurora.path(Vec2::new(931.0, 443.0), Vec2::new(1003.0, 299.0)),
        174.761
    );

    assert_delta!(
        aurora.path(Vec2::new(534.0, 473.0), Vec2::new(390.0, 386.0)),
        168.241
    );

    assert_delta!(
        aurora.path(Vec2::new(414.0, 519.0), Vec2::new(570.0, 564.0)),
        170.573
    );

    assert_delta!(
        aurora.path(Vec2::new(437.0, 393.0), Vec2::new(545.0, 280.0)),
        171.173
    );

    assert_delta!(
        aurora.path(Vec2::new(83.0, 474.0), Vec2::new(169.0, 326.0)),
        171.883
    );

    assert_delta!(
        aurora.path(Vec2::new(181.0, 636.0), Vec2::new(23.0, 612.0)),
        172.545
    );

    assert_delta!(
        aurora.path(Vec2::new(375.0, 604.0), Vec2::new(223.0, 533.0)),
        171.12
    );

    assert_delta!(
        aurora.path(Vec2::new(935.0, 612.0), Vec2::new(800.0, 693.0)),
        167.634
    );

    assert_delta!(
        aurora.path(Vec2::new(431.0, 242.0), Vec2::new(279.0, 173.0)),
        167.413
    );

    assert_delta!(
        aurora.path(Vec2::new(465.0, 295.0), Vec2::new(622.0, 339.0)),
        169.643
    );

    assert_delta!(
        aurora.path(Vec2::new(544.0, 660.0), Vec2::new(642.0, 514.0)),
        175.98
    );

    assert_delta!(
        aurora.path(Vec2::new(243.0, 688.0), Vec2::new(240.0, 528.0)),
        178.875
    );

    assert_delta!(
        aurora.path(Vec2::new(819.0, 231.0), Vec2::new(650.0, 195.0)),
        175.312
    );

    assert_delta!(
        aurora.path(Vec2::new(375.0, 484.0), Vec2::new(519.0, 556.0)),
        171.601
    );

    assert_delta!(
        aurora.path(Vec2::new(322.0, 545.0), Vec2::new(480.0, 508.0)),
        174.304
    );

    assert_delta!(
        aurora.path(Vec2::new(924.0, 560.0), Vec2::new(856.0, 466.0)),
        172.538
    );

    assert_delta!(
        aurora.path(Vec2::new(579.0, 293.0), Vec2::new(411.0, 311.0)),
        172.892
    );

    assert_delta!(
        aurora.path(Vec2::new(334.0, 331.0), Vec2::new(503.0, 339.0)),
        175.926
    );

    assert_delta!(
        aurora.path(Vec2::new(679.0, 354.0), Vec2::new(590.0, 280.0)),
        172.808
    );

    assert_delta!(
        aurora.path(Vec2::new(551.0, 328.0), Vec2::new(377.0, 315.0)),
        177.865
    );

    assert_delta!(
        aurora.path(Vec2::new(890.0, 490.0), Vec2::new(980.0, 446.0)),
        181.533
    );

    assert_delta!(
        aurora.path(Vec2::new(243.0, 295.0), Vec2::new(101.0, 364.0)),
        177.181
    );

    assert_delta!(
        aurora.path(Vec2::new(846.0, 438.0), Vec2::new(741.0, 294.0)),
        181.413
    );

    assert_delta!(
        aurora.path(Vec2::new(766.0, 514.0), Vec2::new(610.0, 596.0)),
        176.298
    );

    assert_delta!(
        aurora.path(Vec2::new(2.0, 409.0), Vec2::new(174.0, 390.0)),
        179.584
    );

    assert_delta!(
        aurora.path(Vec2::new(35.0, 506.0), Vec2::new(204.0, 453.0)),
        178.924
    );

    assert_delta!(
        aurora.path(Vec2::new(276.0, 248.0), Vec2::new(236.0, 357.0)),
        179.213
    );

    assert_delta!(
        aurora.path(Vec2::new(346.0, 662.0), Vec2::new(189.0, 686.0)),
        175.221
    );

    assert_delta!(
        aurora.path(Vec2::new(915.0, 136.0), Vec2::new(743.0, 110.0)),
        176.634
    );

    assert_delta!(
        aurora.path(Vec2::new(459.0, 488.0), Vec2::new(286.0, 512.0)),
        181.012
    );

    assert_delta!(
        aurora.path(Vec2::new(809.0, 250.0), Vec2::new(904.0, 325.0)),
        180.298
    );

    assert_delta!(
        aurora.path(Vec2::new(681.0, 180.0), Vec2::new(844.0, 194.0)),
        179.713
    );

    assert_delta!(
        aurora.path(Vec2::new(863.0, 117.0), Vec2::new(678.0, 100.0)),
        186.097
    );

    assert_delta!(
        aurora.path(Vec2::new(332.0, 99.0), Vec2::new(493.0, 55.0)),
        179.698
    );

    assert_delta!(
        aurora.path(Vec2::new(603.0, 188.0), Vec2::new(762.0, 104.0)),
        179.825
    );

    assert_delta!(
        aurora.path(Vec2::new(650.0, 622.0), Vec2::new(764.0, 530.0)),
        182.094
    );

    assert_delta!(
        aurora.path(Vec2::new(255.0, 23.0), Vec2::new(414.0, 108.0)),
        180.294
    );

    assert_delta!(
        aurora.path(Vec2::new(464.0, 444.0), Vec2::new(589.0, 560.0)),
        181.807
    );

    assert_delta!(
        aurora.path(Vec2::new(634.0, 294.0), Vec2::new(780.0, 365.0)),
        181.317
    );

    assert_delta!(
        aurora.path(Vec2::new(894.0, 172.0), Vec2::new(725.0, 211.0)),
        183.489
    );

    assert_delta!(
        aurora.path(Vec2::new(197.0, 485.0), Vec2::new(39.0, 386.0)),
        187.699
    );

    assert_delta!(
        aurora.path(Vec2::new(448.0, 230.0), Vec2::new(609.0, 248.0)),
        184.641
    );

    assert_delta!(
        aurora.path(Vec2::new(168.0, 170.0), Vec2::new(338.0, 235.0)),
        182.003
    );

    assert_delta!(
        aurora.path(Vec2::new(70.0, 352.0), Vec2::new(45.0, 519.0)),
        183.486
    );

    assert_delta!(
        aurora.path(Vec2::new(232.0, 643.0), Vec2::new(105.0, 516.0)),
        189.423
    );

    assert_delta!(
        aurora.path(Vec2::new(696.0, 112.0), Vec2::new(565.0, 38.0)),
        183.043
    );

    assert_delta!(
        aurora.path(Vec2::new(481.0, 361.0), Vec2::new(309.0, 330.0)),
        183.115
    );

    assert_delta!(
        aurora.path(Vec2::new(248.0, 213.0), Vec2::new(251.0, 53.0)),
        185.151
    );

    assert_delta!(
        aurora.path(Vec2::new(245.0, 323.0), Vec2::new(99.0, 228.0)),
        185.278
    );

    assert_delta!(
        aurora.path(Vec2::new(175.0, 616.0), Vec2::new(52.0, 509.0)),
        188.069
    );

    assert_delta!(
        aurora.path(Vec2::new(592.0, 376.0), Vec2::new(681.0, 451.0)),
        192.924
    );

    assert_delta!(
        aurora.path(Vec2::new(697.0, 675.0), Vec2::new(642.0, 510.0)),
        191.851
    );

    assert_delta!(
        aurora.path(Vec2::new(781.0, 198.0), Vec2::new(602.0, 154.0)),
        188.678
    );

    assert_delta!(
        aurora.path(Vec2::new(749.0, 97.0), Vec2::new(621.0, 222.0)),
        186.518
    );

    assert_delta!(
        aurora.path(Vec2::new(322.0, 287.0), Vec2::new(173.0, 328.0)),
        188.084
    );

    assert_delta!(
        aurora.path(Vec2::new(854.0, 448.0), Vec2::new(823.0, 629.0)),
        194.695
    );

    assert_delta!(
        aurora.path(Vec2::new(602.0, 513.0), Vec2::new(678.0, 630.0)),
        186.046
    );

    assert_delta!(
        aurora.path(Vec2::new(747.0, 566.0), Vec2::new(749.0, 669.0)),
        192.915
    );

    assert_delta!(
        aurora.path(Vec2::new(355.0, 266.0), Vec2::new(463.0, 157.0)),
        186.596
    );

    assert_delta!(
        aurora.path(Vec2::new(147.0, 705.0), Vec2::new(321.0, 638.0)),
        186.55
    );

    assert_delta!(
        aurora.path(Vec2::new(608.0, 378.0), Vec2::new(471.0, 300.0)),
        194.326
    );

    assert_delta!(
        aurora.path(Vec2::new(187.0, 136.0), Vec2::new(345.0, 253.0)),
        196.664
    );

    assert_delta!(
        aurora.path(Vec2::new(537.0, 209.0), Vec2::new(371.0, 305.0)),
        191.76
    );

    assert_delta!(
        aurora.path(Vec2::new(549.0, 562.0), Vec2::new(735.0, 514.0)),
        194.016
    );

    assert_delta!(
        aurora.path(Vec2::new(33.0, 601.0), Vec2::new(223.0, 623.0)),
        196.21
    );

    assert_delta!(
        aurora.path(Vec2::new(475.0, 301.0), Vec2::new(658.0, 251.0)),
        198.251
    );

    assert_delta!(
        aurora.path(Vec2::new(842.0, 670.0), Vec2::new(652.0, 650.0)),
        193.201
    );

    assert_delta!(
        aurora.path(Vec2::new(103.0, 700.0), Vec2::new(228.0, 565.0)),
        195.824
    );

    assert_delta!(
        aurora.path(Vec2::new(386.0, 661.0), Vec2::new(524.0, 644.0)),
        200.087
    );

    assert_delta!(
        aurora.path(Vec2::new(835.0, 475.0), Vec2::new(889.0, 626.0)),
        196.045
    );

    assert_delta!(
        aurora.path(Vec2::new(990.0, 423.0), Vec2::new(819.0, 369.0)),
        196.748
    );

    assert_delta!(
        aurora.path(Vec2::new(915.0, 533.0), Vec2::new(978.0, 406.0)),
        197.576
    );

    assert_delta!(
        aurora.path(Vec2::new(794.0, 134.0), Vec2::new(620.0, 51.0)),
        193.667
    );

    assert_delta!(
        aurora.path(Vec2::new(9.0, 437.0), Vec2::new(160.0, 504.0)),
        204.523
    );

    assert_delta!(
        aurora.path(Vec2::new(88.0, 292.0), Vec2::new(277.0, 242.0)),
        197.357
    );

    assert_delta!(
        aurora.path(Vec2::new(226.0, 229.0), Vec2::new(33.0, 197.0)),
        197.895
    );

    assert_delta!(
        aurora.path(Vec2::new(530.0, 162.0), Vec2::new(726.0, 192.0)),
        199.664
    );

    assert_delta!(
        aurora.path(Vec2::new(508.0, 454.0), Vec2::new(518.0, 633.0)),
        199.708
    );

    assert_delta!(
        aurora.path(Vec2::new(771.0, 378.0), Vec2::new(628.0, 315.0)),
        198.242
    );

    assert_delta!(
        aurora.path(Vec2::new(792.0, 84.0), Vec2::new(626.0, 188.0)),
        196.305
    );

    assert_delta!(
        aurora.path(Vec2::new(402.0, 505.0), Vec2::new(597.0, 501.0)),
        206.097
    );

    assert_delta!(
        aurora.path(Vec2::new(978.0, 326.0), Vec2::new(983.0, 455.0)),
        207.018
    );

    assert_delta!(
        aurora.path(Vec2::new(304.0, 537.0), Vec2::new(491.0, 472.0)),
        201.396
    );

    assert_delta!(
        aurora.path(Vec2::new(622.0, 698.0), Vec2::new(813.0, 660.0)),
        199.122
    );

    assert_delta!(
        aurora.path(Vec2::new(389.0, 476.0), Vec2::new(337.0, 316.0)),
        204.466
    );

    assert_delta!(
        aurora.path(Vec2::new(958.0, 384.0), Vec2::new(877.0, 243.0)),
        202.756
    );

    assert_delta!(
        aurora.path(Vec2::new(133.0, 106.0), Vec2::new(220.0, 220.0)),
        202.284
    );

    assert_delta!(
        aurora.path(Vec2::new(681.0, 697.0), Vec2::new(548.0, 605.0)),
        199.454
    );

    assert_delta!(
        aurora.path(Vec2::new(701.0, 426.0), Vec2::new(836.0, 522.0)),
        198.012
    );

    assert_delta!(
        aurora.path(Vec2::new(160.0, 291.0), Vec2::new(42.0, 434.0)),
        199.254
    );

    assert_delta!(
        aurora.path(Vec2::new(396.0, 590.0), Vec2::new(590.0, 644.0)),
        202.771
    );

    assert_delta!(
        aurora.path(Vec2::new(801.0, 446.0), Vec2::new(730.0, 324.0)),
        206.696
    );

    assert_delta!(
        aurora.path(Vec2::new(597.0, 457.0), Vec2::new(414.0, 377.0)),
        200.115
    );

    assert_delta!(
        aurora.path(Vec2::new(125.0, 216.0), Vec2::new(298.0, 174.0)),
        203.715
    );

    assert_delta!(
        aurora.path(Vec2::new(273.0, 525.0), Vec2::new(288.0, 665.0)),
        202.155
    );

    assert_delta!(
        aurora.path(Vec2::new(601.0, 533.0), Vec2::new(715.0, 665.0)),
        205.821
    );

    assert_delta!(
        aurora.path(Vec2::new(711.0, 266.0), Vec2::new(874.0, 175.0)),
        206.216
    );

    assert_delta!(
        aurora.path(Vec2::new(293.0, 249.0), Vec2::new(177.0, 172.0)),
        201.476
    );

    assert_delta!(
        aurora.path(Vec2::new(444.0, 430.0), Vec2::new(489.0, 614.0)),
        207.735
    );

    assert_delta!(
        aurora.path(Vec2::new(458.0, 532.0), Vec2::new(258.0, 539.0)),
        206.98
    );

    assert_delta!(
        aurora.path(Vec2::new(664.0, 364.0), Vec2::new(875.0, 362.0)),
        213.235
    );

    assert_delta!(
        aurora.path(Vec2::new(653.0, 318.0), Vec2::new(506.0, 194.0)),
        206.847
    );

    assert_delta!(
        aurora.path(Vec2::new(847.0, 326.0), Vec2::new(652.0, 330.0)),
        207.1
    );

    assert_delta!(
        aurora.path(Vec2::new(494.0, 280.0), Vec2::new(583.0, 410.0)),
        208.274
    );

    assert_delta!(
        aurora.path(Vec2::new(38.0, 554.0), Vec2::new(200.0, 461.0)),
        208.278
    );

    assert_delta!(
        aurora.path(Vec2::new(805.0, 474.0), Vec2::new(687.0, 354.0)),
        207.139
    );

    assert_delta!(
        aurora.path(Vec2::new(425.0, 77.0), Vec2::new(303.0, 228.0)),
        211.559
    );

    assert_delta!(
        aurora.path(Vec2::new(920.0, 301.0), Vec2::new(910.0, 492.0)),
        210.543
    );

    assert_delta!(
        aurora.path(Vec2::new(1018.0, 306.0), Vec2::new(862.0, 434.0)),
        207.889
    );

    assert_delta!(
        aurora.path(Vec2::new(796.0, 115.0), Vec2::new(595.0, 105.0)),
        207.152
    );

    assert_delta!(
        aurora.path(Vec2::new(957.0, 321.0), Vec2::new(877.0, 183.0)),
        211.778
    );

    assert_delta!(
        aurora.path(Vec2::new(418.0, 694.0), Vec2::new(625.0, 670.0)),
        213.136
    );

    assert_delta!(
        aurora.path(Vec2::new(604.0, 732.0), Vec2::new(607.0, 690.0)),
        212.884
    );

    assert_delta!(
        aurora.path(Vec2::new(922.0, 415.0), Vec2::new(864.0, 522.0)),
        210.044
    );

    assert_delta!(
        aurora.path(Vec2::new(385.0, 578.0), Vec2::new(567.0, 680.0)),
        211.439
    );

    assert_delta!(
        aurora.path(Vec2::new(213.0, 460.0), Vec2::new(13.0, 521.0)),
        211.274
    );

    assert_delta!(
        aurora.path(Vec2::new(42.0, 588.0), Vec2::new(199.0, 701.0)),
        213.179
    );

    assert_delta!(
        aurora.path(Vec2::new(284.0, 85.0), Vec2::new(368.0, 275.0)),
        209.189
    );

    assert_delta!(
        aurora.path(Vec2::new(210.0, 647.0), Vec2::new(232.0, 466.0)),
        215.503
    );

    assert_delta!(
        aurora.path(Vec2::new(251.0, 231.0), Vec2::new(111.0, 118.0)),
        210.622
    );

    assert_delta!(
        aurora.path(Vec2::new(778.0, 647.0), Vec2::new(788.0, 487.0)),
        214.56
    );

    assert_delta!(
        aurora.path(Vec2::new(357.0, 484.0), Vec2::new(476.0, 576.0)),
        213.721
    );

    assert_delta!(
        aurora.path(Vec2::new(413.0, 177.0), Vec2::new(473.0, 271.0)),
        214.377
    );

    assert_delta!(
        aurora.path(Vec2::new(164.0, 351.0), Vec2::new(230.0, 415.0)),
        221.926
    );

    assert_delta!(
        aurora.path(Vec2::new(960.0, 274.0), Vec2::new(770.0, 370.0)),
        213.957
    );

    assert_delta!(
        aurora.path(Vec2::new(173.0, 227.0), Vec2::new(187.0, 346.0)),
        215.753
    );

    assert_delta!(
        aurora.path(Vec2::new(894.0, 487.0), Vec2::new(789.0, 348.0)),
        217.319
    );

    assert_delta!(
        aurora.path(Vec2::new(225.0, 274.0), Vec2::new(421.0, 219.0)),
        217.784
    );

    assert_delta!(
        aurora.path(Vec2::new(635.0, 584.0), Vec2::new(837.0, 652.0)),
        214.457
    );

    assert_delta!(
        aurora.path(Vec2::new(621.0, 153.0), Vec2::new(526.0, 28.0)),
        214.572
    );

    assert_delta!(
        aurora.path(Vec2::new(881.0, 244.0), Vec2::new(848.0, 129.0)),
        218.857
    );

    assert_delta!(
        aurora.path(Vec2::new(741.0, 412.0), Vec2::new(664.0, 523.0)),
        216.44
    );

    assert_delta!(
        aurora.path(Vec2::new(407.0, 561.0), Vec2::new(511.0, 666.0)),
        217.731
    );

    assert_delta!(
        aurora.path(Vec2::new(778.0, 507.0), Vec2::new(950.0, 616.0)),
        218.827
    );

    assert_delta!(
        aurora.path(Vec2::new(495.0, 376.0), Vec2::new(335.0, 371.0)),
        214.937
    );

    assert_delta!(
        aurora.path(Vec2::new(944.0, 333.0), Vec2::new(728.0, 326.0)),
        221.049
    );

    assert_delta!(
        aurora.path(Vec2::new(511.0, 389.0), Vec2::new(309.0, 318.0)),
        220.553
    );

    assert_delta!(
        aurora.path(Vec2::new(713.0, 537.0), Vec2::new(504.0, 580.0)),
        217.888
    );

    assert_delta!(
        aurora.path(Vec2::new(504.0, 160.0), Vec2::new(603.0, 270.0)),
        221.895
    );

    assert_delta!(
        aurora.path(Vec2::new(799.0, 504.0), Vec2::new(954.0, 383.0)),
        218.98
    );

    assert_delta!(
        aurora.path(Vec2::new(364.0, 614.0), Vec2::new(255.0, 485.0)),
        220.263
    );

    assert_delta!(
        aurora.path(Vec2::new(365.0, 602.0), Vec2::new(588.0, 600.0)),
        226.416
    );

    assert_delta!(
        aurora.path(Vec2::new(547.0, 281.0), Vec2::new(696.0, 353.0)),
        219.968
    );

    assert_delta!(
        aurora.path(Vec2::new(721.0, 394.0), Vec2::new(539.0, 341.0)),
        221.279
    );

    assert_delta!(
        aurora.path(Vec2::new(156.0, 297.0), Vec2::new(318.0, 375.0)),
        220.32
    );

    assert_delta!(
        aurora.path(Vec2::new(104.0, 374.0), Vec2::new(79.0, 214.0)),
        227.57
    );

    assert_delta!(
        aurora.path(Vec2::new(459.0, 364.0), Vec2::new(513.0, 181.0)),
        227.045
    );

    assert_delta!(
        aurora.path(Vec2::new(422.0, 339.0), Vec2::new(220.0, 267.0)),
        222.11
    );

    assert_delta!(
        aurora.path(Vec2::new(581.0, 681.0), Vec2::new(482.0, 637.0)),
        221.812
    );

    assert_delta!(
        aurora.path(Vec2::new(723.0, 329.0), Vec2::new(555.0, 221.0)),
        219.11
    );

    assert_delta!(
        aurora.path(Vec2::new(438.0, 636.0), Vec2::new(661.0, 594.0)),
        229.07
    );

    assert_delta!(
        aurora.path(Vec2::new(424.0, 411.0), Vec2::new(634.0, 465.0)),
        225.474
    );

    assert_delta!(
        aurora.path(Vec2::new(45.0, 416.0), Vec2::new(229.0, 473.0)),
        223.592
    );

    assert_delta!(
        aurora.path(Vec2::new(198.0, 352.0), Vec2::new(52.0, 534.0)),
        239.656
    );

    assert_delta!(
        aurora.path(Vec2::new(44.0, 429.0), Vec2::new(184.0, 271.0)),
        226.393
    );

    assert_delta!(
        aurora.path(Vec2::new(294.0, 324.0), Vec2::new(486.0, 422.0)),
        234.908
    );

    assert_delta!(
        aurora.path(Vec2::new(481.0, 366.0), Vec2::new(304.0, 383.0)),
        224.219
    );

    assert_delta!(
        aurora.path(Vec2::new(376.0, 611.0), Vec2::new(368.0, 463.0)),
        230.688
    );

    assert_delta!(
        aurora.path(Vec2::new(738.0, 262.0), Vec2::new(858.0, 135.0)),
        223.436
    );

    assert_delta!(
        aurora.path(Vec2::new(237.0, 603.0), Vec2::new(26.0, 656.0)),
        223.627
    );

    assert_delta!(
        aurora.path(Vec2::new(609.0, 415.0), Vec2::new(624.0, 452.0)),
        225.432
    );

    assert_delta!(
        aurora.path(Vec2::new(989.0, 596.0), Vec2::new(838.0, 473.0)),
        228.117
    );

    assert_delta!(
        aurora.path(Vec2::new(431.0, 164.0), Vec2::new(194.0, 167.0)),
        239.016
    );

    assert_delta!(
        aurora.path(Vec2::new(660.0, 726.0), Vec2::new(855.0, 640.0)),
        229.616
    );

    assert_delta!(
        aurora.path(Vec2::new(770.0, 639.0), Vec2::new(555.0, 620.0)),
        227.223
    );

    assert_delta!(
        aurora.path(Vec2::new(797.0, 613.0), Vec2::new(599.0, 508.0)),
        231.476
    );

    assert_delta!(
        aurora.path(Vec2::new(695.0, 661.0), Vec2::new(618.0, 482.0)),
        235.097
    );

    assert_delta!(
        aurora.path(Vec2::new(211.0, 660.0), Vec2::new(89.0, 541.0)),
        232.407
    );

    assert_delta!(
        aurora.path(Vec2::new(387.0, 514.0), Vec2::new(300.0, 483.0)),
        233.101
    );

    assert_delta!(
        aurora.path(Vec2::new(147.0, 333.0), Vec2::new(199.0, 244.0)),
        231.198
    );

    assert_delta!(
        aurora.path(Vec2::new(446.0, 654.0), Vec2::new(527.0, 564.0)),
        238.427
    );

    assert_delta!(
        aurora.path(Vec2::new(862.0, 484.0), Vec2::new(751.0, 638.0)),
        233.369
    );

    assert_delta!(
        aurora.path(Vec2::new(323.0, 708.0), Vec2::new(90.0, 686.0)),
        239.059
    );

    assert_delta!(
        aurora.path(Vec2::new(423.0, 93.0), Vec2::new(301.0, 274.0)),
        238.412
    );

    assert_delta!(
        aurora.path(Vec2::new(890.0, 296.0), Vec2::new(860.0, 449.0)),
        237.411
    );

    assert_delta!(
        aurora.path(Vec2::new(287.0, 144.0), Vec2::new(505.0, 217.0)),
        231.767
    );

    assert_delta!(
        aurora.path(Vec2::new(449.0, 686.0), Vec2::new(308.0, 707.0)),
        232.008
    );

    assert_delta!(
        aurora.path(Vec2::new(461.0, 307.0), Vec2::new(452.0, 113.0)),
        242.494
    );

    assert_delta!(
        aurora.path(Vec2::new(231.0, 480.0), Vec2::new(218.0, 354.0)),
        235.39
    );

    assert_delta!(
        aurora.path(Vec2::new(434.0, 156.0), Vec2::new(198.0, 177.0)),
        242.862
    );

    assert_delta!(
        aurora.path(Vec2::new(734.0, 573.0), Vec2::new(530.0, 683.0)),
        240.421
    );

    assert_delta!(
        aurora.path(Vec2::new(384.0, 294.0), Vec2::new(160.0, 295.0)),
        235.407
    );

    assert_delta!(
        aurora.path(Vec2::new(367.0, 280.0), Vec2::new(560.0, 160.0)),
        242.259
    );

    assert_delta!(
        aurora.path(Vec2::new(932.0, 416.0), Vec2::new(771.0, 539.0)),
        235.097
    );

    assert_delta!(
        aurora.path(Vec2::new(808.0, 601.0), Vec2::new(575.0, 637.0)),
        243.909
    );

    assert_delta!(
        aurora.path(Vec2::new(194.0, 347.0), Vec2::new(33.0, 534.0)),
        247.181
    );

    assert_delta!(
        aurora.path(Vec2::new(495.0, 495.0), Vec2::new(729.0, 532.0)),
        239.689
    );

    assert_delta!(
        aurora.path(Vec2::new(399.0, 647.0), Vec2::new(502.0, 603.0)),
        239.987
    );

    assert_delta!(
        aurora.path(Vec2::new(714.0, 358.0), Vec2::new(573.0, 212.0)),
        237.645
    );

    assert_delta!(
        aurora.path(Vec2::new(642.0, 102.0), Vec2::new(848.0, 169.0)),
        235.78
    );

    assert_delta!(
        aurora.path(Vec2::new(461.0, 292.0), Vec2::new(234.0, 325.0)),
        239.853
    );

    assert_delta!(
        aurora.path(Vec2::new(528.0, 141.0), Vec2::new(288.0, 137.0)),
        244.596
    );

    assert_delta!(
        aurora.path(Vec2::new(699.0, 588.0), Vec2::new(496.0, 684.0)),
        242.071
    );

    assert_delta!(
        aurora.path(Vec2::new(571.0, 474.0), Vec2::new(795.0, 527.0)),
        240.125
    );

    assert_delta!(
        aurora.path(Vec2::new(852.0, 268.0), Vec2::new(964.0, 434.0)),
        241.389
    );

    assert_delta!(
        aurora.path(Vec2::new(591.0, 564.0), Vec2::new(417.0, 671.0)),
        244.188
    );

    assert_delta!(
        aurora.path(Vec2::new(232.0, 401.0), Vec2::new(301.0, 302.0)),
        243.265
    );

    assert_delta!(
        aurora.path(Vec2::new(55.0, 589.0), Vec2::new(298.0, 616.0)),
        248.444
    );

    assert_delta!(
        aurora.path(Vec2::new(353.0, 453.0), Vec2::new(164.0, 593.0)),
        241.667
    );

    assert_delta!(
        aurora.path(Vec2::new(623.0, 706.0), Vec2::new(827.0, 626.0)),
        243.438
    );

    assert_delta!(
        aurora.path(Vec2::new(31.0, 466.0), Vec2::new(16.0, 541.0)),
        247.582
    );

    assert_delta!(
        aurora.path(Vec2::new(745.0, 670.0), Vec2::new(608.0, 684.0)),
        246.752
    );

    assert_delta!(
        aurora.path(Vec2::new(244.0, 185.0), Vec2::new(480.0, 186.0)),
        249.351
    );

    assert_delta!(
        aurora.path(Vec2::new(203.0, 511.0), Vec2::new(420.0, 553.0)),
        244.072
    );

    assert_delta!(
        aurora.path(Vec2::new(909.0, 490.0), Vec2::new(973.0, 280.0)),
        249.973
    );

    assert_delta!(
        aurora.path(Vec2::new(148.0, 469.0), Vec2::new(125.0, 240.0)),
        248.567
    );

    assert_delta!(
        aurora.path(Vec2::new(581.0, 293.0), Vec2::new(818.0, 338.0)),
        245.051
    );

    assert_delta!(
        aurora.path(Vec2::new(241.0, 333.0), Vec2::new(476.0, 328.0)),
        243.293
    );

    assert_delta!(
        aurora.path(Vec2::new(433.0, 657.0), Vec2::new(199.0, 663.0)),
        246.176
    );

    assert_delta!(
        aurora.path(Vec2::new(570.0, 33.0), Vec2::new(431.0, 142.0)),
        245.148
    );

    assert_delta!(
        aurora.path(Vec2::new(739.0, 514.0), Vec2::new(519.0, 621.0)),
        252.843
    );

    assert_delta!(
        aurora.path(Vec2::new(461.0, 126.0), Vec2::new(267.0, 258.0)),
        249.148
    );

    assert_delta!(
        aurora.path(Vec2::new(919.0, 153.0), Vec2::new(714.0, 170.0)),
        247.913
    );

    assert_delta!(
        aurora.path(Vec2::new(635.0, 187.0), Vec2::new(880.0, 147.0)),
        255.253
    );

    assert_delta!(
        aurora.path(Vec2::new(909.0, 465.0), Vec2::new(694.0, 559.0)),
        243.719
    );

    assert_delta!(
        aurora.path(Vec2::new(532.0, 147.0), Vec2::new(429.0, 225.0)),
        254.289
    );

    assert_delta!(
        aurora.path(Vec2::new(218.0, 602.0), Vec2::new(397.0, 583.0)),
        245.525
    );

    assert_delta!(
        aurora.path(Vec2::new(179.0, 122.0), Vec2::new(425.0, 172.0)),
        254.369
    );

    assert_delta!(
        aurora.path(Vec2::new(365.0, 691.0), Vec2::new(255.0, 538.0)),
        250.886
    );

    assert_delta!(
        aurora.path(Vec2::new(509.0, 454.0), Vec2::new(734.0, 400.0)),
        248.635
    );

    assert_delta!(
        aurora.path(Vec2::new(67.0, 330.0), Vec2::new(217.0, 179.0)),
        256.281
    );

    assert_delta!(
        aurora.path(Vec2::new(154.0, 675.0), Vec2::new(55.0, 496.0)),
        256.523
    );

    assert_delta!(
        aurora.path(Vec2::new(577.0, 480.0), Vec2::new(398.0, 595.0)),
        251.883
    );

    assert_delta!(
        aurora.path(Vec2::new(47.0, 450.0), Vec2::new(6.0, 554.0)),
        249.712
    );

    assert_delta!(
        aurora.path(Vec2::new(379.0, 526.0), Vec2::new(298.0, 498.0)),
        250.278
    );

    assert_delta!(
        aurora.path(Vec2::new(314.0, 215.0), Vec2::new(555.0, 143.0)),
        257.637
    );

    assert_delta!(
        aurora.path(Vec2::new(630.0, 253.0), Vec2::new(456.0, 400.0)),
        251.281
    );

    assert_delta!(
        aurora.path(Vec2::new(207.0, 419.0), Vec2::new(377.0, 604.0)),
        263.3
    );

    assert_delta!(
        aurora.path(Vec2::new(43.0, 578.0), Vec2::new(89.0, 446.0)),
        250.51
    );

    assert_delta!(
        aurora.path(Vec2::new(470.0, 51.0), Vec2::new(598.0, 165.0)),
        254.42
    );

    assert_delta!(
        aurora.path(Vec2::new(791.0, 188.0), Vec2::new(794.0, 338.0)),
        254.738
    );

    assert_delta!(
        aurora.path(Vec2::new(169.0, 131.0), Vec2::new(72.0, 293.0)),
        255.546
    );

    assert_delta!(
        aurora.path(Vec2::new(827.0, 418.0), Vec2::new(701.0, 442.0)),
        256.967
    );

    assert_delta!(
        aurora.path(Vec2::new(848.0, 279.0), Vec2::new(753.0, 232.0)),
        256.52
    );

    assert_delta!(
        aurora.path(Vec2::new(418.0, 547.0), Vec2::new(183.0, 487.0)),
        256.25
    );

    assert_delta!(
        aurora.path(Vec2::new(815.0, 406.0), Vec2::new(944.0, 547.0)),
        255.274
    );

    assert_delta!(
        aurora.path(Vec2::new(807.0, 198.0), Vec2::new(774.0, 396.0)),
        259.527
    );

    assert_delta!(
        aurora.path(Vec2::new(836.0, 209.0), Vec2::new(1014.0, 379.0)),
        256.155
    );

    assert_delta!(
        aurora.path(Vec2::new(734.0, 407.0), Vec2::new(524.0, 488.0)),
        257.078
    );

    assert_delta!(
        aurora.path(Vec2::new(238.0, 421.0), Vec2::new(237.0, 272.0)),
        255.65
    );

    assert_delta!(
        aurora.path(Vec2::new(497.0, 665.0), Vec2::new(285.0, 702.0)),
        263.069
    );

    assert_delta!(
        aurora.path(Vec2::new(364.0, 166.0), Vec2::new(621.0, 147.0)),
        264.982
    );

    assert_delta!(
        aurora.path(Vec2::new(650.0, 575.0), Vec2::new(746.0, 417.0)),
        261.928
    );

    assert_delta!(
        aurora.path(Vec2::new(259.0, 293.0), Vec2::new(169.0, 187.0)),
        256.412
    );

    assert_delta!(
        aurora.path(Vec2::new(49.0, 536.0), Vec2::new(115.0, 303.0)),
        264.878
    );

    assert_delta!(
        aurora.path(Vec2::new(883.0, 372.0), Vec2::new(880.0, 174.0)),
        259.556
    );

    assert_delta!(
        aurora.path(Vec2::new(866.0, 351.0), Vec2::new(832.0, 513.0)),
        261.386
    );

    assert_delta!(
        aurora.path(Vec2::new(102.0, 211.0), Vec2::new(359.0, 232.0)),
        269.746
    );

    assert_delta!(
        aurora.path(Vec2::new(822.0, 640.0), Vec2::new(746.0, 423.0)),
        266.339
    );

    assert_delta!(
        aurora.path(Vec2::new(684.0, 70.0), Vec2::new(827.0, 217.0)),
        261.986
    );

    assert_delta!(
        aurora.path(Vec2::new(419.0, 574.0), Vec2::new(646.0, 558.0)),
        267.048
    );

    assert_delta!(
        aurora.path(Vec2::new(220.0, 511.0), Vec2::new(1.0, 441.0)),
        264.169
    );

    assert_delta!(
        aurora.path(Vec2::new(787.0, 432.0), Vec2::new(850.0, 350.0)),
        268.436
    );

    assert_delta!(
        aurora.path(Vec2::new(451.0, 72.0), Vec2::new(603.0, 243.0)),
        267.156
    );

    assert_delta!(
        aurora.path(Vec2::new(916.0, 612.0), Vec2::new(662.0, 652.0)),
        266.9
    );

    assert_delta!(
        aurora.path(Vec2::new(843.0, 478.0), Vec2::new(609.0, 375.0)),
        262.484
    );

    assert_delta!(
        aurora.path(Vec2::new(391.0, 663.0), Vec2::new(138.0, 695.0)),
        266.84
    );

    assert_delta!(
        aurora.path(Vec2::new(512.0, 164.0), Vec2::new(269.0, 83.0)),
        260.84
    );

    assert_delta!(
        aurora.path(Vec2::new(80.0, 160.0), Vec2::new(82.0, 299.0)),
        264.576
    );

    assert_delta!(
        aurora.path(Vec2::new(280.0, 568.0), Vec2::new(407.0, 601.0)),
        259.138
    );

    assert_delta!(
        aurora.path(Vec2::new(767.0, 429.0), Vec2::new(889.0, 353.0)),
        274.702
    );

    assert_delta!(
        aurora.path(Vec2::new(362.0, 220.0), Vec2::new(125.0, 306.0)),
        263.792
    );

    assert_delta!(
        aurora.path(Vec2::new(764.0, 203.0), Vec2::new(940.0, 357.0)),
        269.994
    );

    assert_delta!(
        aurora.path(Vec2::new(220.0, 54.0), Vec2::new(398.0, 231.0)),
        263.402
    );

    assert_delta!(
        aurora.path(Vec2::new(67.0, 402.0), Vec2::new(313.0, 330.0)),
        266.703
    );

    assert_delta!(
        aurora.path(Vec2::new(149.0, 294.0), Vec2::new(408.0, 316.0)),
        266.903
    );

    assert_delta!(
        aurora.path(Vec2::new(725.0, 260.0), Vec2::new(906.0, 127.0)),
        261.839
    );

    assert_delta!(
        aurora.path(Vec2::new(99.0, 464.0), Vec2::new(28.0, 603.0)),
        269.578
    );

    assert_delta!(
        aurora.path(Vec2::new(242.0, 489.0), Vec2::new(492.0, 568.0)),
        269.65
    );

    assert_delta!(
        aurora.path(Vec2::new(525.0, 529.0), Vec2::new(614.0, 686.0)),
        267.8
    );

    assert_delta!(
        aurora.path(Vec2::new(789.0, 106.0), Vec2::new(535.0, 181.0)),
        268.19
    );

    assert_delta!(
        aurora.path(Vec2::new(325.0, 640.0), Vec2::new(60.0, 685.0)),
        276.914
    );

    assert_delta!(
        aurora.path(Vec2::new(443.0, 349.0), Vec2::new(694.0, 355.0)),
        271.022
    );

    assert_delta!(
        aurora.path(Vec2::new(817.0, 584.0), Vec2::new(557.0, 605.0)),
        274.758
    );

    assert_delta!(
        aurora.path(Vec2::new(72.0, 662.0), Vec2::new(17.0, 488.0)),
        275.481
    );

    assert_delta!(
        aurora.path(Vec2::new(411.0, 521.0), Vec2::new(662.0, 453.0)),
        273.995
    );

    assert_delta!(
        aurora.path(Vec2::new(322.0, 375.0), Vec2::new(91.0, 425.0)),
        273.026
    );

    assert_delta!(
        aurora.path(Vec2::new(220.0, 493.0), Vec2::new(318.0, 693.0)),
        272.764
    );

    assert_delta!(
        aurora.path(Vec2::new(78.0, 660.0), Vec2::new(21.0, 482.0)),
        275.712
    );

    assert_delta!(
        aurora.path(Vec2::new(247.0, 97.0), Vec2::new(179.0, 255.0)),
        269.23
    );

    assert_delta!(
        aurora.path(Vec2::new(294.0, 499.0), Vec2::new(37.0, 545.0)),
        272.027
    );

    assert_delta!(
        aurora.path(Vec2::new(468.0, 163.0), Vec2::new(704.0, 196.0)),
        275.26
    );

    assert_delta!(
        aurora.path(Vec2::new(546.0, 427.0), Vec2::new(760.0, 426.0)),
        272.081
    );

    assert_delta!(
        aurora.path(Vec2::new(940.0, 430.0), Vec2::new(853.0, 231.0)),
        278.698
    );

    assert_delta!(
        aurora.path(Vec2::new(414.0, 273.0), Vec2::new(157.0, 363.0)),
        275.684
    );

    assert_delta!(
        aurora.path(Vec2::new(410.0, 452.0), Vec2::new(449.0, 264.0)),
        278.757
    );

    assert_delta!(
        aurora.path(Vec2::new(250.0, 83.0), Vec2::new(115.0, 275.0)),
        275.379
    );

    assert_delta!(
        aurora.path(Vec2::new(32.0, 653.0), Vec2::new(264.0, 500.0)),
        277.908
    );

    assert_delta!(
        aurora.path(Vec2::new(196.0, 69.0), Vec2::new(348.0, 272.0)),
        271.375
    );

    assert_delta!(
        aurora.path(Vec2::new(704.0, 282.0), Vec2::new(792.0, 264.0)),
        270.881
    );

    assert_delta!(
        aurora.path(Vec2::new(329.0, 351.0), Vec2::new(528.0, 545.0)),
        287.643
    );

    assert_delta!(
        aurora.path(Vec2::new(373.0, 726.0), Vec2::new(277.0, 602.0)),
        278.563
    );

    assert_delta!(
        aurora.path(Vec2::new(565.0, 607.0), Vec2::new(706.0, 396.0)),
        282.344
    );

    assert_delta!(
        aurora.path(Vec2::new(140.0, 637.0), Vec2::new(174.0, 453.0)),
        281.129
    );

    assert_delta!(
        aurora.path(Vec2::new(174.0, 592.0), Vec2::new(443.0, 648.0)),
        277.524
    );

    assert_delta!(
        aurora.path(Vec2::new(774.0, 503.0), Vec2::new(624.0, 721.0)),
        283.334
    );

    assert_delta!(
        aurora.path(Vec2::new(393.0, 449.0), Vec2::new(149.0, 575.0)),
        274.612
    );

    assert_delta!(
        aurora.path(Vec2::new(349.0, 145.0), Vec2::new(482.0, 274.0)),
        277.004
    );

    assert_delta!(
        aurora.path(Vec2::new(712.0, 315.0), Vec2::new(435.0, 299.0)),
        281.968
    );

    assert_delta!(
        aurora.path(Vec2::new(606.0, 170.0), Vec2::new(862.0, 174.0)),
        281.659
    );

    assert_delta!(
        aurora.path(Vec2::new(808.0, 270.0), Vec2::new(1012.0, 377.0)),
        283.56
    );

    assert_delta!(
        aurora.path(Vec2::new(363.0, 237.0), Vec2::new(295.0, 368.0)),
        282.696
    );

    assert_delta!(
        aurora.path(Vec2::new(140.0, 137.0), Vec2::new(205.0, 302.0)),
        282.889
    );

    assert_delta!(
        aurora.path(Vec2::new(868.0, 299.0), Vec2::new(700.0, 420.0)),
        281.845
    );

    assert_delta!(
        aurora.path(Vec2::new(775.0, 420.0), Vec2::new(967.0, 347.0)),
        286.67
    );

    assert_delta!(
        aurora.path(Vec2::new(794.0, 179.0), Vec2::new(878.0, 348.0)),
        279.989
    );

    assert_delta!(
        aurora.path(Vec2::new(194.0, 310.0), Vec2::new(183.0, 109.0)),
        289.481
    );

    assert_delta!(
        aurora.path(Vec2::new(844.0, 199.0), Vec2::new(677.0, 369.0)),
        284.239
    );

    assert_delta!(
        aurora.path(Vec2::new(686.0, 382.0), Vec2::new(505.0, 448.0)),
        280.895
    );

    assert_delta!(
        aurora.path(Vec2::new(743.0, 280.0), Vec2::new(783.0, 355.0)),
        283.718
    );

    assert_delta!(
        aurora.path(Vec2::new(62.0, 617.0), Vec2::new(324.0, 550.0)),
        284.318
    );

    assert_delta!(
        aurora.path(Vec2::new(213.0, 346.0), Vec2::new(361.0, 202.0)),
        286.119
    );

    assert_delta!(
        aurora.path(Vec2::new(96.0, 597.0), Vec2::new(145.0, 421.0)),
        291.28
    );

    assert_delta!(
        aurora.path(Vec2::new(570.0, 343.0), Vec2::new(573.0, 132.0)),
        289.533
    );

    assert_delta!(
        aurora.path(Vec2::new(497.0, 158.0), Vec2::new(762.0, 259.0)),
        285.367
    );

    assert_delta!(
        aurora.path(Vec2::new(838.0, 248.0), Vec2::new(625.0, 95.0)),
        283.857
    );

    assert_delta!(
        aurora.path(Vec2::new(113.0, 478.0), Vec2::new(110.0, 220.0)),
        287.128
    );

    assert_delta!(
        aurora.path(Vec2::new(46.0, 518.0), Vec2::new(176.0, 266.0)),
        289.133
    );

    assert_delta!(
        aurora.path(Vec2::new(655.0, 636.0), Vec2::new(458.0, 493.0)),
        284.045
    );

    assert_delta!(
        aurora.path(Vec2::new(648.0, 585.0), Vec2::new(900.0, 479.0)),
        282.311
    );

    assert_delta!(
        aurora.path(Vec2::new(385.0, 339.0), Vec2::new(472.0, 129.0)),
        294.848
    );

    assert_delta!(
        aurora.path(Vec2::new(629.0, 239.0), Vec2::new(347.0, 210.0)),
        290.739
    );

    assert_delta!(
        aurora.path(Vec2::new(259.0, 627.0), Vec2::new(27.0, 487.0)),
        290.22
    );

    assert_delta!(
        aurora.path(Vec2::new(808.0, 343.0), Vec2::new(553.0, 211.0)),
        287.153
    );

    assert_delta!(
        aurora.path(Vec2::new(477.0, 124.0), Vec2::new(454.0, 361.0)),
        295.356
    );

    assert_delta!(
        aurora.path(Vec2::new(233.0, 106.0), Vec2::new(492.0, 174.0)),
        290.251
    );

    assert_delta!(
        aurora.path(Vec2::new(441.0, 141.0), Vec2::new(552.0, 346.0)),
        289.594
    );

    assert_delta!(
        aurora.path(Vec2::new(287.0, 486.0), Vec2::new(281.0, 715.0)),
        295.692
    );

    assert_delta!(
        aurora.path(Vec2::new(275.0, 25.0), Vec2::new(111.0, 230.0)),
        291.598
    );

    assert_delta!(
        aurora.path(Vec2::new(724.0, 529.0), Vec2::new(867.0, 420.0)),
        285.903
    );

    assert_delta!(
        aurora.path(Vec2::new(668.0, 353.0), Vec2::new(823.0, 260.0)),
        288.852
    );

    assert_delta!(
        aurora.path(Vec2::new(39.0, 164.0), Vec2::new(13.0, 327.0)),
        291.061
    );

    assert_delta!(
        aurora.path(Vec2::new(496.0, 591.0), Vec2::new(355.0, 623.0)),
        297.477
    );

    assert_delta!(
        aurora.path(Vec2::new(228.0, 384.0), Vec2::new(349.0, 210.0)),
        296.628
    );

    assert_delta!(
        aurora.path(Vec2::new(878.0, 576.0), Vec2::new(636.0, 695.0)),
        291.473
    );

    assert_delta!(
        aurora.path(Vec2::new(749.0, 705.0), Vec2::new(571.0, 700.0)),
        295.334
    );

    assert_delta!(
        aurora.path(Vec2::new(270.0, 69.0), Vec2::new(17.0, 193.0)),
        291.812
    );

    assert_delta!(
        aurora.path(Vec2::new(230.0, 356.0), Vec2::new(281.0, 200.0)),
        296.23
    );

    assert_delta!(
        aurora.path(Vec2::new(139.0, 490.0), Vec2::new(309.0, 618.0)),
        296.522
    );

    assert_delta!(
        aurora.path(Vec2::new(505.0, 149.0), Vec2::new(449.0, 383.0)),
        298.154
    );

    assert_delta!(
        aurora.path(Vec2::new(51.0, 299.0), Vec2::new(341.0, 313.0)),
        298.338
    );

    assert_delta!(
        aurora.path(Vec2::new(267.0, 376.0), Vec2::new(523.0, 263.0)),
        293.847
    );

    assert_delta!(
        aurora.path(Vec2::new(392.0, 627.0), Vec2::new(563.0, 550.0)),
        297.653
    );

    assert_delta!(
        aurora.path(Vec2::new(132.0, 100.0), Vec2::new(405.0, 158.0)),
        294.013
    );

    assert_delta!(
        aurora.path(Vec2::new(660.0, 196.0), Vec2::new(872.0, 289.0)),
        299.148
    );

    assert_delta!(
        aurora.path(Vec2::new(335.0, 323.0), Vec2::new(510.0, 139.0)),
        297.536
    );

    assert_delta!(
        aurora.path(Vec2::new(77.0, 375.0), Vec2::new(337.0, 265.0)),
        298.86
    );

    assert_delta!(
        aurora.path(Vec2::new(698.0, 615.0), Vec2::new(727.0, 409.0)),
        304.138
    );

    assert_delta!(
        aurora.path(Vec2::new(458.0, 194.0), Vec2::new(166.0, 132.0)),
        299.568
    );

    assert_delta!(
        aurora.path(Vec2::new(631.0, 464.0), Vec2::new(834.0, 614.0)),
        299.631
    );

    assert_delta!(
        aurora.path(Vec2::new(86.0, 565.0), Vec2::new(32.0, 428.0)),
        298.591
    );

    assert_delta!(
        aurora.path(Vec2::new(376.0, 719.0), Vec2::new(233.0, 646.0)),
        302.246
    );

    assert_delta!(
        aurora.path(Vec2::new(335.0, 611.0), Vec2::new(138.0, 483.0)),
        302.391
    );

    assert_delta!(
        aurora.path(Vec2::new(701.0, 223.0), Vec2::new(424.0, 171.0)),
        298.38
    );

    assert_delta!(
        aurora.path(Vec2::new(405.0, 134.0), Vec2::new(115.0, 177.0)),
        306.041
    );

    assert_delta!(
        aurora.path(Vec2::new(660.0, 614.0), Vec2::new(946.0, 578.0)),
        300.536
    );

    assert_delta!(
        aurora.path(Vec2::new(216.0, 305.0), Vec2::new(427.0, 485.0)),
        305.795
    );

    assert_delta!(
        aurora.path(Vec2::new(448.0, 395.0), Vec2::new(420.0, 463.0)),
        309.479
    );

    assert_delta!(
        aurora.path(Vec2::new(672.0, 103.0), Vec2::new(859.0, 183.0)),
        303.844
    );

    assert_delta!(
        aurora.path(Vec2::new(466.0, 531.0), Vec2::new(257.0, 578.0)),
        299.773
    );

    assert_delta!(
        aurora.path(Vec2::new(228.0, 97.0), Vec2::new(179.0, 311.0)),
        303.671
    );

    assert_delta!(
        aurora.path(Vec2::new(543.0, 277.0), Vec2::new(637.0, 96.0)),
        301.05
    );

    assert_delta!(
        aurora.path(Vec2::new(896.0, 175.0), Vec2::new(693.0, 318.0)),
        306.758
    );

    assert_delta!(
        aurora.path(Vec2::new(525.0, 654.0), Vec2::new(311.0, 541.0)),
        304.721
    );

    assert_delta!(
        aurora.path(Vec2::new(470.0, 462.0), Vec2::new(700.0, 615.0)),
        303.386
    );

    assert_delta!(
        aurora.path(Vec2::new(784.0, 734.0), Vec2::new(619.0, 531.0)),
        308.02
    );

    assert_delta!(
        aurora.path(Vec2::new(646.0, 412.0), Vec2::new(892.0, 285.0)),
        300.29
    );

    assert_delta!(
        aurora.path(Vec2::new(376.0, 192.0), Vec2::new(588.0, 27.0)),
        306.702
    );

    assert_delta!(
        aurora.path(Vec2::new(223.0, 151.0), Vec2::new(196.0, 360.0)),
        304.762
    );

    assert_delta!(
        aurora.path(Vec2::new(246.0, 586.0), Vec2::new(455.0, 560.0)),
        304.17
    );

    assert_delta!(
        aurora.path(Vec2::new(445.0, 69.0), Vec2::new(645.0, 260.0)),
        309.947
    );

    assert_delta!(
        aurora.path(Vec2::new(628.0, 501.0), Vec2::new(647.0, 347.0)),
        307.523
    );

    assert_delta!(
        aurora.path(Vec2::new(903.0, 353.0), Vec2::new(711.0, 438.0)),
        311.782
    );

    assert_delta!(
        aurora.path(Vec2::new(62.0, 552.0), Vec2::new(290.0, 664.0)),
        308.318
    );

    assert_delta!(
        aurora.path(Vec2::new(491.0, 536.0), Vec2::new(798.0, 546.0)),
        313.495
    );

    assert_delta!(
        aurora.path(Vec2::new(142.0, 142.0), Vec2::new(452.0, 190.0)),
        314.74
    );

    assert_delta!(
        aurora.path(Vec2::new(698.0, 188.0), Vec2::new(470.0, 260.0)),
        312.797
    );

    assert_delta!(
        aurora.path(Vec2::new(197.0, 84.0), Vec2::new(451.0, 235.0)),
        311.626
    );

    assert_delta!(
        aurora.path(Vec2::new(398.0, 245.0), Vec2::new(97.0, 178.0)),
        312.181
    );

    assert_delta!(
        aurora.path(Vec2::new(721.0, 370.0), Vec2::new(509.0, 231.0)),
        305.99
    );

    assert_delta!(
        aurora.path(Vec2::new(129.0, 670.0), Vec2::new(73.0, 448.0)),
        312.338
    );

    assert_delta!(
        aurora.path(Vec2::new(778.0, 524.0), Vec2::new(516.0, 659.0)),
        317.78
    );

    assert_delta!(
        aurora.path(Vec2::new(259.0, 78.0), Vec2::new(195.0, 271.0)),
        313.106
    );

    assert_delta!(
        aurora.path(Vec2::new(255.0, 572.0), Vec2::new(66.0, 395.0)),
        318.313
    );

    assert_delta!(
        aurora.path(Vec2::new(767.0, 610.0), Vec2::new(483.0, 508.0)),
        315.955
    );

    assert_delta!(
        aurora.path(Vec2::new(63.0, 333.0), Vec2::new(286.0, 504.0)),
        316.434
    );

    assert_delta!(
        aurora.path(Vec2::new(346.0, 263.0), Vec2::new(45.0, 335.0)),
        313.267
    );

    assert_delta!(
        aurora.path(Vec2::new(336.0, 660.0), Vec2::new(174.0, 498.0)),
        314.05
    );

    assert_delta!(
        aurora.path(Vec2::new(696.0, 248.0), Vec2::new(440.0, 164.0)),
        309.947
    );

    assert_delta!(
        aurora.path(Vec2::new(212.0, 225.0), Vec2::new(509.0, 153.0)),
        311.711
    );

    assert_delta!(
        aurora.path(Vec2::new(486.0, 397.0), Vec2::new(565.0, 167.0)),
        313.135
    );

    assert_delta!(
        aurora.path(Vec2::new(380.0, 307.0), Vec2::new(258.0, 509.0)),
        318.701
    );

    assert_delta!(
        aurora.path(Vec2::new(141.0, 366.0), Vec2::new(337.0, 174.0)),
        314.686
    );

    assert_delta!(
        aurora.path(Vec2::new(428.0, 265.0), Vec2::new(329.0, 220.0)),
        316.204
    );

    assert_delta!(
        aurora.path(Vec2::new(716.0, 345.0), Vec2::new(494.0, 166.0)),
        316.741
    );

    assert_delta!(
        aurora.path(Vec2::new(193.0, 100.0), Vec2::new(501.0, 56.0)),
        315.439
    );

    assert_delta!(
        aurora.path(Vec2::new(662.0, 447.0), Vec2::new(711.0, 697.0)),
        321.433
    );

    assert_delta!(
        aurora.path(Vec2::new(207.0, 436.0), Vec2::new(378.0, 640.0)),
        318.014
    );

    assert_delta!(
        aurora.path(Vec2::new(284.0, 553.0), Vec2::new(599.0, 539.0)),
        323.287
    );

    assert_delta!(
        aurora.path(Vec2::new(448.0, 244.0), Vec2::new(559.0, 18.0)),
        315.571
    );

    assert_delta!(
        aurora.path(Vec2::new(716.0, 363.0), Vec2::new(420.0, 347.0)),
        317.424
    );

    assert_delta!(
        aurora.path(Vec2::new(558.0, 678.0), Vec2::new(243.0, 660.0)),
        323.434
    );

    assert_delta!(
        aurora.path(Vec2::new(59.0, 514.0), Vec2::new(233.0, 298.0)),
        319.251
    );

    assert_delta!(
        aurora.path(Vec2::new(117.0, 393.0), Vec2::new(3.0, 174.0)),
        319.349
    );

    assert_delta!(
        aurora.path(Vec2::new(550.0, 451.0), Vec2::new(807.0, 422.0)),
        320.221
    );

    assert_delta!(
        aurora.path(Vec2::new(737.0, 683.0), Vec2::new(583.0, 533.0)),
        325.535
    );

    assert_delta!(
        aurora.path(Vec2::new(235.0, 561.0), Vec2::new(394.0, 546.0)),
        317.456
    );

    assert_delta!(
        aurora.path(Vec2::new(376.0, 330.0), Vec2::new(294.0, 218.0)),
        317.698
    );

    assert_delta!(
        aurora.path(Vec2::new(324.0, 343.0), Vec2::new(392.0, 569.0)),
        327.473
    );

    assert_delta!(
        aurora.path(Vec2::new(283.0, 537.0), Vec2::new(597.0, 541.0)),
        324.725
    );

    assert_delta!(
        aurora.path(Vec2::new(969.0, 274.0), Vec2::new(814.0, 469.0)),
        331.225
    );

    assert_delta!(
        aurora.path(Vec2::new(598.0, 474.0), Vec2::new(791.0, 322.0)),
        321.992
    );

    assert_delta!(
        aurora.path(Vec2::new(461.0, 601.0), Vec2::new(346.0, 720.0)),
        327.48
    );

    assert_delta!(
        aurora.path(Vec2::new(101.0, 670.0), Vec2::new(349.0, 581.0)),
        325.527
    );

    assert_delta!(
        aurora.path(Vec2::new(93.0, 696.0), Vec2::new(318.0, 528.0)),
        325.764
    );

    assert_delta!(
        aurora.path(Vec2::new(256.0, 703.0), Vec2::new(28.0, 595.0)),
        330.379
    );

    assert_delta!(
        aurora.path(Vec2::new(539.0, 686.0), Vec2::new(246.0, 558.0)),
        323.091
    );

    assert_delta!(
        aurora.path(Vec2::new(648.0, 161.0), Vec2::new(334.0, 136.0)),
        324.917
    );

    assert_delta!(
        aurora.path(Vec2::new(925.0, 344.0), Vec2::new(619.0, 328.0)),
        326.091
    );

    assert_delta!(
        aurora.path(Vec2::new(143.0, 438.0), Vec2::new(431.0, 326.0)),
        323.525
    );

    assert_delta!(
        aurora.path(Vec2::new(757.0, 114.0), Vec2::new(899.0, 287.0)),
        328.351
    );

    assert_delta!(
        aurora.path(Vec2::new(22.0, 158.0), Vec2::new(308.0, 292.0)),
        328.302
    );

    assert_delta!(
        aurora.path(Vec2::new(128.0, 550.0), Vec2::new(117.0, 323.0)),
        325.882
    );

    assert_delta!(
        aurora.path(Vec2::new(362.0, 78.0), Vec2::new(669.0, 75.0)),
        323.802
    );

    assert_delta!(
        aurora.path(Vec2::new(415.0, 409.0), Vec2::new(316.0, 312.0)),
        327.931
    );

    assert_delta!(
        aurora.path(Vec2::new(591.0, 502.0), Vec2::new(845.0, 706.0)),
        329.833
    );

    assert_delta!(
        aurora.path(Vec2::new(685.0, 530.0), Vec2::new(725.0, 340.0)),
        329.019
    );

    assert_delta!(
        aurora.path(Vec2::new(371.0, 566.0), Vec2::new(53.0, 509.0)),
        329.439
    );

    assert_delta!(
        aurora.path(Vec2::new(74.0, 399.0), Vec2::new(182.0, 155.0)),
        331.16
    );

    assert_delta!(
        aurora.path(Vec2::new(655.0, 587.0), Vec2::new(941.0, 461.0)),
        325.09
    );

    assert_delta!(
        aurora.path(Vec2::new(286.0, 360.0), Vec2::new(184.0, 171.0)),
        331.889
    );

    assert_delta!(
        aurora.path(Vec2::new(58.0, 133.0), Vec2::new(147.0, 386.0)),
        334.082
    );

    assert_delta!(
        aurora.path(Vec2::new(857.0, 650.0), Vec2::new(573.0, 491.0)),
        330.302
    );

    assert_delta!(
        aurora.path(Vec2::new(206.0, 102.0), Vec2::new(208.0, 335.0)),
        331.32
    );

    assert_delta!(
        aurora.path(Vec2::new(380.0, 564.0), Vec2::new(98.0, 535.0)),
        329.336
    );

    assert_delta!(
        aurora.path(Vec2::new(507.0, 311.0), Vec2::new(653.0, 96.0)),
        330.127
    );

    assert_delta!(
        aurora.path(Vec2::new(113.0, 463.0), Vec2::new(398.0, 333.0)),
        333.927
    );

    assert_delta!(
        aurora.path(Vec2::new(233.0, 184.0), Vec2::new(329.0, 339.0)),
        328.142
    );

    assert_delta!(
        aurora.path(Vec2::new(19.0, 505.0), Vec2::new(318.0, 371.0)),
        333.661
    );

    assert_delta!(
        aurora.path(Vec2::new(1015.0, 445.0), Vec2::new(693.0, 443.0)),
        332.103
    );

    assert_delta!(
        aurora.path(Vec2::new(374.0, 334.0), Vec2::new(198.0, 521.0)),
        332.168
    );

    assert_delta!(
        aurora.path(Vec2::new(284.0, 248.0), Vec2::new(493.0, 329.0)),
        335.657
    );

    assert_delta!(
        aurora.path(Vec2::new(593.0, 661.0), Vec2::new(279.0, 574.0)),
        335.327
    );

    assert_delta!(
        aurora.path(Vec2::new(862.0, 397.0), Vec2::new(545.0, 383.0)),
        331.306
    );

    assert_delta!(
        aurora.path(Vec2::new(401.0, 543.0), Vec2::new(174.0, 534.0)),
        335.188
    );

    assert_delta!(
        aurora.path(Vec2::new(652.0, 449.0), Vec2::new(460.0, 680.0)),
        337.743
    );

    assert_delta!(
        aurora.path(Vec2::new(749.0, 417.0), Vec2::new(895.0, 235.0)),
        333.643
    );

    assert_delta!(
        aurora.path(Vec2::new(664.0, 239.0), Vec2::new(434.0, 274.0)),
        332.787
    );

    assert_delta!(
        aurora.path(Vec2::new(468.0, 577.0), Vec2::new(789.0, 489.0)),
        335.683
    );

    assert_delta!(
        aurora.path(Vec2::new(616.0, 62.0), Vec2::new(306.0, 51.0)),
        330.974
    );

    assert_delta!(
        aurora.path(Vec2::new(696.0, 110.0), Vec2::new(826.0, 280.0)),
        341.174
    );

    assert_delta!(
        aurora.path(Vec2::new(73.0, 136.0), Vec2::new(164.0, 386.0)),
        341.101
    );

    assert_delta!(
        aurora.path(Vec2::new(477.0, 590.0), Vec2::new(335.0, 752.0)),
        337.186
    );

    assert_delta!(
        aurora.path(Vec2::new(438.0, 499.0), Vec2::new(632.0, 671.0)),
        336.215
    );

    assert_delta!(
        aurora.path(Vec2::new(253.0, 339.0), Vec2::new(572.0, 247.0)),
        336.527
    );

    assert_delta!(
        aurora.path(Vec2::new(512.0, 412.0), Vec2::new(417.0, 188.0)),
        337.467
    );

    assert_delta!(
        aurora.path(Vec2::new(433.0, 197.0), Vec2::new(132.0, 319.0)),
        338.677
    );

    assert_delta!(
        aurora.path(Vec2::new(420.0, 317.0), Vec2::new(205.0, 504.0)),
        335.174
    );

    assert_delta!(
        aurora.path(Vec2::new(427.0, 460.0), Vec2::new(288.0, 265.0)),
        340.82
    );

    assert_delta!(
        aurora.path(Vec2::new(502.0, 497.0), Vec2::new(181.0, 462.0)),
        339.457
    );

    assert_delta!(
        aurora.path(Vec2::new(808.0, 470.0), Vec2::new(483.0, 521.0)),
        342.358
    );

    assert_delta!(
        aurora.path(Vec2::new(406.0, 464.0), Vec2::new(95.0, 454.0)),
        340.502
    );

    assert_delta!(
        aurora.path(Vec2::new(343.0, 254.0), Vec2::new(566.0, 263.0)),
        346.188
    );

    assert_delta!(
        aurora.path(Vec2::new(903.0, 466.0), Vec2::new(601.0, 604.0)),
        338.366
    );

    assert_delta!(
        aurora.path(Vec2::new(554.0, 575.0), Vec2::new(866.0, 442.0)),
        343.438
    );

    assert_delta!(
        aurora.path(Vec2::new(767.0, 303.0), Vec2::new(473.0, 143.0)),
        340.131
    );

    assert_delta!(
        aurora.path(Vec2::new(277.0, 318.0), Vec2::new(13.0, 529.0)),
        348.338
    );

    assert_delta!(
        aurora.path(Vec2::new(752.0, 511.0), Vec2::new(639.0, 359.0)),
        339.859
    );

    assert_delta!(
        aurora.path(Vec2::new(702.0, 215.0), Vec2::new(790.0, 391.0)),
        346.261
    );

    assert_delta!(
        aurora.path(Vec2::new(539.0, 151.0), Vec2::new(764.0, 326.0)),
        339.301
    );

    assert_delta!(
        aurora.path(Vec2::new(878.0, 118.0), Vec2::new(774.0, 301.0)),
        348.095
    );

    assert_delta!(
        aurora.path(Vec2::new(341.0, 605.0), Vec2::new(34.0, 572.0)),
        345.946
    );

    assert_delta!(
        aurora.path(Vec2::new(464.0, 361.0), Vec2::new(463.0, 483.0)),
        345.542
    );

    assert_delta!(
        aurora.path(Vec2::new(17.0, 156.0), Vec2::new(316.0, 308.0)),
        347.137
    );

    assert_delta!(
        aurora.path(Vec2::new(100.0, 274.0), Vec2::new(431.0, 299.0)),
        346.763
    );

    assert_delta!(
        aurora.path(Vec2::new(607.0, 216.0), Vec2::new(453.0, 354.0)),
        351.466
    );

    assert_delta!(
        aurora.path(Vec2::new(305.0, 89.0), Vec2::new(268.0, 342.0)),
        351.094
    );

    assert_delta!(
        aurora.path(Vec2::new(763.0, 492.0), Vec2::new(442.0, 462.0)),
        344.025
    );

    assert_delta!(
        aurora.path(Vec2::new(950.0, 365.0), Vec2::new(640.0, 428.0)),
        357.351
    );

    assert_delta!(
        aurora.path(Vec2::new(459.0, 669.0), Vec2::new(794.0, 658.0)),
        347.926
    );

    assert_delta!(
        aurora.path(Vec2::new(776.0, 592.0), Vec2::new(715.0, 357.0)),
        348.08
    );

    assert_delta!(
        aurora.path(Vec2::new(876.0, 172.0), Vec2::new(647.0, 366.0)),
        349.325
    );

    assert_delta!(
        aurora.path(Vec2::new(579.0, 360.0), Vec2::new(604.0, 540.0)),
        348.051
    );

    assert_delta!(
        aurora.path(Vec2::new(98.0, 307.0), Vec2::new(142.0, 93.0)),
        353.399
    );

    assert_delta!(
        aurora.path(Vec2::new(453.0, 229.0), Vec2::new(761.0, 248.0)),
        349.701
    );

    assert_delta!(
        aurora.path(Vec2::new(52.0, 178.0), Vec2::new(389.0, 126.0)),
        349.761
    );

    assert_delta!(
        aurora.path(Vec2::new(425.0, 530.0), Vec2::new(159.0, 586.0)),
        345.464
    );

    assert_delta!(
        aurora.path(Vec2::new(197.0, 275.0), Vec2::new(463.0, 490.0)),
        354.4
    );

    assert_delta!(
        aurora.path(Vec2::new(634.0, 327.0), Vec2::new(389.0, 133.0)),
        355.084
    );

    assert_delta!(
        aurora.path(Vec2::new(131.0, 159.0), Vec2::new(112.0, 394.0)),
        350.785
    );

    assert_delta!(
        aurora.path(Vec2::new(416.0, 525.0), Vec2::new(150.0, 462.0)),
        350.559
    );

    assert_delta!(
        aurora.path(Vec2::new(669.0, 377.0), Vec2::new(633.0, 624.0)),
        356.228
    );

    assert_delta!(
        aurora.path(Vec2::new(371.0, 570.0), Vec2::new(128.0, 710.0)),
        354.963
    );

    assert_delta!(
        aurora.path(Vec2::new(402.0, 309.0), Vec2::new(97.0, 465.0)),
        354.542
    );

    assert_delta!(
        aurora.path(Vec2::new(380.0, 83.0), Vec2::new(514.0, 351.0)),
        357.85
    );

    assert_delta!(
        aurora.path(Vec2::new(370.0, 162.0), Vec2::new(112.0, 360.0)),
        354.594
    );

    assert_delta!(
        aurora.path(Vec2::new(8.0, 649.0), Vec2::new(331.0, 524.0)),
        353.463
    );

    assert_delta!(
        aurora.path(Vec2::new(224.0, 240.0), Vec2::new(115.0, 430.0)),
        350.03
    );

    assert_delta!(
        aurora.path(Vec2::new(771.0, 202.0), Vec2::new(433.0, 190.0)),
        357.452
    );

    assert_delta!(
        aurora.path(Vec2::new(837.0, 330.0), Vec2::new(488.0, 272.0)),
        358.773
    );

    assert_delta!(
        aurora.path(Vec2::new(248.0, 269.0), Vec2::new(120.0, 121.0)),
        352.39
    );

    assert_delta!(
        aurora.path(Vec2::new(561.0, 659.0), Vec2::new(250.0, 486.0)),
        356.431
    );

    assert_delta!(
        aurora.path(Vec2::new(604.0, 690.0), Vec2::new(279.0, 568.0)),
        355.942
    );

    assert_delta!(
        aurora.path(Vec2::new(333.0, 632.0), Vec2::new(50.0, 477.0)),
        355.498
    );

    assert_delta!(
        aurora.path(Vec2::new(638.0, 259.0), Vec2::new(326.0, 101.0)),
        357.855
    );

    assert_delta!(
        aurora.path(Vec2::new(535.0, 248.0), Vec2::new(725.0, 169.0)),
        365.636
    );

    assert_delta!(
        aurora.path(Vec2::new(560.0, 612.0), Vec2::new(648.0, 358.0)),
        358.285
    );

    assert_delta!(
        aurora.path(Vec2::new(321.0, 732.0), Vec2::new(320.0, 551.0)),
        357.478
    );

    assert_delta!(
        aurora.path(Vec2::new(119.0, 387.0), Vec2::new(279.0, 633.0)),
        359.545
    );

    assert_delta!(
        aurora.path(Vec2::new(696.0, 68.0), Vec2::new(361.0, 110.0)),
        356.232
    );

    assert_delta!(
        aurora.path(Vec2::new(877.0, 410.0), Vec2::new(715.0, 631.0)),
        363.353
    );

    assert_delta!(
        aurora.path(Vec2::new(529.0, 547.0), Vec2::new(194.0, 496.0)),
        358.741
    );

    assert_delta!(
        aurora.path(Vec2::new(822.0, 431.0), Vec2::new(734.0, 208.0)),
        363.901
    );

    assert_delta!(
        aurora.path(Vec2::new(825.0, 387.0), Vec2::new(700.0, 258.0)),
        361.149
    );

    assert_delta!(
        aurora.path(Vec2::new(77.0, 210.0), Vec2::new(447.0, 212.0)),
        373.266
    );

    assert_delta!(
        aurora.path(Vec2::new(907.0, 431.0), Vec2::new(632.0, 599.0)),
        361.269
    );

    assert_delta!(
        aurora.path(Vec2::new(602.0, 108.0), Vec2::new(690.0, 324.0)),
        361.726
    );

    assert_delta!(
        aurora.path(Vec2::new(179.0, 623.0), Vec2::new(320.0, 371.0)),
        364.53
    );

    assert_delta!(
        aurora.path(Vec2::new(790.0, 660.0), Vec2::new(446.0, 585.0)),
        361.427
    );

    assert_delta!(
        aurora.path(Vec2::new(393.0, 281.0), Vec2::new(369.0, 545.0)),
        365.689
    );

    assert_delta!(
        aurora.path(Vec2::new(292.0, 552.0), Vec2::new(278.0, 303.0)),
        374.587
    );

    assert_delta!(
        aurora.path(Vec2::new(484.0, 437.0), Vec2::new(614.0, 685.0)),
        369.36
    );

    assert_delta!(
        aurora.path(Vec2::new(937.0, 378.0), Vec2::new(725.0, 156.0)),
        365.721
    );

    assert_delta!(
        aurora.path(Vec2::new(499.0, 497.0), Vec2::new(200.0, 350.0)),
        368.583
    );

    assert_delta!(
        aurora.path(Vec2::new(580.0, 112.0), Vec2::new(262.0, 263.0)),
        368.233
    );

    assert_delta!(
        aurora.path(Vec2::new(450.0, 580.0), Vec2::new(156.0, 439.0)),
        360.804
    );

    assert_delta!(
        aurora.path(Vec2::new(400.0, 335.0), Vec2::new(47.0, 306.0)),
        368.488
    );

    assert_delta!(
        aurora.path(Vec2::new(529.0, 188.0), Vec2::new(832.0, 220.0)),
        364.854
    );

    assert_delta!(
        aurora.path(Vec2::new(161.0, 373.0), Vec2::new(167.0, 634.0)),
        375.213
    );

    assert_delta!(
        aurora.path(Vec2::new(473.0, 600.0), Vec2::new(251.0, 632.0)),
        366.017
    );

    assert_delta!(
        aurora.path(Vec2::new(118.0, 430.0), Vec2::new(251.0, 211.0)),
        368.427
    );

    assert_delta!(
        aurora.path(Vec2::new(430.0, 137.0), Vec2::new(272.0, 368.0)),
        369.794
    );

    assert_delta!(
        aurora.path(Vec2::new(590.0, 516.0), Vec2::new(935.0, 633.0)),
        369.327
    );

    assert_delta!(
        aurora.path(Vec2::new(338.0, 374.0), Vec2::new(5.0, 413.0)),
        368.801
    );

    assert_delta!(
        aurora.path(Vec2::new(621.0, 479.0), Vec2::new(875.0, 360.0)),
        374.571
    );

    assert_delta!(
        aurora.path(Vec2::new(482.0, 271.0), Vec2::new(240.0, 226.0)),
        376.014
    );

    assert_delta!(
        aurora.path(Vec2::new(613.0, 200.0), Vec2::new(261.0, 188.0)),
        375.463
    );

    assert_delta!(
        aurora.path(Vec2::new(2.0, 362.0), Vec2::new(119.0, 171.0)),
        366.018
    );

    assert_delta!(
        aurora.path(Vec2::new(793.0, 387.0), Vec2::new(807.0, 594.0)),
        369.769
    );

    assert_delta!(
        aurora.path(Vec2::new(276.0, 275.0), Vec2::new(535.0, 287.0)),
        367.206
    );

    assert_delta!(
        aurora.path(Vec2::new(483.0, 367.0), Vec2::new(314.0, 303.0)),
        368.626
    );

    assert_delta!(
        aurora.path(Vec2::new(767.0, 729.0), Vec2::new(437.0, 667.0)),
        373.482
    );

    assert_delta!(
        aurora.path(Vec2::new(871.0, 361.0), Vec2::new(522.0, 298.0)),
        377.479
    );

    assert_delta!(
        aurora.path(Vec2::new(121.0, 645.0), Vec2::new(44.0, 396.0)),
        374.455
    );

    assert_delta!(
        aurora.path(Vec2::new(801.0, 328.0), Vec2::new(446.0, 400.0)),
        372.177
    );

    assert_delta!(
        aurora.path(Vec2::new(540.0, 460.0), Vec2::new(785.0, 332.0)),
        369.9
    );

    assert_delta!(
        aurora.path(Vec2::new(894.0, 620.0), Vec2::new(555.0, 477.0)),
        370.948
    );

    assert_delta!(
        aurora.path(Vec2::new(653.0, 285.0), Vec2::new(662.0, 221.0)),
        369.702
    );

    assert_delta!(
        aurora.path(Vec2::new(17.0, 607.0), Vec2::new(383.0, 668.0)),
        380.292
    );

    assert_delta!(
        aurora.path(Vec2::new(463.0, 356.0), Vec2::new(365.0, 263.0)),
        379.594
    );

    assert_delta!(
        aurora.path(Vec2::new(647.0, 585.0), Vec2::new(344.0, 510.0)),
        376.343
    );

    assert_delta!(
        aurora.path(Vec2::new(313.0, 176.0), Vec2::new(141.0, 435.0)),
        376.441
    );

    assert_delta!(
        aurora.path(Vec2::new(905.0, 377.0), Vec2::new(665.0, 555.0)),
        376.086
    );

    assert_delta!(
        aurora.path(Vec2::new(850.0, 663.0), Vec2::new(471.0, 636.0)),
        385.347
    );

    assert_delta!(
        aurora.path(Vec2::new(183.0, 555.0), Vec2::new(264.0, 354.0)),
        378.811
    );

    assert_delta!(
        aurora.path(Vec2::new(382.0, 440.0), Vec2::new(730.0, 401.0)),
        374.328
    );

    assert_delta!(
        aurora.path(Vec2::new(294.0, 284.0), Vec2::new(43.0, 505.0)),
        380.548
    );

    assert_delta!(
        aurora.path(Vec2::new(407.0, 490.0), Vec2::new(515.0, 363.0)),
        376.821
    );

    assert_delta!(
        aurora.path(Vec2::new(338.0, 163.0), Vec2::new(712.0, 108.0)),
        381.264
    );

    assert_delta!(
        aurora.path(Vec2::new(990.0, 295.0), Vec2::new(747.0, 513.0)),
        378.891
    );

    assert_delta!(
        aurora.path(Vec2::new(682.0, 149.0), Vec2::new(344.0, 256.0)),
        384.67
    );

    assert_delta!(
        aurora.path(Vec2::new(711.0, 404.0), Vec2::new(688.0, 683.0)),
        387.244
    );

    assert_delta!(
        aurora.path(Vec2::new(613.0, 393.0), Vec2::new(968.0, 384.0)),
        377.47
    );

    assert_delta!(
        aurora.path(Vec2::new(754.0, 405.0), Vec2::new(977.0, 285.0)),
        387.29
    );

    assert_delta!(
        aurora.path(Vec2::new(776.0, 308.0), Vec2::new(614.0, 185.0)),
        380.363
    );

    assert_delta!(
        aurora.path(Vec2::new(483.0, 656.0), Vec2::new(345.0, 492.0)),
        378.174
    );

    assert_delta!(
        aurora.path(Vec2::new(27.0, 434.0), Vec2::new(243.0, 234.0)),
        378.012
    );

    assert_delta!(
        aurora.path(Vec2::new(411.0, 412.0), Vec2::new(733.0, 602.0)),
        377.631
    );

    assert_delta!(
        aurora.path(Vec2::new(798.0, 401.0), Vec2::new(726.0, 152.0)),
        383.313
    );

    assert_delta!(
        aurora.path(Vec2::new(187.0, 434.0), Vec2::new(526.0, 545.0)),
        383.499
    );

    assert_delta!(
        aurora.path(Vec2::new(343.0, 101.0), Vec2::new(99.0, 322.0)),
        383.827
    );

    assert_delta!(
        aurora.path(Vec2::new(137.0, 504.0), Vec2::new(502.0, 569.0)),
        386.451
    );

    assert_delta!(
        aurora.path(Vec2::new(215.0, 368.0), Vec2::new(248.0, 614.0)),
        388.117
    );

    assert_delta!(
        aurora.path(Vec2::new(65.0, 583.0), Vec2::new(162.0, 317.0)),
        386.419
    );

    assert_delta!(
        aurora.path(Vec2::new(550.0, 269.0), Vec2::new(928.0, 278.0)),
        386.758
    );

    assert_delta!(
        aurora.path(Vec2::new(324.0, 536.0), Vec2::new(213.0, 304.0)),
        384.076
    );

    assert_delta!(
        aurora.path(Vec2::new(627.0, 318.0), Vec2::new(671.0, 175.0)),
        385.533
    );

    assert_delta!(
        aurora.path(Vec2::new(318.0, 388.0), Vec2::new(444.0, 238.0)),
        385.422
    );

    assert_delta!(
        aurora.path(Vec2::new(361.0, 568.0), Vec2::new(49.0, 362.0)),
        387.463
    );

    assert_delta!(
        aurora.path(Vec2::new(14.0, 331.0), Vec2::new(45.0, 581.0)),
        384.823
    );

    assert_delta!(
        aurora.path(Vec2::new(116.0, 371.0), Vec2::new(285.0, 120.0)),
        389.062
    );

    assert_delta!(
        aurora.path(Vec2::new(339.0, 686.0), Vec2::new(394.0, 430.0)),
        391.129
    );

    assert_delta!(
        aurora.path(Vec2::new(475.0, 444.0), Vec2::new(129.0, 291.0)),
        386.706
    );

    assert_delta!(
        aurora.path(Vec2::new(8.0, 531.0), Vec2::new(343.0, 649.0)),
        382.428
    );

    assert_delta!(
        aurora.path(Vec2::new(743.0, 239.0), Vec2::new(747.0, 367.0)),
        389.112
    );

    assert_delta!(
        aurora.path(Vec2::new(30.0, 459.0), Vec2::new(217.0, 171.0)),
        390.319
    );

    assert_delta!(
        aurora.path(Vec2::new(143.0, 415.0), Vec2::new(324.0, 142.0)),
        390.041
    );

    assert_delta!(
        aurora.path(Vec2::new(386.0, 300.0), Vec2::new(382.0, 569.0)),
        391.428
    );

    assert_delta!(
        aurora.path(Vec2::new(852.0, 365.0), Vec2::new(750.0, 220.0)),
        386.869
    );

    assert_delta!(
        aurora.path(Vec2::new(625.0, 138.0), Vec2::new(252.0, 79.0)),
        384.961
    );

    assert_delta!(
        aurora.path(Vec2::new(637.0, 346.0), Vec2::new(829.0, 584.0)),
        389.616
    );

    assert_delta!(
        aurora.path(Vec2::new(506.0, 329.0), Vec2::new(901.0, 303.0)),
        398.747
    );

    assert_delta!(
        aurora.path(Vec2::new(482.0, 553.0), Vec2::new(285.0, 646.0)),
        391.15
    );

    assert_delta!(
        aurora.path(Vec2::new(630.0, 697.0), Vec2::new(766.0, 432.0)),
        391.079
    );

    assert_delta!(
        aurora.path(Vec2::new(454.0, 257.0), Vec2::new(816.0, 364.0)),
        394.071
    );

    assert_delta!(
        aurora.path(Vec2::new(704.0, 369.0), Vec2::new(478.0, 513.0)),
        398.299
    );

    assert_delta!(
        aurora.path(Vec2::new(864.0, 356.0), Vec2::new(662.0, 154.0)),
        391.296
    );

    assert_delta!(
        aurora.path(Vec2::new(630.0, 68.0), Vec2::new(544.0, 347.0)),
        391.383
    );

    assert_delta!(
        aurora.path(Vec2::new(446.0, 318.0), Vec2::new(265.0, 206.0)),
        399.45
    );

    assert_delta!(
        aurora.path(Vec2::new(136.0, 625.0), Vec2::new(465.0, 460.0)),
        390.413
    );

    assert_delta!(
        aurora.path(Vec2::new(812.0, 331.0), Vec2::new(600.0, 162.0)),
        394.792
    );

    assert_delta!(
        aurora.path(Vec2::new(847.0, 683.0), Vec2::new(521.0, 467.0)),
        395.976
    );

    assert_delta!(
        aurora.path(Vec2::new(380.0, 437.0), Vec2::new(26.0, 600.0)),
        390.548
    );

    assert_delta!(
        aurora.path(Vec2::new(380.0, 89.0), Vec2::new(385.0, 351.0)),
        400.243
    );

    assert_delta!(
        aurora.path(Vec2::new(786.0, 324.0), Vec2::new(411.0, 291.0)),
        399.437
    );

    assert_delta!(
        aurora.path(Vec2::new(642.0, 569.0), Vec2::new(349.0, 366.0)),
        391.431
    );

    assert_delta!(
        aurora.path(Vec2::new(443.0, 390.0), Vec2::new(192.0, 505.0)),
        398.109
    );

    assert_delta!(
        aurora.path(Vec2::new(467.0, 669.0), Vec2::new(70.0, 661.0)),
        402.888
    );

    assert_delta!(
        aurora.path(Vec2::new(767.0, 559.0), Vec2::new(786.0, 334.0)),
        396.009
    );

    assert_delta!(
        aurora.path(Vec2::new(529.0, 440.0), Vec2::new(431.0, 456.0)),
        403.223
    );

    assert_delta!(
        aurora.path(Vec2::new(612.0, 47.0), Vec2::new(888.0, 251.0)),
        398.249
    );

    assert_delta!(
        aurora.path(Vec2::new(532.0, 381.0), Vec2::new(624.0, 211.0)),
        398.016
    );

    assert_delta!(
        aurora.path(Vec2::new(829.0, 375.0), Vec2::new(454.0, 342.0)),
        401.96
    );

    assert_delta!(
        aurora.path(Vec2::new(463.0, 61.0), Vec2::new(355.0, 303.0)),
        399.139
    );

    assert_delta!(
        aurora.path(Vec2::new(610.0, 319.0), Vec2::new(400.0, 244.0)),
        403.331
    );

    assert_delta!(
        aurora.path(Vec2::new(485.0, 373.0), Vec2::new(111.0, 285.0)),
        397.104
    );

    assert_delta!(
        aurora.path(Vec2::new(802.0, 723.0), Vec2::new(439.0, 616.0)),
        400.644
    );

    assert_delta!(
        aurora.path(Vec2::new(844.0, 559.0), Vec2::new(784.0, 307.0)),
        398.74
    );

    assert_delta!(
        aurora.path(Vec2::new(741.0, 397.0), Vec2::new(385.0, 543.0)),
        402.343
    );

    assert_delta!(
        aurora.path(Vec2::new(509.0, 50.0), Vec2::new(418.0, 306.0)),
        401.729
    );

    assert_delta!(
        aurora.path(Vec2::new(181.0, 472.0), Vec2::new(504.0, 368.0)),
        398.876
    );

    assert_delta!(
        aurora.path(Vec2::new(384.0, 679.0), Vec2::new(627.0, 697.0)),
        402.887
    );

    assert_delta!(
        aurora.path(Vec2::new(610.0, 547.0), Vec2::new(996.0, 455.0)),
        402.25
    );

    assert_delta!(
        aurora.path(Vec2::new(198.0, 570.0), Vec2::new(129.0, 320.0)),
        406.694
    );

    assert_delta!(
        aurora.path(Vec2::new(995.0, 618.0), Vec2::new(610.0, 637.0)),
        402.078
    );

    assert_delta!(
        aurora.path(Vec2::new(459.0, 479.0), Vec2::new(525.0, 262.0)),
        403.232
    );

    assert_delta!(
        aurora.path(Vec2::new(443.0, 194.0), Vec2::new(782.0, 368.0)),
        399.551
    );

    assert_delta!(
        aurora.path(Vec2::new(560.0, 320.0), Vec2::new(292.0, 118.0)),
        403.301
    );

    assert_delta!(
        aurora.path(Vec2::new(496.0, 543.0), Vec2::new(759.0, 360.0)),
        409.36
    );

    assert_delta!(
        aurora.path(Vec2::new(478.0, 328.0), Vec2::new(403.0, 530.0)),
        402.696
    );

    assert_delta!(
        aurora.path(Vec2::new(64.0, 604.0), Vec2::new(49.0, 320.0)),
        403.551
    );

    assert_delta!(
        aurora.path(Vec2::new(609.0, 478.0), Vec2::new(418.0, 314.0)),
        402.082
    );

    assert_delta!(
        aurora.path(Vec2::new(594.0, 557.0), Vec2::new(219.0, 507.0)),
        410.619
    );

    assert_delta!(
        aurora.path(Vec2::new(684.0, 102.0), Vec2::new(583.0, 323.0)),
        403.567
    );

    assert_delta!(
        aurora.path(Vec2::new(516.0, 565.0), Vec2::new(232.0, 378.0)),
        413.029
    );

    assert_delta!(
        aurora.path(Vec2::new(464.0, 207.0), Vec2::new(850.0, 102.0)),
        411.882
    );

    assert_delta!(
        aurora.path(Vec2::new(246.0, 607.0), Vec2::new(16.0, 362.0)),
        410.466
    );

    assert_delta!(
        aurora.path(Vec2::new(81.0, 621.0), Vec2::new(43.0, 315.0)),
        408.736
    );

    assert_delta!(
        aurora.path(Vec2::new(399.0, 716.0), Vec2::new(306.0, 545.0)),
        403.656
    );

    assert_delta!(
        aurora.path(Vec2::new(79.0, 125.0), Vec2::new(111.0, 430.0)),
        409.152
    );

    assert_delta!(
        aurora.path(Vec2::new(757.0, 230.0), Vec2::new(465.0, 289.0)),
        409.409
    );

    assert_delta!(
        aurora.path(Vec2::new(765.0, 176.0), Vec2::new(511.0, 305.0)),
        419.386
    );

    assert_delta!(
        aurora.path(Vec2::new(693.0, 113.0), Vec2::new(309.0, 228.0)),
        409.659
    );

    assert_delta!(
        aurora.path(Vec2::new(280.0, 77.0), Vec2::new(250.0, 384.0)),
        411.317
    );

    assert_delta!(
        aurora.path(Vec2::new(536.0, 408.0), Vec2::new(285.0, 266.0)),
        407.221
    );

    assert_delta!(
        aurora.path(Vec2::new(737.0, 147.0), Vec2::new(559.0, 310.0)),
        415.318
    );

    assert_delta!(
        aurora.path(Vec2::new(305.0, 139.0), Vec2::new(473.0, 350.0)),
        411.773
    );

    assert_delta!(
        aurora.path(Vec2::new(63.0, 154.0), Vec2::new(213.0, 412.0)),
        409.67
    );

    assert_delta!(
        aurora.path(Vec2::new(53.0, 622.0), Vec2::new(9.0, 335.0)),
        413.208
    );

    assert_delta!(
        aurora.path(Vec2::new(32.0, 308.0), Vec2::new(328.0, 564.0)),
        418.83
    );

    assert_delta!(
        aurora.path(Vec2::new(431.0, 192.0), Vec2::new(201.0, 412.0)),
        415.771
    );

    assert_delta!(
        aurora.path(Vec2::new(87.0, 169.0), Vec2::new(2.0, 421.0)),
        415.887
    );

    assert_delta!(
        aurora.path(Vec2::new(934.0, 663.0), Vec2::new(526.0, 608.0)),
        420.271
    );

    assert_delta!(
        aurora.path(Vec2::new(884.0, 409.0), Vec2::new(589.0, 522.0)),
        412.703
    );

    assert_delta!(
        aurora.path(Vec2::new(154.0, 201.0), Vec2::new(433.0, 392.0)),
        414.176
    );

    assert_delta!(
        aurora.path(Vec2::new(613.0, 303.0), Vec2::new(216.0, 357.0)),
        411.753
    );

    assert_delta!(
        aurora.path(Vec2::new(785.0, 371.0), Vec2::new(523.0, 539.0)),
        417.059
    );

    assert_delta!(
        aurora.path(Vec2::new(130.0, 273.0), Vec2::new(320.0, 567.0)),
        420.097
    );

    assert_delta!(
        aurora.path(Vec2::new(271.0, 694.0), Vec2::new(632.0, 600.0)),
        420.676
    );

    assert_delta!(
        aurora.path(Vec2::new(162.0, 536.0), Vec2::new(168.0, 247.0)),
        412.399
    );

    assert_delta!(
        aurora.path(Vec2::new(621.0, 157.0), Vec2::new(620.0, 400.0)),
        417.794
    );

    assert_delta!(
        aurora.path(Vec2::new(577.0, 371.0), Vec2::new(782.0, 225.0)),
        416.74
    );

    assert_delta!(
        aurora.path(Vec2::new(142.0, 686.0), Vec2::new(79.0, 366.0)),
        417.615
    );

    assert_delta!(
        aurora.path(Vec2::new(399.0, 353.0), Vec2::new(47.0, 480.0)),
        414.584
    );

    assert_delta!(
        aurora.path(Vec2::new(83.0, 634.0), Vec2::new(110.0, 325.0)),
        417.201
    );

    assert_delta!(
        aurora.path(Vec2::new(290.0, 369.0), Vec2::new(569.0, 172.0)),
        412.043
    );

    assert_delta!(
        aurora.path(Vec2::new(265.0, 412.0), Vec2::new(399.0, 242.0)),
        417.273
    );

    assert_delta!(
        aurora.path(Vec2::new(693.0, 273.0), Vec2::new(862.0, 552.0)),
        423.417
    );

    assert_delta!(
        aurora.path(Vec2::new(581.0, 438.0), Vec2::new(819.0, 505.0)),
        413.794
    );

    assert_delta!(
        aurora.path(Vec2::new(829.0, 651.0), Vec2::new(936.0, 335.0)),
        424.047
    );

    assert_delta!(
        aurora.path(Vec2::new(466.0, 505.0), Vec2::new(634.0, 429.0)),
        421.057
    );

    assert_delta!(
        aurora.path(Vec2::new(576.0, 191.0), Vec2::new(706.0, 375.0)),
        418.507
    );

    assert_delta!(
        aurora.path(Vec2::new(612.0, 270.0), Vec2::new(206.0, 336.0)),
        421.58
    );

    assert_delta!(
        aurora.path(Vec2::new(567.0, 558.0), Vec2::new(237.0, 614.0)),
        426.173
    );

    assert_delta!(
        aurora.path(Vec2::new(328.0, 89.0), Vec2::new(234.0, 396.0)),
        424.918
    );

    assert_delta!(
        aurora.path(Vec2::new(224.0, 285.0), Vec2::new(349.0, 539.0)),
        424.938
    );

    assert_delta!(
        aurora.path(Vec2::new(456.0, 635.0), Vec2::new(880.0, 657.0)),
        431.499
    );

    assert_delta!(
        aurora.path(Vec2::new(721.0, 345.0), Vec2::new(432.0, 418.0)),
        417.287
    );

    assert_delta!(
        aurora.path(Vec2::new(450.0, 382.0), Vec2::new(107.0, 491.0)),
        428.436
    );

    assert_delta!(
        aurora.path(Vec2::new(523.0, 233.0), Vec2::new(853.0, 102.0)),
        423.712
    );

    assert_delta!(
        aurora.path(Vec2::new(513.0, 401.0), Vec2::new(335.0, 283.0)),
        423.937
    );

    assert_delta!(
        aurora.path(Vec2::new(690.0, 102.0), Vec2::new(281.0, 117.0)),
        424.367
    );

    assert_delta!(
        aurora.path(Vec2::new(386.0, 288.0), Vec2::new(502.0, 630.0)),
        434.657
    );

    assert_delta!(
        aurora.path(Vec2::new(339.0, 360.0), Vec2::new(616.0, 568.0)),
        422.809
    );

    assert_delta!(
        aurora.path(Vec2::new(846.0, 659.0), Vec2::new(489.0, 434.0)),
        425.043
    );

    assert_delta!(
        aurora.path(Vec2::new(862.0, 388.0), Vec2::new(823.0, 95.0)),
        421.173
    );

    assert_delta!(
        aurora.path(Vec2::new(539.0, 335.0), Vec2::new(123.0, 285.0)),
        429.989
    );

    assert_delta!(
        aurora.path(Vec2::new(673.0, 696.0), Vec2::new(334.0, 551.0)),
        422.286
    );

    assert_delta!(
        aurora.path(Vec2::new(826.0, 425.0), Vec2::new(633.0, 565.0)),
        423.725
    );

    assert_delta!(
        aurora.path(Vec2::new(906.0, 584.0), Vec2::new(489.0, 536.0)),
        429.246
    );

    assert_delta!(
        aurora.path(Vec2::new(495.0, 623.0), Vec2::new(722.0, 363.0)),
        427.88
    );

    assert_delta!(
        aurora.path(Vec2::new(876.0, 585.0), Vec2::new(465.0, 485.0)),
        427.27
    );

    assert_delta!(
        aurora.path(Vec2::new(261.0, 413.0), Vec2::new(650.0, 445.0)),
        424.059
    );

    assert_delta!(
        aurora.path(Vec2::new(155.0, 652.0), Vec2::new(580.0, 695.0)),
        434.844
    );

    assert_delta!(
        aurora.path(Vec2::new(484.0, 321.0), Vec2::new(719.0, 249.0)),
        427.143
    );

    assert_delta!(
        aurora.path(Vec2::new(288.0, 589.0), Vec2::new(581.0, 526.0)),
        427.611
    );

    assert_delta!(
        aurora.path(Vec2::new(142.0, 166.0), Vec2::new(384.0, 301.0)),
        423.228
    );

    assert_delta!(
        aurora.path(Vec2::new(447.0, 149.0), Vec2::new(586.0, 464.0)),
        429.709
    );

    assert_delta!(
        aurora.path(Vec2::new(847.0, 160.0), Vec2::new(461.0, 73.0)),
        423.783
    );

    assert_delta!(
        aurora.path(Vec2::new(757.0, 410.0), Vec2::new(508.0, 168.0)),
        425.9
    );

    assert_delta!(
        aurora.path(Vec2::new(729.0, 82.0), Vec2::new(662.0, 264.0)),
        426.236
    );

    assert_delta!(
        aurora.path(Vec2::new(726.0, 232.0), Vec2::new(326.0, 91.0)),
        428.791
    );

    assert_delta!(
        aurora.path(Vec2::new(108.0, 291.0), Vec2::new(163.0, 616.0)),
        432.743
    );

    assert_delta!(
        aurora.path(Vec2::new(570.0, 401.0), Vec2::new(754.0, 517.0)),
        428.601
    );

    assert_delta!(
        aurora.path(Vec2::new(313.0, 622.0), Vec2::new(352.0, 354.0)),
        433.174
    );

    assert_delta!(
        aurora.path(Vec2::new(549.0, 603.0), Vec2::new(960.0, 573.0)),
        434.068
    );

    assert_delta!(
        aurora.path(Vec2::new(587.0, 334.0), Vec2::new(1002.0, 363.0)),
        434.586
    );

    assert_delta!(
        aurora.path(Vec2::new(471.0, 499.0), Vec2::new(127.0, 277.0)),
        437.855
    );

    assert_delta!(
        aurora.path(Vec2::new(190.0, 509.0), Vec2::new(498.0, 360.0)),
        431.204
    );

    assert_delta!(
        aurora.path(Vec2::new(731.0, 276.0), Vec2::new(414.0, 281.0)),
        429.956
    );

    assert_delta!(
        aurora.path(Vec2::new(634.0, 341.0), Vec2::new(297.0, 171.0)),
        441.632
    );

    assert_delta!(
        aurora.path(Vec2::new(408.0, 704.0), Vec2::new(68.0, 684.0)),
        440.138
    );

    assert_delta!(
        aurora.path(Vec2::new(307.0, 321.0), Vec2::new(457.0, 63.0)),
        438.653
    );

    assert_delta!(
        aurora.path(Vec2::new(745.0, 279.0), Vec2::new(590.0, 274.0)),
        434.65
    );

    assert_delta!(
        aurora.path(Vec2::new(322.0, 499.0), Vec2::new(510.0, 344.0)),
        439.298
    );

    assert_delta!(
        aurora.path(Vec2::new(382.0, 457.0), Vec2::new(609.0, 685.0)),
        437.802
    );

    assert_delta!(
        aurora.path(Vec2::new(249.0, 224.0), Vec2::new(234.0, 416.0)),
        433.503
    );

    assert_delta!(
        aurora.path(Vec2::new(106.0, 363.0), Vec2::new(138.0, 690.0)),
        439.785
    );

    assert_delta!(
        aurora.path(Vec2::new(225.0, 50.0), Vec2::new(639.0, 144.0)),
        434.946
    );

    assert_delta!(
        aurora.path(Vec2::new(455.0, 561.0), Vec2::new(734.0, 676.0)),
        439.071
    );

    assert_delta!(
        aurora.path(Vec2::new(743.0, 423.0), Vec2::new(618.0, 724.0)),
        440.014
    );

    assert_delta!(
        aurora.path(Vec2::new(251.0, 216.0), Vec2::new(537.0, 357.0)),
        447.117
    );

    assert_delta!(
        aurora.path(Vec2::new(593.0, 331.0), Vec2::new(918.0, 552.0)),
        434.761
    );

    assert_delta!(
        aurora.path(Vec2::new(271.0, 43.0), Vec2::new(684.0, 179.0)),
        440.584
    );

    assert_delta!(
        aurora.path(Vec2::new(370.0, 331.0), Vec2::new(656.0, 456.0)),
        434.808
    );

    assert_delta!(
        aurora.path(Vec2::new(64.0, 673.0), Vec2::new(398.0, 700.0)),
        446.771
    );

    assert_delta!(
        aurora.path(Vec2::new(113.0, 389.0), Vec2::new(539.0, 322.0)),
        440.949
    );

    assert_delta!(
        aurora.path(Vec2::new(456.0, 307.0), Vec2::new(780.0, 204.0)),
        446.48
    );

    assert_delta!(
        aurora.path(Vec2::new(34.0, 312.0), Vec2::new(445.0, 200.0)),
        442.506
    );

    assert_delta!(
        aurora.path(Vec2::new(344.0, 317.0), Vec2::new(683.0, 207.0)),
        436.972
    );

    assert_delta!(
        aurora.path(Vec2::new(584.0, 156.0), Vec2::new(147.0, 195.0)),
        456.86
    );

    assert_delta!(
        aurora.path(Vec2::new(650.0, 97.0), Vec2::new(511.0, 427.0)),
        440.539
    );

    assert_delta!(
        aurora.path(Vec2::new(225.0, 124.0), Vec2::new(534.0, 293.0)),
        442.192
    );

    assert_delta!(
        aurora.path(Vec2::new(767.0, 656.0), Vec2::new(922.0, 356.0)),
        447.14
    );

    assert_delta!(
        aurora.path(Vec2::new(17.0, 510.0), Vec2::new(393.0, 696.0)),
        440.979
    );

    assert_delta!(
        aurora.path(Vec2::new(410.0, 197.0), Vec2::new(850.0, 134.0)),
        455.242
    );

    assert_delta!(
        aurora.path(Vec2::new(461.0, 398.0), Vec2::new(365.0, 79.0)),
        441.681
    );

    assert_delta!(
        aurora.path(Vec2::new(588.0, 579.0), Vec2::new(827.0, 409.0)),
        444.749
    );

    assert_delta!(
        aurora.path(Vec2::new(372.0, 300.0), Vec2::new(73.0, 177.0)),
        437.185
    );

    assert_delta!(
        aurora.path(Vec2::new(671.0, 273.0), Vec2::new(279.0, 85.0)),
        442.336
    );

    assert_delta!(
        aurora.path(Vec2::new(324.0, 748.0), Vec2::new(666.0, 708.0)),
        446.15
    );

    assert_delta!(
        aurora.path(Vec2::new(853.0, 299.0), Vec2::new(557.0, 517.0)),
        447.985
    );

    assert_delta!(
        aurora.path(Vec2::new(101.0, 442.0), Vec2::new(381.0, 547.0)),
        444.246
    );

    assert_delta!(
        aurora.path(Vec2::new(563.0, 550.0), Vec2::new(124.0, 509.0)),
        450.851
    );

    assert_delta!(
        aurora.path(Vec2::new(74.0, 560.0), Vec2::new(137.0, 240.0)),
        448.51
    );

    assert_delta!(
        aurora.path(Vec2::new(94.0, 424.0), Vec2::new(502.0, 514.0)),
        445.108
    );

    assert_delta!(
        aurora.path(Vec2::new(641.0, 559.0), Vec2::new(246.0, 664.0)),
        456.713
    );

    assert_delta!(
        aurora.path(Vec2::new(132.0, 435.0), Vec2::new(540.0, 276.0)),
        446.555
    );

    assert_delta!(
        aurora.path(Vec2::new(693.0, 216.0), Vec2::new(515.0, 387.0)),
        446.361
    );

    assert_delta!(
        aurora.path(Vec2::new(892.0, 442.0), Vec2::new(464.0, 478.0)),
        445.846
    );

    assert_delta!(
        aurora.path(Vec2::new(148.0, 716.0), Vec2::new(101.0, 370.0)),
        458.984
    );

    assert_delta!(
        aurora.path(Vec2::new(433.0, 405.0), Vec2::new(559.0, 142.0)),
        448.283
    );

    assert_delta!(
        aurora.path(Vec2::new(252.0, 632.0), Vec2::new(131.0, 293.0)),
        458.671
    );

    assert_delta!(
        aurora.path(Vec2::new(980.0, 369.0), Vec2::new(593.0, 546.0)),
        455.998
    );

    assert_delta!(
        aurora.path(Vec2::new(566.0, 404.0), Vec2::new(941.0, 459.0)),
        447.871
    );

    assert_delta!(
        aurora.path(Vec2::new(989.0, 356.0), Vec2::new(653.0, 612.0)),
        454.895
    );

    assert_delta!(
        aurora.path(Vec2::new(1018.0, 612.0), Vec2::new(664.0, 367.0)),
        449.968
    );

    assert_delta!(
        aurora.path(Vec2::new(641.0, 411.0), Vec2::new(421.0, 538.0)),
        452.05
    );

    assert_delta!(
        aurora.path(Vec2::new(696.0, 315.0), Vec2::new(308.0, 359.0)),
        456.803
    );

    assert_delta!(
        aurora.path(Vec2::new(489.0, 329.0), Vec2::new(644.0, 516.0)),
        454.117
    );

    assert_delta!(
        aurora.path(Vec2::new(527.0, 329.0), Vec2::new(785.0, 278.0)),
        459.407
    );

    assert_delta!(
        aurora.path(Vec2::new(908.0, 300.0), Vec2::new(690.0, 594.0)),
        457.837
    );

    assert_delta!(
        aurora.path(Vec2::new(320.0, 143.0), Vec2::new(764.0, 200.0)),
        457.929
    );

    assert_delta!(
        aurora.path(Vec2::new(190.0, 225.0), Vec2::new(558.0, 252.0)),
        464.633
    );

    assert_delta!(
        aurora.path(Vec2::new(132.0, 300.0), Vec2::new(377.0, 611.0)),
        464.107
    );

    assert_delta!(
        aurora.path(Vec2::new(574.0, 434.0), Vec2::new(354.0, 210.0)),
        459.375
    );

    assert_delta!(
        aurora.path(Vec2::new(366.0, 71.0), Vec2::new(746.0, 312.0)),
        453.473
    );

    assert_delta!(
        aurora.path(Vec2::new(363.0, 538.0), Vec2::new(332.0, 619.0)),
        450.005
    );

    assert_delta!(
        aurora.path(Vec2::new(115.0, 386.0), Vec2::new(528.0, 462.0)),
        453.153
    );

    assert_delta!(
        aurora.path(Vec2::new(318.0, 50.0), Vec2::new(608.0, 318.0)),
        453.569
    );

    assert_delta!(
        aurora.path(Vec2::new(585.0, 295.0), Vec2::new(745.0, 92.0)),
        451.598
    );

    assert_delta!(
        aurora.path(Vec2::new(730.0, 152.0), Vec2::new(275.0, 218.0)),
        469.235
    );

    assert_delta!(
        aurora.path(Vec2::new(549.0, 318.0), Vec2::new(518.0, 523.0)),
        459.476
    );

    assert_delta!(
        aurora.path(Vec2::new(793.0, 335.0), Vec2::new(911.0, 613.0)),
        460.876
    );

    assert_delta!(
        aurora.path(Vec2::new(296.0, 396.0), Vec2::new(273.0, 84.0)),
        458.677
    );

    assert_delta!(
        aurora.path(Vec2::new(316.0, 234.0), Vec2::new(773.0, 165.0)),
        476.46
    );

    assert_delta!(
        aurora.path(Vec2::new(179.0, 169.0), Vec2::new(434.0, 263.0)),
        458.648
    );

    assert_delta!(
        aurora.path(Vec2::new(545.0, 266.0), Vec2::new(241.0, 473.0)),
        453.32
    );

    assert_delta!(
        aurora.path(Vec2::new(339.0, 337.0), Vec2::new(689.0, 147.0)),
        457.375
    );

    assert_delta!(
        aurora.path(Vec2::new(264.0, 536.0), Vec2::new(699.0, 431.0)),
        457.561
    );

    assert_delta!(
        aurora.path(Vec2::new(384.0, 386.0), Vec2::new(272.0, 82.0)),
        461.096
    );

    assert_delta!(
        aurora.path(Vec2::new(633.0, 503.0), Vec2::new(303.0, 665.0)),
        465.27
    );

    assert_delta!(
        aurora.path(Vec2::new(671.0, 695.0), Vec2::new(656.0, 377.0)),
        469.023
    );

    assert_delta!(
        aurora.path(Vec2::new(784.0, 111.0), Vec2::new(678.0, 323.0)),
        461.64
    );

    assert_delta!(
        aurora.path(Vec2::new(141.0, 306.0), Vec2::new(383.0, 538.0)),
        467.381
    );

    assert_delta!(
        aurora.path(Vec2::new(504.0, 630.0), Vec2::new(622.0, 432.0)),
        461.923
    );

    assert_delta!(
        aurora.path(Vec2::new(243.0, 668.0), Vec2::new(312.0, 320.0)),
        471.454
    );

    assert_delta!(
        aurora.path(Vec2::new(799.0, 97.0), Vec2::new(703.0, 370.0)),
        463.419
    );

    assert_delta!(
        aurora.path(Vec2::new(72.0, 335.0), Vec2::new(388.0, 97.0)),
        461.089
    );

    assert_delta!(
        aurora.path(Vec2::new(400.0, 221.0), Vec2::new(458.0, 418.0)),
        459.115
    );

    assert_delta!(
        aurora.path(Vec2::new(451.0, 512.0), Vec2::new(905.0, 473.0)),
        467.006
    );

    assert_delta!(
        aurora.path(Vec2::new(215.0, 460.0), Vec2::new(364.0, 204.0)),
        465.092
    );

    assert_delta!(
        aurora.path(Vec2::new(660.0, 623.0), Vec2::new(654.0, 349.0)),
        463.335
    );

    assert_delta!(
        aurora.path(Vec2::new(9.0, 371.0), Vec2::new(417.0, 186.0)),
        459.064
    );

    assert_delta!(
        aurora.path(Vec2::new(227.0, 604.0), Vec2::new(389.0, 363.0)),
        467.833
    );

    assert_delta!(
        aurora.path(Vec2::new(283.0, 671.0), Vec2::new(533.0, 496.0)),
        471.053
    );

    assert_delta!(
        aurora.path(Vec2::new(477.0, 644.0), Vec2::new(902.0, 484.0)),
        466.233
    );

    assert_delta!(
        aurora.path(Vec2::new(541.0, 379.0), Vec2::new(240.0, 484.0)),
        464.909
    );

    assert_delta!(
        aurora.path(Vec2::new(213.0, 459.0), Vec2::new(341.0, 187.0)),
        465.939
    );

    assert_delta!(
        aurora.path(Vec2::new(823.0, 675.0), Vec2::new(375.0, 713.0)),
        463.853
    );

    assert_delta!(
        aurora.path(Vec2::new(981.0, 415.0), Vec2::new(670.0, 652.0)),
        470.458
    );

    assert_delta!(
        aurora.path(Vec2::new(689.0, 536.0), Vec2::new(949.0, 275.0)),
        465.79
    );

    assert_delta!(
        aurora.path(Vec2::new(756.0, 235.0), Vec2::new(516.0, 357.0)),
        469.38
    );

    assert_delta!(
        aurora.path(Vec2::new(23.0, 544.0), Vec2::new(194.0, 254.0)),
        470.662
    );

    assert_delta!(
        aurora.path(Vec2::new(739.0, 574.0), Vec2::new(818.0, 282.0)),
        474.274
    );

    assert_delta!(
        aurora.path(Vec2::new(653.0, 113.0), Vec2::new(215.0, 57.0)),
        465.554
    );

    assert_delta!(
        aurora.path(Vec2::new(639.0, 147.0), Vec2::new(639.0, 427.0)),
        466.703
    );

    assert_delta!(
        aurora.path(Vec2::new(478.0, 52.0), Vec2::new(557.0, 403.0)),
        473.214
    );

    assert_delta!(
        aurora.path(Vec2::new(808.0, 194.0), Vec2::new(683.0, 452.0)),
        471.055
    );

    assert_delta!(
        aurora.path(Vec2::new(264.0, 239.0), Vec2::new(671.0, 297.0)),
        471.527
    );

    assert_delta!(
        aurora.path(Vec2::new(454.0, 477.0), Vec2::new(594.0, 223.0)),
        475.617
    );

    assert_delta!(
        aurora.path(Vec2::new(589.0, 319.0), Vec2::new(508.0, 595.0)),
        472.897
    );

    assert_delta!(
        aurora.path(Vec2::new(948.0, 393.0), Vec2::new(571.0, 420.0)),
        472.307
    );

    assert_delta!(
        aurora.path(Vec2::new(838.0, 94.0), Vec2::new(507.0, 283.0)),
        472.239
    );

    assert_delta!(
        aurora.path(Vec2::new(729.0, 278.0), Vec2::new(310.0, 81.0)),
        470.131
    );

    assert_delta!(
        aurora.path(Vec2::new(741.0, 56.0), Vec2::new(297.0, 33.0)),
        469.718
    );

    assert_delta!(
        aurora.path(Vec2::new(431.0, 634.0), Vec2::new(34.0, 384.0)),
        478.876
    );

    assert_delta!(
        aurora.path(Vec2::new(432.0, 427.0), Vec2::new(635.0, 708.0)),
        476.123
    );

    assert_delta!(
        aurora.path(Vec2::new(793.0, 515.0), Vec2::new(320.0, 572.0)),
        487.151
    );

    assert_delta!(
        aurora.path(Vec2::new(376.0, 529.0), Vec2::new(309.0, 682.0)),
        470.566
    );

    assert_delta!(
        aurora.path(Vec2::new(414.0, 609.0), Vec2::new(61.0, 437.0)),
        476.724
    );

    assert_delta!(
        aurora.path(Vec2::new(960.0, 650.0), Vec2::new(623.0, 410.0)),
        482.714
    );

    assert_delta!(
        aurora.path(Vec2::new(147.0, 346.0), Vec2::new(374.0, 651.0)),
        481.703
    );

    assert_delta!(
        aurora.path(Vec2::new(415.0, 435.0), Vec2::new(681.0, 422.0)),
        471.401
    );

    assert_delta!(
        aurora.path(Vec2::new(740.0, 419.0), Vec2::new(290.0, 528.0)),
        472.62
    );

    assert_delta!(
        aurora.path(Vec2::new(545.0, 23.0), Vec2::new(99.0, 150.0)),
        473.543
    );

    assert_delta!(
        aurora.path(Vec2::new(234.0, 361.0), Vec2::new(528.0, 622.0)),
        484.101
    );

    assert_delta!(
        aurora.path(Vec2::new(152.0, 590.0), Vec2::new(600.0, 619.0)),
        480.639
    );

    assert_delta!(
        aurora.path(Vec2::new(96.0, 117.0), Vec2::new(542.0, 212.0)),
        483.633
    );

    assert_delta!(
        aurora.path(Vec2::new(416.0, 419.0), Vec2::new(769.0, 362.0)),
        477.74
    );

    assert_delta!(
        aurora.path(Vec2::new(953.0, 562.0), Vec2::new(524.0, 708.0)),
        479.178
    );

    assert_delta!(
        aurora.path(Vec2::new(30.0, 477.0), Vec2::new(497.0, 516.0)),
        485.753
    );

    assert_delta!(
        aurora.path(Vec2::new(603.0, 549.0), Vec2::new(481.0, 332.0)),
        483.533
    );

    assert_delta!(
        aurora.path(Vec2::new(33.0, 583.0), Vec2::new(348.0, 383.0)),
        476.255
    );

    assert_delta!(
        aurora.path(Vec2::new(179.0, 173.0), Vec2::new(655.0, 113.0)),
        490.491
    );

    assert_delta!(
        aurora.path(Vec2::new(932.0, 156.0), Vec2::new(682.0, 450.0)),
        485.575
    );

    assert_delta!(
        aurora.path(Vec2::new(485.0, 446.0), Vec2::new(904.0, 665.0)),
        482.32
    );

    assert_delta!(
        aurora.path(Vec2::new(147.0, 105.0), Vec2::new(32.0, 430.0)),
        480.632
    );

    assert_delta!(
        aurora.path(Vec2::new(376.0, 575.0), Vec2::new(806.0, 717.0)),
        477.084
    );

    assert_delta!(
        aurora.path(Vec2::new(482.0, 581.0), Vec2::new(55.0, 671.0)),
        483.528
    );

    assert_delta!(
        aurora.path(Vec2::new(657.0, 575.0), Vec2::new(490.0, 374.0)),
        487.415
    );

    assert_delta!(
        aurora.path(Vec2::new(1011.0, 311.0), Vec2::new(926.0, 649.0)),
        488.482
    );

    assert_delta!(
        aurora.path(Vec2::new(502.0, 388.0), Vec2::new(731.0, 242.0)),
        481.328
    );

    assert_delta!(
        aurora.path(Vec2::new(474.0, 558.0), Vec2::new(38.0, 400.0)),
        482.414
    );

    assert_delta!(
        aurora.path(Vec2::new(609.0, 653.0), Vec2::new(974.0, 396.0)),
        480.578
    );

    assert_delta!(
        aurora.path(Vec2::new(759.0, 552.0), Vec2::new(315.0, 370.0)),
        482.398
    );

    assert_delta!(
        aurora.path(Vec2::new(806.0, 92.0), Vec2::new(935.0, 413.0)),
        490.011
    );

    assert_delta!(
        aurora.path(Vec2::new(548.0, 428.0), Vec2::new(983.0, 361.0)),
        492.026
    );

    assert_delta!(
        aurora.path(Vec2::new(696.0, 705.0), Vec2::new(282.0, 668.0)),
        485.222
    );

    assert_delta!(
        aurora.path(Vec2::new(745.0, 251.0), Vec2::new(708.0, 401.0)),
        484.553
    );

    assert_delta!(
        aurora.path(Vec2::new(456.0, 242.0), Vec2::new(79.0, 469.0)),
        494.778
    );

    assert_delta!(
        aurora.path(Vec2::new(381.0, 224.0), Vec2::new(794.0, 207.0)),
        492.921
    );

    assert_delta!(
        aurora.path(Vec2::new(63.0, 334.0), Vec2::new(167.0, 734.0)),
        492.445
    );

    assert_delta!(
        aurora.path(Vec2::new(570.0, 698.0), Vec2::new(333.0, 468.0)),
        492.232
    );

    assert_delta!(
        aurora.path(Vec2::new(411.0, 583.0), Vec2::new(877.0, 688.0)),
        485.689
    );

    assert_delta!(
        aurora.path(Vec2::new(489.0, 600.0), Vec2::new(478.0, 318.0)),
        491.786
    );

    assert_delta!(
        aurora.path(Vec2::new(243.0, 423.0), Vec2::new(577.0, 204.0)),
        491.426
    );

    assert_delta!(
        aurora.path(Vec2::new(609.0, 644.0), Vec2::new(883.0, 363.0)),
        487.707
    );

    assert_delta!(
        aurora.path(Vec2::new(925.0, 594.0), Vec2::new(463.0, 447.0)),
        487.106
    );

    assert_delta!(
        aurora.path(Vec2::new(203.0, 181.0), Vec2::new(681.0, 85.0)),
        500.275
    );

    assert_delta!(
        aurora.path(Vec2::new(693.0, 533.0), Vec2::new(875.0, 267.0)),
        491.335
    );

    assert_delta!(
        aurora.path(Vec2::new(197.0, 305.0), Vec2::new(43.0, 665.0)),
        499.197
    );

    assert_delta!(
        aurora.path(Vec2::new(441.0, 203.0), Vec2::new(205.0, 442.0)),
        492.786
    );

    assert_delta!(
        aurora.path(Vec2::new(768.0, 166.0), Vec2::new(727.0, 402.0)),
        488.639
    );

    assert_delta!(
        aurora.path(Vec2::new(193.0, 179.0), Vec2::new(548.0, 342.0)),
        497.748
    );

    assert_delta!(
        aurora.path(Vec2::new(429.0, 146.0), Vec2::new(716.0, 422.0)),
        489.641
    );

    assert_delta!(
        aurora.path(Vec2::new(470.0, 687.0), Vec2::new(28.0, 540.0)),
        485.16
    );

    assert_delta!(
        aurora.path(Vec2::new(485.0, 644.0), Vec2::new(644.0, 347.0)),
        494.053
    );

    assert_delta!(
        aurora.path(Vec2::new(83.0, 317.0), Vec2::new(311.0, 641.0)),
        499.216
    );

    assert_delta!(
        aurora.path(Vec2::new(37.0, 651.0), Vec2::new(491.0, 486.0)),
        496.928
    );

    assert_delta!(
        aurora.path(Vec2::new(594.0, 723.0), Vec2::new(420.0, 473.0)),
        492.653
    );

    assert_delta!(
        aurora.path(Vec2::new(415.0, 599.0), Vec2::new(913.0, 614.0)),
        508.97
    );

    assert_delta!(
        aurora.path(Vec2::new(505.0, 355.0), Vec2::new(519.0, 583.0)),
        500.307
    );

    assert_delta!(
        aurora.path(Vec2::new(999.0, 438.0), Vec2::new(753.0, 122.0)),
        494.758
    );

    assert_delta!(
        aurora.path(Vec2::new(127.0, 306.0), Vec2::new(604.0, 262.0)),
        494.298
    );

    assert_delta!(
        aurora.path(Vec2::new(585.0, 574.0), Vec2::new(941.0, 433.0)),
        499.593
    );

    assert_delta!(
        aurora.path(Vec2::new(548.0, 430.0), Vec2::new(127.0, 439.0)),
        495.496
    );

    assert_delta!(
        aurora.path(Vec2::new(707.0, 694.0), Vec2::new(263.0, 741.0)),
        493.97
    );

    assert_delta!(
        aurora.path(Vec2::new(284.0, 72.0), Vec2::new(759.0, 80.0)),
        493.851
    );

    assert_delta!(
        aurora.path(Vec2::new(564.0, 571.0), Vec2::new(155.0, 353.0)),
        511.745
    );

    assert_delta!(
        aurora.path(Vec2::new(332.0, 221.0), Vec2::new(803.0, 283.0)),
        500.716
    );

    assert_delta!(
        aurora.path(Vec2::new(441.0, 472.0), Vec2::new(428.0, 197.0)),
        505.727
    );

    assert_delta!(
        aurora.path(Vec2::new(441.0, 80.0), Vec2::new(141.0, 402.0)),
        499.754
    );

    assert_delta!(
        aurora.path(Vec2::new(601.0, 278.0), Vec2::new(870.0, 155.0)),
        495.758
    );

    assert_delta!(
        aurora.path(Vec2::new(353.0, 293.0), Vec2::new(561.0, 463.0)),
        503.222
    );

    assert_delta!(
        aurora.path(Vec2::new(157.0, 150.0), Vec2::new(666.0, 174.0)),
        515.258
    );

    assert_delta!(
        aurora.path(Vec2::new(326.0, 516.0), Vec2::new(310.0, 228.0)),
        502.646
    );

    assert_delta!(
        aurora.path(Vec2::new(643.0, 672.0), Vec2::new(760.0, 399.0)),
        502.887
    );

    assert_delta!(
        aurora.path(Vec2::new(630.0, 342.0), Vec2::new(239.0, 247.0)),
        503.551
    );

    assert_delta!(
        aurora.path(Vec2::new(611.0, 281.0), Vec2::new(199.0, 161.0)),
        503.642
    );

    assert_delta!(
        aurora.path(Vec2::new(626.0, 390.0), Vec2::new(754.0, 228.0)),
        500.934
    );

    assert_delta!(
        aurora.path(Vec2::new(573.0, 165.0), Vec2::new(103.0, 292.0)),
        511.339
    );

    assert_delta!(
        aurora.path(Vec2::new(120.0, 386.0), Vec2::new(580.0, 427.0)),
        498.937
    );

    assert_delta!(
        aurora.path(Vec2::new(981.0, 419.0), Vec2::new(547.0, 485.0)),
        500.841
    );

    assert_delta!(
        aurora.path(Vec2::new(660.0, 215.0), Vec2::new(185.0, 76.0)),
        499.544
    );

    assert_delta!(
        aurora.path(Vec2::new(269.0, 261.0), Vec2::new(650.0, 360.0)),
        502.568
    );

    assert_delta!(
        aurora.path(Vec2::new(236.0, 218.0), Vec2::new(745.0, 172.0)),
        522.283
    );

    assert_delta!(
        aurora.path(Vec2::new(392.0, 518.0), Vec2::new(510.0, 193.0)),
        507.167
    );

    assert_delta!(
        aurora.path(Vec2::new(454.0, 534.0), Vec2::new(581.0, 294.0)),
        509.588
    );

    assert_delta!(
        aurora.path(Vec2::new(126.0, 268.0), Vec2::new(122.0, 645.0)),
        510.925
    );

    assert_delta!(
        aurora.path(Vec2::new(280.0, 623.0), Vec2::new(122.0, 245.0)),
        512.137
    );

    assert_delta!(
        aurora.path(Vec2::new(167.0, 708.0), Vec2::new(35.0, 309.0)),
        505.972
    );

    assert_delta!(
        aurora.path(Vec2::new(108.0, 156.0), Vec2::new(420.0, 273.0)),
        500.906
    );

    assert_delta!(
        aurora.path(Vec2::new(764.0, 507.0), Vec2::new(462.0, 296.0)),
        504.778
    );

    assert_delta!(
        aurora.path(Vec2::new(861.0, 388.0), Vec2::new(600.0, 104.0)),
        505.278
    );

    assert_delta!(
        aurora.path(Vec2::new(497.0, 549.0), Vec2::new(555.0, 430.0)),
        509.249
    );

    assert_delta!(
        aurora.path(Vec2::new(324.0, 654.0), Vec2::new(272.0, 349.0)),
        511.787
    );

    assert_delta!(
        aurora.path(Vec2::new(505.0, 317.0), Vec2::new(227.0, 560.0)),
        507.114
    );

    assert_delta!(
        aurora.path(Vec2::new(748.0, 149.0), Vec2::new(887.0, 481.0)),
        508.815
    );

    assert_delta!(
        aurora.path(Vec2::new(433.0, 312.0), Vec2::new(70.0, 189.0)),
        503.959
    );

    assert_delta!(
        aurora.path(Vec2::new(594.0, 208.0), Vec2::new(707.0, 426.0)),
        510.3
    );

    assert_delta!(
        aurora.path(Vec2::new(764.0, 201.0), Vec2::new(546.0, 414.0)),
        510.129
    );

    assert_delta!(
        aurora.path(Vec2::new(645.0, 168.0), Vec2::new(228.0, 352.0)),
        514.682
    );

    assert_delta!(
        aurora.path(Vec2::new(220.0, 415.0), Vec2::new(167.0, 121.0)),
        517.634
    );

    assert_delta!(
        aurora.path(Vec2::new(353.0, 458.0), Vec2::new(616.0, 443.0)),
        514.181
    );

    assert_delta!(
        aurora.path(Vec2::new(826.0, 292.0), Vec2::new(696.0, 616.0)),
        515.78
    );

    assert_delta!(
        aurora.path(Vec2::new(520.0, 638.0), Vec2::new(452.0, 267.0)),
        518.081
    );

    assert_delta!(
        aurora.path(Vec2::new(903.0, 381.0), Vec2::new(614.0, 671.0)),
        512.421
    );

    assert_delta!(
        aurora.path(Vec2::new(569.0, 394.0), Vec2::new(769.0, 275.0)),
        506.105
    );

    assert_delta!(
        aurora.path(Vec2::new(862.0, 269.0), Vec2::new(506.0, 516.0)),
        514.74
    );

    assert_delta!(
        aurora.path(Vec2::new(286.0, 205.0), Vec2::new(651.0, 369.0)),
        521.171
    );

    assert_delta!(
        aurora.path(Vec2::new(988.0, 586.0), Vec2::new(806.0, 321.0)),
        510.488
    );

    assert_delta!(
        aurora.path(Vec2::new(742.0, 103.0), Vec2::new(709.0, 397.0)),
        515.466
    );

    assert_delta!(
        aurora.path(Vec2::new(975.0, 466.0), Vec2::new(486.0, 413.0)),
        513.562
    );

    assert_delta!(
        aurora.path(Vec2::new(798.0, 325.0), Vec2::new(358.0, 365.0)),
        516.42
    );

    assert_delta!(
        aurora.path(Vec2::new(506.0, 660.0), Vec2::new(25.0, 534.0)),
        511.684
    );

    assert_delta!(
        aurora.path(Vec2::new(863.0, 431.0), Vec2::new(388.0, 343.0)),
        510.944
    );

    assert_delta!(
        aurora.path(Vec2::new(162.0, 476.0), Vec2::new(548.0, 701.0)),
        510.9
    );

    assert_delta!(
        aurora.path(Vec2::new(546.0, 656.0), Vec2::new(867.0, 394.0)),
        518.093
    );

    assert_delta!(
        aurora.path(Vec2::new(426.0, 133.0), Vec2::new(227.0, 413.0)),
        511.548
    );

    assert_delta!(
        aurora.path(Vec2::new(371.0, 604.0), Vec2::new(637.0, 405.0)),
        516.361
    );

    assert_delta!(
        aurora.path(Vec2::new(196.0, 664.0), Vec2::new(658.0, 608.0)),
        527.336
    );

    assert_delta!(
        aurora.path(Vec2::new(305.0, 547.0), Vec2::new(341.0, 273.0)),
        526.123
    );

    assert_delta!(
        aurora.path(Vec2::new(106.0, 190.0), Vec2::new(237.0, 474.0)),
        518.445
    );

    assert_delta!(
        aurora.path(Vec2::new(151.0, 301.0), Vec2::new(255.0, 712.0)),
        524.833
    );

    assert_delta!(
        aurora.path(Vec2::new(639.0, 409.0), Vec2::new(698.0, 114.0)),
        516.397
    );

    assert_delta!(
        aurora.path(Vec2::new(336.0, 587.0), Vec2::new(498.0, 394.0)),
        523.083
    );

    assert_delta!(
        aurora.path(Vec2::new(552.0, 331.0), Vec2::new(231.0, 533.0)),
        523.221
    );

    assert_delta!(
        aurora.path(Vec2::new(333.0, 196.0), Vec2::new(847.0, 150.0)),
        528.238
    );

    assert_delta!(
        aurora.path(Vec2::new(572.0, 19.0), Vec2::new(895.0, 289.0)),
        519.295
    );

    assert_delta!(
        aurora.path(Vec2::new(454.0, 335.0), Vec2::new(782.0, 78.0)),
        517.51
    );

    assert_delta!(
        aurora.path(Vec2::new(23.0, 483.0), Vec2::new(359.0, 533.0)),
        525.71
    );

    assert_delta!(
        aurora.path(Vec2::new(807.0, 450.0), Vec2::new(391.0, 639.0)),
        528.322
    );

    assert_delta!(
        aurora.path(Vec2::new(446.0, 380.0), Vec2::new(830.0, 189.0)),
        524.155
    );

    assert_delta!(
        aurora.path(Vec2::new(630.0, 604.0), Vec2::new(520.0, 344.0)),
        522.982
    );

    assert_delta!(
        aurora.path(Vec2::new(705.0, 305.0), Vec2::new(217.0, 149.0)),
        524.068
    );

    assert_delta!(
        aurora.path(Vec2::new(665.0, 313.0), Vec2::new(151.0, 262.0)),
        528.171
    );

    assert_delta!(
        aurora.path(Vec2::new(583.0, 648.0), Vec2::new(968.0, 433.0)),
        522.613
    );

    assert_delta!(
        aurora.path(Vec2::new(529.0, 355.0), Vec2::new(50.0, 466.0)),
        520.719
    );

    assert_delta!(
        aurora.path(Vec2::new(862.0, 413.0), Vec2::new(520.0, 615.0)),
        522.422
    );

    assert_delta!(
        aurora.path(Vec2::new(518.0, 388.0), Vec2::new(38.0, 196.0)),
        524.49
    );

    assert_delta!(
        aurora.path(Vec2::new(783.0, 277.0), Vec2::new(298.0, 99.0)),
        526.083
    );

    assert_delta!(
        aurora.path(Vec2::new(616.0, 737.0), Vec2::new(369.0, 492.0)),
        526.081
    );

    assert_delta!(
        aurora.path(Vec2::new(324.0, 610.0), Vec2::new(789.0, 694.0)),
        523.457
    );

    assert_delta!(
        aurora.path(Vec2::new(143.0, 192.0), Vec2::new(250.0, 590.0)),
        530.199
    );

    assert_delta!(
        aurora.path(Vec2::new(415.0, 404.0), Vec2::new(92.0, 177.0)),
        522.84
    );

    assert_delta!(
        aurora.path(Vec2::new(288.0, 673.0), Vec2::new(653.0, 453.0)),
        531.232
    );

    assert_delta!(
        aurora.path(Vec2::new(480.0, 606.0), Vec2::new(517.0, 338.0)),
        534.49
    );

    assert_delta!(
        aurora.path(Vec2::new(123.0, 310.0), Vec2::new(542.0, 576.0)),
        536.104
    );

    assert_delta!(
        aurora.path(Vec2::new(830.0, 169.0), Vec2::new(299.0, 163.0)),
        541.875
    );

    assert_delta!(
        aurora.path(Vec2::new(307.0, 617.0), Vec2::new(404.0, 364.0)),
        530.291
    );

    assert_delta!(
        aurora.path(Vec2::new(908.0, 577.0), Vec2::new(384.0, 533.0)),
        534.947
    );

    assert_delta!(
        aurora.path(Vec2::new(673.0, 346.0), Vec2::new(702.0, 58.0)),
        528.087
    );

    assert_delta!(
        aurora.path(Vec2::new(241.0, 262.0), Vec2::new(243.0, 642.0)),
        532.829
    );

    assert_delta!(
        aurora.path(Vec2::new(521.0, 469.0), Vec2::new(31.0, 308.0)),
        534.416
    );

    assert_delta!(
        aurora.path(Vec2::new(441.0, 429.0), Vec2::new(822.0, 277.0)),
        526.544
    );

    assert_delta!(
        aurora.path(Vec2::new(288.0, 733.0), Vec2::new(759.0, 513.0)),
        539.269
    );

    assert_delta!(
        aurora.path(Vec2::new(586.0, 461.0), Vec2::new(344.0, 291.0)),
        528.907
    );

    assert_delta!(
        aurora.path(Vec2::new(132.0, 247.0), Vec2::new(645.0, 351.0)),
        534.687
    );

    assert_delta!(
        aurora.path(Vec2::new(443.0, 75.0), Vec2::new(811.0, 401.0)),
        531.169
    );

    assert_delta!(
        aurora.path(Vec2::new(502.0, 414.0), Vec2::new(1002.0, 400.0)),
        529.984
    );

    assert_delta!(
        aurora.path(Vec2::new(863.0, 284.0), Vec2::new(766.0, 690.0)),
        545.17
    );

    assert_delta!(
        aurora.path(Vec2::new(704.0, 421.0), Vec2::new(228.0, 360.0)),
        527.835
    );

    assert_delta!(
        aurora.path(Vec2::new(733.0, 598.0), Vec2::new(224.0, 541.0)),
        534.313
    );

    assert_delta!(
        aurora.path(Vec2::new(770.0, 225.0), Vec2::new(792.0, 542.0)),
        537.68
    );

    assert_delta!(
        aurora.path(Vec2::new(868.0, 361.0), Vec2::new(408.0, 136.0)),
        537.724
    );

    assert_delta!(
        aurora.path(Vec2::new(610.0, 435.0), Vec2::new(660.0, 71.0)),
        535.392
    );

    assert_delta!(
        aurora.path(Vec2::new(354.0, 461.0), Vec2::new(401.0, 153.0)),
        536.449
    );

    assert_delta!(
        aurora.path(Vec2::new(514.0, 445.0), Vec2::new(1009.0, 621.0)),
        528.697
    );

    assert_delta!(
        aurora.path(Vec2::new(834.0, 630.0), Vec2::new(550.0, 283.0)),
        530.87
    );

    assert_delta!(
        aurora.path(Vec2::new(245.0, 295.0), Vec2::new(52.0, 634.0)),
        539.564
    );

    assert_delta!(
        aurora.path(Vec2::new(396.0, 100.0), Vec2::new(137.0, 484.0)),
        539.577
    );

    assert_delta!(
        aurora.path(Vec2::new(418.0, 347.0), Vec2::new(216.0, 658.0)),
        541.428
    );

    assert_delta!(
        aurora.path(Vec2::new(685.0, 359.0), Vec2::new(280.0, 539.0)),
        536.787
    );

    assert_delta!(
        aurora.path(Vec2::new(229.0, 290.0), Vec2::new(555.0, 674.0)),
        548.793
    );

    assert_delta!(
        aurora.path(Vec2::new(421.0, 135.0), Vec2::new(75.0, 488.0)),
        533.52
    );

    assert_delta!(
        aurora.path(Vec2::new(679.0, 703.0), Vec2::new(212.0, 639.0)),
        543.561
    );

    assert_delta!(
        aurora.path(Vec2::new(477.0, 203.0), Vec2::new(944.0, 390.0)),
        532.576
    );

    assert_delta!(
        aurora.path(Vec2::new(57.0, 154.0), Vec2::new(592.0, 192.0)),
        548.47
    );

    assert_delta!(
        aurora.path(Vec2::new(315.0, 635.0), Vec2::new(396.0, 358.0)),
        536.34
    );

    assert_delta!(
        aurora.path(Vec2::new(36.0, 362.0), Vec2::new(369.0, 524.0)),
        546.326
    );

    assert_delta!(
        aurora.path(Vec2::new(277.0, 507.0), Vec2::new(577.0, 400.0)),
        540.861
    );

    assert_delta!(
        aurora.path(Vec2::new(907.0, 585.0), Vec2::new(396.0, 680.0)),
        548.847
    );

    assert_delta!(
        aurora.path(Vec2::new(116.0, 400.0), Vec2::new(491.0, 663.0)),
        535.653
    );

    assert_delta!(
        aurora.path(Vec2::new(43.0, 383.0), Vec2::new(421.0, 673.0)),
        540.353
    );

    assert_delta!(
        aurora.path(Vec2::new(643.0, 294.0), Vec2::new(476.0, 636.0)),
        540.244
    );

    assert_delta!(
        aurora.path(Vec2::new(916.0, 624.0), Vec2::new(589.0, 269.0)),
        541.102
    );

    assert_delta!(
        aurora.path(Vec2::new(248.0, 482.0), Vec2::new(376.0, 165.0)),
        541.162
    );

    assert_delta!(
        aurora.path(Vec2::new(426.0, 518.0), Vec2::new(835.0, 378.0)),
        544.14
    );

    assert_delta!(
        aurora.path(Vec2::new(504.0, 265.0), Vec2::new(733.0, 565.0)),
        540.685
    );

    assert_delta!(
        aurora.path(Vec2::new(17.0, 390.0), Vec2::new(555.0, 326.0)),
        550.582
    );

    assert_delta!(
        aurora.path(Vec2::new(11.0, 321.0), Vec2::new(329.0, 690.0)),
        552.94
    );

    assert_delta!(
        aurora.path(Vec2::new(207.0, 712.0), Vec2::new(126.0, 295.0)),
        556.698
    );

    assert_delta!(
        aurora.path(Vec2::new(440.0, 685.0), Vec2::new(951.0, 575.0)),
        550.166
    );

    assert_delta!(
        aurora.path(Vec2::new(95.0, 534.0), Vec2::new(348.0, 738.0)),
        537.337
    );

    assert_delta!(
        aurora.path(Vec2::new(950.0, 565.0), Vec2::new(417.0, 471.0)),
        546.343
    );

    assert_delta!(
        aurora.path(Vec2::new(786.0, 395.0), Vec2::new(318.0, 376.0)),
        539.337
    );

    assert_delta!(
        aurora.path(Vec2::new(947.0, 285.0), Vec2::new(845.0, 702.0)),
        544.114
    );

    assert_delta!(
        aurora.path(Vec2::new(457.0, 618.0), Vec2::new(630.0, 293.0)),
        545.544
    );

    assert_delta!(
        aurora.path(Vec2::new(324.0, 554.0), Vec2::new(571.0, 262.0)),
        546.455
    );

    assert_delta!(
        aurora.path(Vec2::new(378.0, 370.0), Vec2::new(692.0, 575.0)),
        546.046
    );

    assert_delta!(
        aurora.path(Vec2::new(354.0, 360.0), Vec2::new(752.0, 138.0)),
        547.526
    );

    assert_delta!(
        aurora.path(Vec2::new(359.0, 510.0), Vec2::new(16.0, 580.0)),
        550.431
    );

    assert_delta!(
        aurora.path(Vec2::new(694.0, 273.0), Vec2::new(1011.0, 600.0)),
        552.206
    );

    assert_delta!(
        aurora.path(Vec2::new(255.0, 228.0), Vec2::new(573.0, 427.0)),
        554.65
    );

    assert_delta!(
        aurora.path(Vec2::new(933.0, 616.0), Vec2::new(643.0, 254.0)),
        556.515
    );

    assert_delta!(
        aurora.path(Vec2::new(771.0, 503.0), Vec2::new(270.0, 643.0)),
        563.121
    );

    assert_delta!(
        aurora.path(Vec2::new(16.0, 463.0), Vec2::new(437.0, 161.0)),
        546.851
    );

    assert_delta!(
        aurora.path(Vec2::new(929.0, 354.0), Vec2::new(610.0, 667.0)),
        552.821
    );

    assert_delta!(
        aurora.path(Vec2::new(277.0, 250.0), Vec2::new(780.0, 299.0)),
        547.361
    );

    assert_delta!(
        aurora.path(Vec2::new(816.0, 275.0), Vec2::new(425.0, 543.0)),
        557.816
    );

    assert_delta!(
        aurora.path(Vec2::new(346.0, 176.0), Vec2::new(853.0, 315.0)),
        546.587
    );

    assert_delta!(
        aurora.path(Vec2::new(207.0, 315.0), Vec2::new(154.0, 729.0)),
        557.65
    );

    assert_delta!(
        aurora.path(Vec2::new(1010.0, 446.0), Vec2::new(473.0, 469.0)),
        554.594
    );

    assert_delta!(
        aurora.path(Vec2::new(397.0, 499.0), Vec2::new(60.0, 216.0)),
        552.321
    );

    assert_delta!(
        aurora.path(Vec2::new(178.0, 410.0), Vec2::new(677.0, 403.0)),
        548.615
    );

    assert_delta!(
        aurora.path(Vec2::new(905.0, 425.0), Vec2::new(567.0, 123.0)),
        552.717
    );

    assert_delta!(
        aurora.path(Vec2::new(155.0, 529.0), Vec2::new(35.0, 152.0)),
        554.842
    );

    assert_delta!(
        aurora.path(Vec2::new(885.0, 398.0), Vec2::new(481.0, 628.0)),
        555.473
    );

    assert_delta!(
        aurora.path(Vec2::new(6.0, 334.0), Vec2::new(241.0, 730.0)),
        561.718
    );

    assert_delta!(
        aurora.path(Vec2::new(529.0, 37.0), Vec2::new(911.0, 300.0)),
        551.075
    );

    assert_delta!(
        aurora.path(Vec2::new(23.0, 478.0), Vec2::new(577.0, 477.0)),
        564.11
    );

    assert_delta!(
        aurora.path(Vec2::new(471.0, 387.0), Vec2::new(271.0, 609.0)),
        558.753
    );

    assert_delta!(
        aurora.path(Vec2::new(35.0, 665.0), Vec2::new(122.0, 234.0)),
        564.869
    );

    assert_delta!(
        aurora.path(Vec2::new(727.0, 551.0), Vec2::new(817.0, 197.0)),
        561.595
    );

    assert_delta!(
        aurora.path(Vec2::new(858.0, 502.0), Vec2::new(739.0, 263.0)),
        559.491
    );

    assert_delta!(
        aurora.path(Vec2::new(742.0, 701.0), Vec2::new(952.0, 309.0)),
        564.684
    );

    assert_delta!(
        aurora.path(Vec2::new(262.0, 234.0), Vec2::new(789.0, 290.0)),
        556.218
    );

    assert_delta!(
        aurora.path(Vec2::new(249.0, 42.0), Vec2::new(731.0, 300.0)),
        553.291
    );

    assert_delta!(
        aurora.path(Vec2::new(768.0, 684.0), Vec2::new(345.0, 472.0)),
        555.376
    );

    assert_delta!(
        aurora.path(Vec2::new(445.0, 68.0), Vec2::new(11.0, 321.0)),
        559.391
    );

    assert_delta!(
        aurora.path(Vec2::new(949.0, 273.0), Vec2::new(421.0, 130.0)),
        558.092
    );

    assert_delta!(
        aurora.path(Vec2::new(890.0, 356.0), Vec2::new(594.0, 711.0)),
        561.182
    );

    assert_delta!(
        aurora.path(Vec2::new(574.0, 635.0), Vec2::new(259.0, 253.0)),
        569.436
    );

    assert_delta!(
        aurora.path(Vec2::new(287.0, 121.0), Vec2::new(829.0, 180.0)),
        563.879
    );

    assert_delta!(
        aurora.path(Vec2::new(721.0, 279.0), Vec2::new(830.0, 517.0)),
        559.339
    );

    assert_delta!(
        aurora.path(Vec2::new(801.0, 636.0), Vec2::new(271.0, 645.0)),
        566.869
    );

    assert_delta!(
        aurora.path(Vec2::new(678.0, 703.0), Vec2::new(937.0, 295.0)),
        561.242
    );

    assert_delta!(
        aurora.path(Vec2::new(851.0, 222.0), Vec2::new(912.0, 615.0)),
        562.728
    );

    assert_delta!(
        aurora.path(Vec2::new(772.0, 195.0), Vec2::new(217.0, 202.0)),
        574.995
    );

    assert_delta!(
        aurora.path(Vec2::new(473.0, 228.0), Vec2::new(934.0, 462.0)),
        560.844
    );

    assert_delta!(
        aurora.path(Vec2::new(560.0, 645.0), Vec2::new(16.0, 589.0)),
        571.298
    );

    assert_delta!(
        aurora.path(Vec2::new(585.0, 671.0), Vec2::new(444.0, 302.0)),
        567.969
    );

    assert_delta!(
        aurora.path(Vec2::new(730.0, 396.0), Vec2::new(330.0, 235.0)),
        563.182
    );

    assert_delta!(
        aurora.path(Vec2::new(94.0, 380.0), Vec2::new(626.0, 389.0)),
        563.028
    );

    assert_delta!(
        aurora.path(Vec2::new(441.0, 465.0), Vec2::new(893.0, 378.0)),
        569.485
    );

    assert_delta!(
        aurora.path(Vec2::new(421.0, 626.0), Vec2::new(303.0, 254.0)),
        567.494
    );

    assert_delta!(
        aurora.path(Vec2::new(361.0, 221.0), Vec2::new(209.0, 533.0)),
        561.595
    );

    assert_delta!(
        aurora.path(Vec2::new(230.0, 212.0), Vec2::new(323.0, 527.0)),
        565.73
    );

    assert_delta!(
        aurora.path(Vec2::new(9.0, 476.0), Vec2::new(213.0, 61.0)),
        567.751
    );

    assert_delta!(
        aurora.path(Vec2::new(916.0, 280.0), Vec2::new(407.0, 93.0)),
        563.881
    );

    assert_delta!(
        aurora.path(Vec2::new(475.0, 642.0), Vec2::new(498.0, 395.0)),
        573.881
    );

    assert_delta!(
        aurora.path(Vec2::new(668.0, 259.0), Vec2::new(868.0, 694.0)),
        573.343
    );

    assert_delta!(
        aurora.path(Vec2::new(551.0, 578.0), Vec2::new(919.0, 264.0)),
        568.176
    );

    assert_delta!(
        aurora.path(Vec2::new(725.0, 586.0), Vec2::new(199.0, 588.0)),
        567.395
    );

    assert_delta!(
        aurora.path(Vec2::new(680.0, 434.0), Vec2::new(355.0, 205.0)),
        570.267
    );

    assert_delta!(
        aurora.path(Vec2::new(772.0, 353.0), Vec2::new(640.0, 691.0)),
        571.901
    );

    assert_delta!(
        aurora.path(Vec2::new(584.0, 657.0), Vec2::new(51.0, 503.0)),
        566.313
    );

    assert_delta!(
        aurora.path(Vec2::new(39.0, 426.0), Vec2::new(561.0, 265.0)),
        568.842
    );

    assert_delta!(
        aurora.path(Vec2::new(678.0, 72.0), Vec2::new(1013.0, 427.0)),
        573.34
    );

    assert_delta!(
        aurora.path(Vec2::new(17.0, 450.0), Vec2::new(512.0, 418.0)),
        574.169
    );

    assert_delta!(
        aurora.path(Vec2::new(263.0, 502.0), Vec2::new(799.0, 654.0)),
        570.491
    );

    assert_delta!(
        aurora.path(Vec2::new(245.0, 355.0), Vec2::new(793.0, 354.0)),
        576.308
    );

    assert_delta!(
        aurora.path(Vec2::new(193.0, 232.0), Vec2::new(739.0, 241.0)),
        574.394
    );

    assert_delta!(
        aurora.path(Vec2::new(944.0, 573.0), Vec2::new(508.0, 279.0)),
        566.158
    );

    assert_delta!(
        aurora.path(Vec2::new(21.0, 320.0), Vec2::new(563.0, 389.0)),
        573.278
    );

    assert_delta!(
        aurora.path(Vec2::new(454.0, 420.0), Vec2::new(653.0, 193.0)),
        565.703
    );

    assert_delta!(
        aurora.path(Vec2::new(503.0, 246.0), Vec2::new(711.0, 563.0)),
        570.109
    );

    assert_delta!(
        aurora.path(Vec2::new(582.0, 588.0), Vec2::new(48.0, 561.0)),
        574.379
    );

    assert_delta!(
        aurora.path(Vec2::new(12.0, 404.0), Vec2::new(435.0, 109.0)),
        571.821
    );

    assert_delta!(
        aurora.path(Vec2::new(452.0, 336.0), Vec2::new(168.0, 88.0)),
        572.912
    );

    assert_delta!(
        aurora.path(Vec2::new(605.0, 643.0), Vec2::new(186.0, 408.0)),
        571.507
    );

    assert_delta!(
        aurora.path(Vec2::new(86.0, 134.0), Vec2::new(634.0, 50.0)),
        572.338
    );

    assert_delta!(
        aurora.path(Vec2::new(482.0, 296.0), Vec2::new(751.0, 593.0)),
        571.192
    );

    assert_delta!(
        aurora.path(Vec2::new(812.0, 279.0), Vec2::new(400.0, 544.0)),
        579.393
    );

    assert_delta!(
        aurora.path(Vec2::new(720.0, 413.0), Vec2::new(311.0, 598.0)),
        573.382
    );

    assert_delta!(
        aurora.path(Vec2::new(828.0, 212.0), Vec2::new(299.0, 241.0)),
        579.497
    );

    assert_delta!(
        aurora.path(Vec2::new(117.0, 326.0), Vec2::new(628.0, 226.0)),
        580.356
    );

    assert_delta!(
        aurora.path(Vec2::new(249.0, 203.0), Vec2::new(303.0, 543.0)),
        577.132
    );

    assert_delta!(
        aurora.path(Vec2::new(430.0, 455.0), Vec2::new(887.0, 403.0)),
        575.539
    );

    assert_delta!(
        aurora.path(Vec2::new(685.0, 416.0), Vec2::new(313.0, 474.0)),
        572.599
    );

    assert_delta!(
        aurora.path(Vec2::new(512.0, 573.0), Vec2::new(392.0, 272.0)),
        590.156
    );

    assert_delta!(
        aurora.path(Vec2::new(145.0, 647.0), Vec2::new(692.0, 578.0)),
        593.825
    );

    assert_delta!(
        aurora.path(Vec2::new(933.0, 141.0), Vec2::new(604.0, 489.0)),
        579.788
    );

    assert_delta!(
        aurora.path(Vec2::new(581.0, 642.0), Vec2::new(883.0, 262.0)),
        577.465
    );

    assert_delta!(
        aurora.path(Vec2::new(587.0, 460.0), Vec2::new(702.0, 76.0)),
        572.758
    );

    assert_delta!(
        aurora.path(Vec2::new(81.0, 215.0), Vec2::new(614.0, 379.0)),
        575.757
    );

    assert_delta!(
        aurora.path(Vec2::new(315.0, 53.0), Vec2::new(153.0, 513.0)),
        580.12
    );

    assert_delta!(
        aurora.path(Vec2::new(612.0, 421.0), Vec2::new(1014.0, 398.0)),
        572.609
    );

    assert_delta!(
        aurora.path(Vec2::new(835.0, 605.0), Vec2::new(276.0, 634.0)),
        590.835
    );

    assert_delta!(
        aurora.path(Vec2::new(458.0, 449.0), Vec2::new(975.0, 404.0)),
        578.789
    );

    assert_delta!(
        aurora.path(Vec2::new(701.0, 592.0), Vec2::new(419.0, 382.0)),
        576.842
    );

    assert_delta!(
        aurora.path(Vec2::new(355.0, 168.0), Vec2::new(684.0, 451.0)),
        583.349
    );

    assert_delta!(
        aurora.path(Vec2::new(678.0, 449.0), Vec2::new(805.0, 89.0)),
        585.723
    );

    assert_delta!(
        aurora.path(Vec2::new(256.0, 633.0), Vec2::new(303.0, 290.0)),
        583.831
    );

    assert_delta!(
        aurora.path(Vec2::new(345.0, 553.0), Vec2::new(615.0, 323.0)),
        586.466
    );

    assert_delta!(
        aurora.path(Vec2::new(153.0, 150.0), Vec2::new(728.0, 54.0)),
        595.294
    );

    assert_delta!(
        aurora.path(Vec2::new(506.0, 373.0), Vec2::new(556.0, 684.0)),
        578.543
    );

    assert_delta!(
        aurora.path(Vec2::new(531.0, 289.0), Vec2::new(971.0, 594.0)),
        575.598
    );

    assert_delta!(
        aurora.path(Vec2::new(591.0, 119.0), Vec2::new(808.0, 441.0)),
        578.632
    );

    assert_delta!(
        aurora.path(Vec2::new(651.0, 426.0), Vec2::new(765.0, 77.0)),
        584.352
    );

    assert_delta!(
        aurora.path(Vec2::new(136.0, 233.0), Vec2::new(646.0, 68.0)),
        586.74
    );

    assert_delta!(
        aurora.path(Vec2::new(362.0, 218.0), Vec2::new(552.0, 566.0)),
        594.669
    );

    assert_delta!(
        aurora.path(Vec2::new(248.0, 79.0), Vec2::new(644.0, 400.0)),
        584.427
    );

    assert_delta!(
        aurora.path(Vec2::new(68.0, 349.0), Vec2::new(632.0, 293.0)),
        585.535
    );

    assert_delta!(
        aurora.path(Vec2::new(649.0, 140.0), Vec2::new(472.0, 425.0)),
        581.048
    );

    assert_delta!(
        aurora.path(Vec2::new(797.0, 297.0), Vec2::new(275.0, 68.0)),
        585.36
    );

    assert_delta!(
        aurora.path(Vec2::new(89.0, 418.0), Vec2::new(655.0, 319.0)),
        590.762
    );

    assert_delta!(
        aurora.path(Vec2::new(112.0, 151.0), Vec2::new(610.0, 293.0)),
        595.009
    );

    assert_delta!(
        aurora.path(Vec2::new(158.0, 312.0), Vec2::new(461.0, 660.0)),
        585.628
    );

    assert_delta!(
        aurora.path(Vec2::new(676.0, 598.0), Vec2::new(464.0, 260.0)),
        590.072
    );

    assert_delta!(
        aurora.path(Vec2::new(263.0, 350.0), Vec2::new(836.0, 319.0)),
        592.106
    );

    assert_delta!(
        aurora.path(Vec2::new(565.0, 493.0), Vec2::new(745.0, 213.0)),
        589.114
    );

    assert_delta!(
        aurora.path(Vec2::new(222.0, 366.0), Vec2::new(685.0, 84.0)),
        581.74
    );

    assert_delta!(
        aurora.path(Vec2::new(497.0, 646.0), Vec2::new(952.0, 420.0)),
        589.055
    );

    assert_delta!(
        aurora.path(Vec2::new(414.0, 436.0), Vec2::new(685.0, 117.0)),
        588.387
    );

    assert_delta!(
        aurora.path(Vec2::new(612.0, 230.0), Vec2::new(673.0, 630.0)),
        591.951
    );

    assert_delta!(
        aurora.path(Vec2::new(240.0, 678.0), Vec2::new(458.0, 291.0)),
        594.041
    );

    assert_delta!(
        aurora.path(Vec2::new(244.0, 485.0), Vec2::new(810.0, 443.0)),
        587.47
    );

    assert_delta!(
        aurora.path(Vec2::new(518.0, 167.0), Vec2::new(64.0, 477.0)),
        587.684
    );

    assert_delta!(
        aurora.path(Vec2::new(231.0, 427.0), Vec2::new(690.0, 669.0)),
        592.132
    );

    assert_delta!(
        aurora.path(Vec2::new(211.0, 235.0), Vec2::new(715.0, 350.0)),
        594.601
    );

    assert_delta!(
        aurora.path(Vec2::new(855.0, 521.0), Vec2::new(317.0, 328.0)),
        594.65
    );

    assert_delta!(
        aurora.path(Vec2::new(638.0, 612.0), Vec2::new(454.0, 284.0)),
        595.112
    );

    assert_delta!(
        aurora.path(Vec2::new(324.0, 468.0), Vec2::new(897.0, 571.0)),
        593.818
    );

    assert_delta!(
        aurora.path(Vec2::new(392.0, 637.0), Vec2::new(231.0, 291.0)),
        593.518
    );

    assert_delta!(
        aurora.path(Vec2::new(236.0, 419.0), Vec2::new(790.0, 422.0)),
        590.363
    );

    assert_delta!(
        aurora.path(Vec2::new(451.0, 507.0), Vec2::new(987.0, 406.0)),
        595.91
    );

    assert_delta!(
        aurora.path(Vec2::new(409.0, 382.0), Vec2::new(784.0, 614.0)),
        596.678
    );

    assert_delta!(
        aurora.path(Vec2::new(40.0, 473.0), Vec2::new(379.0, 93.0)),
        593.616
    );

    assert_delta!(
        aurora.path(Vec2::new(275.0, 530.0), Vec2::new(626.0, 294.0)),
        600.581
    );

    assert_delta!(
        aurora.path(Vec2::new(627.0, 589.0), Vec2::new(179.0, 405.0)),
        599.901
    );

    assert_delta!(
        aurora.path(Vec2::new(299.0, 152.0), Vec2::new(906.0, 145.0)),
        613.826
    );

    assert_delta!(
        aurora.path(Vec2::new(356.0, 166.0), Vec2::new(661.0, 441.0)),
        595.041
    );

    assert_delta!(
        aurora.path(Vec2::new(129.0, 538.0), Vec2::new(527.0, 276.0)),
        596.105
    );

    assert_delta!(
        aurora.path(Vec2::new(413.0, 419.0), Vec2::new(868.0, 297.0)),
        593.492
    );

    assert_delta!(
        aurora.path(Vec2::new(334.0, 459.0), Vec2::new(793.0, 286.0)),
        600.659
    );

    assert_delta!(
        aurora.path(Vec2::new(554.0, 714.0), Vec2::new(9.0, 541.0)),
        590.835
    );

    assert_delta!(
        aurora.path(Vec2::new(642.0, 516.0), Vec2::new(806.0, 185.0)),
        596.485
    );

    assert_delta!(
        aurora.path(Vec2::new(162.0, 376.0), Vec2::new(634.0, 234.0)),
        594.587
    );

    assert_delta!(
        aurora.path(Vec2::new(459.0, 590.0), Vec2::new(985.0, 365.0)),
        603.568
    );

    assert_delta!(
        aurora.path(Vec2::new(297.0, 158.0), Vec2::new(737.0, 404.0)),
        601.47
    );

    assert_delta!(
        aurora.path(Vec2::new(477.0, 225.0), Vec2::new(35.0, 543.0)),
        607.805
    );

    assert_delta!(
        aurora.path(Vec2::new(504.0, 319.0), Vec2::new(118.0, 593.0)),
        602.211
    );

    assert_delta!(
        aurora.path(Vec2::new(902.0, 540.0), Vec2::new(357.0, 354.0)),
        598.287
    );

    assert_delta!(
        aurora.path(Vec2::new(827.0, 678.0), Vec2::new(460.0, 383.0)),
        605.301
    );

    assert_delta!(
        aurora.path(Vec2::new(412.0, 453.0), Vec2::new(692.0, 150.0)),
        604.084
    );

    assert_delta!(
        aurora.path(Vec2::new(855.0, 236.0), Vec2::new(586.0, 606.0)),
        603.812
    );

    assert_delta!(
        aurora.path(Vec2::new(233.0, 323.0), Vec2::new(422.0, 650.0)),
        598.005
    );

    assert_delta!(
        aurora.path(Vec2::new(906.0, 161.0), Vec2::new(753.0, 566.0)),
        603.828
    );

    assert_delta!(
        aurora.path(Vec2::new(468.0, 584.0), Vec2::new(280.0, 199.0)),
        614.314
    );

    assert_delta!(
        aurora.path(Vec2::new(120.0, 698.0), Vec2::new(424.0, 298.0)),
        610.808
    );

    assert_delta!(
        aurora.path(Vec2::new(962.0, 389.0), Vec2::new(427.0, 191.0)),
        599.377
    );

    assert_delta!(
        aurora.path(Vec2::new(512.0, 170.0), Vec2::new(480.0, 595.0)),
        607.774
    );

    assert_delta!(
        aurora.path(Vec2::new(611.0, 658.0), Vec2::new(494.0, 282.0)),
        604.497
    );

    assert_delta!(
        aurora.path(Vec2::new(782.0, 221.0), Vec2::new(203.0, 68.0)),
        603.645
    );

    assert_delta!(
        aurora.path(Vec2::new(911.0, 345.0), Vec2::new(564.0, 698.0)),
        603.986
    );

    assert_delta!(
        aurora.path(Vec2::new(352.0, 267.0), Vec2::new(506.0, 633.0)),
        613.012
    );

    assert_delta!(
        aurora.path(Vec2::new(265.0, 408.0), Vec2::new(793.0, 622.0)),
        599.604
    );

    assert_delta!(
        aurora.path(Vec2::new(705.0, 538.0), Vec2::new(176.0, 410.0)),
        599.187
    );

    assert_delta!(
        aurora.path(Vec2::new(548.0, 454.0), Vec2::new(32.0, 490.0)),
        610.232
    );

    assert_delta!(
        aurora.path(Vec2::new(572.0, 131.0), Vec2::new(522.0, 519.0)),
        612.35
    );

    assert_delta!(
        aurora.path(Vec2::new(708.0, 631.0), Vec2::new(139.0, 727.0)),
        616.802
    );

    assert_delta!(
        aurora.path(Vec2::new(580.0, 412.0), Vec2::new(173.0, 182.0)),
        605.375
    );

    assert_delta!(
        aurora.path(Vec2::new(622.0, 246.0), Vec2::new(694.0, 672.0)),
        613.922
    );

    assert_delta!(
        aurora.path(Vec2::new(899.0, 313.0), Vec2::new(366.0, 67.0)),
        607.405
    );

    assert_delta!(
        aurora.path(Vec2::new(391.0, 234.0), Vec2::new(210.0, 552.0)),
        606.426
    );

    assert_delta!(
        aurora.path(Vec2::new(736.0, 318.0), Vec2::new(148.0, 391.0)),
        612.109
    );

    assert_delta!(
        aurora.path(Vec2::new(222.0, 472.0), Vec2::new(803.0, 563.0)),
        608.266
    );

    assert_delta!(
        aurora.path(Vec2::new(415.0, 668.0), Vec2::new(55.0, 317.0)),
        610.401
    );

    assert_delta!(
        aurora.path(Vec2::new(358.0, 266.0), Vec2::new(873.0, 423.0)),
        612.259
    );

    assert_delta!(
        aurora.path(Vec2::new(46.0, 398.0), Vec2::new(649.0, 323.0)),
        620.905
    );

    assert_delta!(
        aurora.path(Vec2::new(110.0, 513.0), Vec2::new(705.0, 564.0)),
        616.64
    );

    assert_delta!(
        aurora.path(Vec2::new(203.0, 268.0), Vec2::new(766.0, 249.0)),
        613.793
    );

    assert_delta!(
        aurora.path(Vec2::new(641.0, 655.0), Vec2::new(42.0, 668.0)),
        619.047
    );

    assert_delta!(
        aurora.path(Vec2::new(252.0, 405.0), Vec2::new(826.0, 285.0)),
        620.896
    );

    assert_delta!(
        aurora.path(Vec2::new(692.0, 189.0), Vec2::new(217.0, 416.0)),
        613.451
    );

    assert_delta!(
        aurora.path(Vec2::new(308.0, 294.0), Vec2::new(265.0, 657.0)),
        614.687
    );

    assert_delta!(
        aurora.path(Vec2::new(676.0, 699.0), Vec2::new(885.0, 252.0)),
        616.443
    );

    assert_delta!(
        aurora.path(Vec2::new(722.0, 428.0), Vec2::new(295.0, 150.0)),
        609.251
    );

    assert_delta!(
        aurora.path(Vec2::new(795.0, 103.0), Vec2::new(196.0, 229.0)),
        618.207
    );

    assert_delta!(
        aurora.path(Vec2::new(513.0, 614.0), Vec2::new(560.0, 218.0)),
        620.518
    );

    assert_delta!(
        aurora.path(Vec2::new(668.0, 601.0), Vec2::new(177.0, 283.0)),
        611.961
    );

    assert_delta!(
        aurora.path(Vec2::new(894.0, 469.0), Vec2::new(453.0, 121.0)),
        615.485
    );

    assert_delta!(
        aurora.path(Vec2::new(627.0, 166.0), Vec2::new(420.0, 515.0)),
        620.628
    );

    assert_delta!(
        aurora.path(Vec2::new(665.0, 271.0), Vec2::new(235.0, 512.0)),
        621.887
    );

    assert_delta!(
        aurora.path(Vec2::new(142.0, 272.0), Vec2::new(662.0, 564.0)),
        613.323
    );

    assert_delta!(
        aurora.path(Vec2::new(793.0, 624.0), Vec2::new(394.0, 300.0)),
        612.886
    );

    assert_delta!(
        aurora.path(Vec2::new(282.0, 344.0), Vec2::new(888.0, 274.0)),
        626.291
    );

    assert_delta!(
        aurora.path(Vec2::new(974.0, 283.0), Vec2::new(595.0, 666.0)),
        618.25
    );

    assert_delta!(
        aurora.path(Vec2::new(725.0, 140.0), Vec2::new(210.0, 368.0)),
        618.698
    );

    assert_delta!(
        aurora.path(Vec2::new(511.0, 661.0), Vec2::new(526.0, 277.0)),
        619.247
    );

    assert_delta!(
        aurora.path(Vec2::new(450.0, 596.0), Vec2::new(882.0, 252.0)),
        629.061
    );

    assert_delta!(
        aurora.path(Vec2::new(511.0, 341.0), Vec2::new(890.0, 667.0)),
        619.387
    );

    assert_delta!(
        aurora.path(Vec2::new(442.0, 316.0), Vec2::new(686.0, 634.0)),
        625.555
    );

    assert_delta!(
        aurora.path(Vec2::new(8.0, 375.0), Vec2::new(550.0, 652.0)),
        615.799
    );

    assert_delta!(
        aurora.path(Vec2::new(814.0, 420.0), Vec2::new(308.0, 268.0)),
        621.068
    );

    assert_delta!(
        aurora.path(Vec2::new(363.0, 115.0), Vec2::new(659.0, 456.0)),
        622.082
    );

    assert_delta!(
        aurora.path(Vec2::new(794.0, 409.0), Vec2::new(318.0, 487.0)),
        617.266
    );

    assert_delta!(
        aurora.path(Vec2::new(419.0, 391.0), Vec2::new(866.0, 132.0)),
        629.989
    );

    assert_delta!(
        aurora.path(Vec2::new(131.0, 116.0), Vec2::new(730.0, 60.0)),
        623.167
    );

    assert_delta!(
        aurora.path(Vec2::new(640.0, 665.0), Vec2::new(877.0, 255.0)),
        625.475
    );

    assert_delta!(
        aurora.path(Vec2::new(578.0, 434.0), Vec2::new(16.0, 187.0)),
        623.406
    );

    assert_delta!(
        aurora.path(Vec2::new(234.0, 220.0), Vec2::new(836.0, 258.0)),
        633.422
    );

    assert_delta!(
        aurora.path(Vec2::new(587.0, 661.0), Vec2::new(939.0, 265.0)),
        623.862
    );

    assert_delta!(
        aurora.path(Vec2::new(609.0, 453.0), Vec2::new(615.0, 742.0)),
        633.342
    );

    assert_delta!(
        aurora.path(Vec2::new(131.0, 571.0), Vec2::new(316.0, 151.0)),
        628.187
    );

    assert_delta!(
        aurora.path(Vec2::new(210.0, 143.0), Vec2::new(598.0, 424.0)),
        624.858
    );

    assert_delta!(
        aurora.path(Vec2::new(476.0, 253.0), Vec2::new(312.0, 667.0)),
        625.925
    );

    assert_delta!(
        aurora.path(Vec2::new(317.0, 156.0), Vec2::new(90.0, 555.0)),
        626.77
    );

    assert_delta!(
        aurora.path(Vec2::new(257.0, 494.0), Vec2::new(747.0, 317.0)),
        628.169
    );

    assert_delta!(
        aurora.path(Vec2::new(726.0, 614.0), Vec2::new(134.0, 606.0)),
        632.218
    );

    assert_delta!(
        aurora.path(Vec2::new(850.0, 303.0), Vec2::new(261.0, 396.0)),
        635.639
    );

    assert_delta!(
        aurora.path(Vec2::new(744.0, 366.0), Vec2::new(245.0, 474.0)),
        625.91
    );

    assert_delta!(
        aurora.path(Vec2::new(171.0, 310.0), Vec2::new(726.0, 554.0)),
        624.069
    );

    assert_delta!(
        aurora.path(Vec2::new(366.0, 293.0), Vec2::new(843.0, 592.0)),
        624.303
    );

    assert_delta!(
        aurora.path(Vec2::new(866.0, 373.0), Vec2::new(457.0, 660.0)),
        633.782
    );

    assert_delta!(
        aurora.path(Vec2::new(706.0, 104.0), Vec2::new(85.0, 123.0)),
        643.778
    );

    assert_delta!(
        aurora.path(Vec2::new(901.0, 552.0), Vec2::new(285.0, 513.0)),
        631.642
    );

    assert_delta!(
        aurora.path(Vec2::new(29.0, 538.0), Vec2::new(558.0, 415.0)),
        629.094
    );

    assert_delta!(
        aurora.path(Vec2::new(375.0, 194.0), Vec2::new(448.0, 590.0)),
        642.373
    );

    assert_delta!(
        aurora.path(Vec2::new(256.0, 496.0), Vec2::new(675.0, 412.0)),
        622.852
    );

    assert_delta!(
        aurora.path(Vec2::new(651.0, 111.0), Vec2::new(858.0, 488.0)),
        629.438
    );

    assert_delta!(
        aurora.path(Vec2::new(429.0, 551.0), Vec2::new(248.0, 134.0)),
        642.785
    );

    assert_delta!(
        aurora.path(Vec2::new(206.0, 138.0), Vec2::new(271.0, 582.0)),
        633.297
    );

    assert_delta!(
        aurora.path(Vec2::new(714.0, 173.0), Vec2::new(68.0, 138.0)),
        654.47
    );

    assert_delta!(
        aurora.path(Vec2::new(903.0, 594.0), Vec2::new(537.0, 196.0)),
        632.631
    );

    assert_delta!(
        aurora.path(Vec2::new(638.0, 561.0), Vec2::new(66.0, 489.0)),
        635.457
    );

    assert_delta!(
        aurora.path(Vec2::new(845.0, 289.0), Vec2::new(634.0, 739.0)),
        641.438
    );

    assert_delta!(
        aurora.path(Vec2::new(624.0, 519.0), Vec2::new(37.0, 399.0)),
        629.169
    );

    assert_delta!(
        aurora.path(Vec2::new(228.0, 586.0), Vec2::new(387.0, 237.0)),
        631.991
    );

    assert_delta!(
        aurora.path(Vec2::new(148.0, 488.0), Vec2::new(767.0, 420.0)),
        638.24
    );

    assert_delta!(
        aurora.path(Vec2::new(474.0, 303.0), Vec2::new(207.0, 700.0)),
        643.417
    );

    assert_delta!(
        aurora.path(Vec2::new(20.0, 403.0), Vec2::new(624.0, 473.0)),
        635.691
    );

    assert_delta!(
        aurora.path(Vec2::new(279.0, 291.0), Vec2::new(638.0, 601.0)),
        629.479
    );

    assert_delta!(
        aurora.path(Vec2::new(127.0, 488.0), Vec2::new(715.0, 390.0)),
        639.701
    );

    assert_delta!(
        aurora.path(Vec2::new(764.0, 364.0), Vec2::new(251.0, 56.0)),
        631.746
    );

    assert_delta!(
        aurora.path(Vec2::new(484.0, 399.0), Vec2::new(702.0, 702.0)),
        639.465
    );

    assert_delta!(
        aurora.path(Vec2::new(876.0, 296.0), Vec2::new(275.0, 389.0)),
        642.142
    );

    assert_delta!(
        aurora.path(Vec2::new(112.0, 521.0), Vec2::new(481.0, 212.0)),
        642.266
    );

    assert_delta!(
        aurora.path(Vec2::new(316.0, 676.0), Vec2::new(847.0, 516.0)),
        640.527
    );

    assert_delta!(
        aurora.path(Vec2::new(126.0, 361.0), Vec2::new(702.0, 168.0)),
        653.175
    );

    assert_delta!(
        aurora.path(Vec2::new(383.0, 632.0), Vec2::new(279.0, 250.0)),
        636.415
    );

    assert_delta!(
        aurora.path(Vec2::new(238.0, 716.0), Vec2::new(821.0, 495.0)),
        648.683
    );

    assert_delta!(
        aurora.path(Vec2::new(629.0, 591.0), Vec2::new(5.0, 648.0)),
        661.471
    );

    assert_delta!(
        aurora.path(Vec2::new(303.0, 263.0), Vec2::new(109.0, 669.0)),
        645.588
    );

    assert_delta!(
        aurora.path(Vec2::new(837.0, 100.0), Vec2::new(304.0, 324.0)),
        634.713
    );

    assert_delta!(
        aurora.path(Vec2::new(350.0, 454.0), Vec2::new(930.0, 654.0)),
        635.923
    );

    assert_delta!(
        aurora.path(Vec2::new(51.0, 521.0), Vec2::new(620.0, 339.0)),
        645.499
    );

    assert_delta!(
        aurora.path(Vec2::new(712.0, 187.0), Vec2::new(157.0, 358.0)),
        647.49
    );

    assert_delta!(
        aurora.path(Vec2::new(557.0, 691.0), Vec2::new(548.0, 452.0)),
        643.226
    );

    assert_delta!(
        aurora.path(Vec2::new(169.0, 150.0), Vec2::new(784.0, 313.0)),
        645.478
    );

    assert_delta!(
        aurora.path(Vec2::new(111.0, 439.0), Vec2::new(608.0, 678.0)),
        635.395
    );

    assert_delta!(
        aurora.path(Vec2::new(286.0, 85.0), Vec2::new(230.0, 524.0)),
        648.844
    );

    assert_delta!(
        aurora.path(Vec2::new(991.0, 444.0), Vec2::new(446.0, 422.0)),
        639.1
    );

    assert_delta!(
        aurora.path(Vec2::new(501.0, 210.0), Vec2::new(1020.0, 442.0)),
        639.91
    );

    assert_delta!(
        aurora.path(Vec2::new(909.0, 245.0), Vec2::new(309.0, 372.0)),
        646.55
    );

    assert_delta!(
        aurora.path(Vec2::new(302.0, 163.0), Vec2::new(621.0, 498.0)),
        636.542
    );

    assert_delta!(
        aurora.path(Vec2::new(96.0, 182.0), Vec2::new(494.0, 488.0)),
        643.572
    );

    assert_delta!(
        aurora.path(Vec2::new(783.0, 228.0), Vec2::new(152.0, 217.0)),
        655.307
    );

    assert_delta!(
        aurora.path(Vec2::new(819.0, 155.0), Vec2::new(386.0, 386.0)),
        653.198
    );

    assert_delta!(
        aurora.path(Vec2::new(413.0, 333.0), Vec2::new(668.0, 671.0)),
        647.926
    );

    assert_delta!(
        aurora.path(Vec2::new(617.0, 501.0), Vec2::new(393.0, 248.0)),
        641.224
    );

    assert_delta!(
        aurora.path(Vec2::new(564.0, 548.0), Vec2::new(221.0, 231.0)),
        650.782
    );

    assert_delta!(
        aurora.path(Vec2::new(232.0, 228.0), Vec2::new(575.0, 554.0)),
        652.04
    );

    assert_delta!(
        aurora.path(Vec2::new(558.0, 259.0), Vec2::new(283.0, 612.0)),
        644.766
    );

    assert_delta!(
        aurora.path(Vec2::new(377.0, 133.0), Vec2::new(321.0, 525.0)),
        651.082
    );

    assert_delta!(
        aurora.path(Vec2::new(331.0, 266.0), Vec2::new(694.0, 554.0)),
        641.774
    );

    assert_delta!(
        aurora.path(Vec2::new(274.0, 587.0), Vec2::new(598.0, 267.0)),
        648.238
    );

    assert_delta!(
        aurora.path(Vec2::new(769.0, 326.0), Vec2::new(143.0, 248.0)),
        650.506
    );

    assert_delta!(
        aurora.path(Vec2::new(608.0, 673.0), Vec2::new(12.0, 496.0)),
        642.854
    );

    assert_delta!(
        aurora.path(Vec2::new(663.0, 650.0), Vec2::new(24.0, 678.0)),
        659.428
    );

    assert_delta!(
        aurora.path(Vec2::new(879.0, 303.0), Vec2::new(274.0, 124.0)),
        643.704
    );

    assert_delta!(
        aurora.path(Vec2::new(345.0, 230.0), Vec2::new(61.0, 637.0)),
        653.937
    );

    assert_delta!(
        aurora.path(Vec2::new(124.0, 549.0), Vec2::new(439.0, 184.0)),
        646.148
    );

    assert_delta!(
        aurora.path(Vec2::new(858.0, 434.0), Vec2::new(346.0, 487.0)),
        648.679
    );

    assert_delta!(
        aurora.path(Vec2::new(339.0, 164.0), Vec2::new(603.0, 532.0)),
        646.767
    );

    assert_delta!(
        aurora.path(Vec2::new(750.0, 696.0), Vec2::new(226.0, 681.0)),
        649.555
    );

    assert_delta!(
        aurora.path(Vec2::new(815.0, 693.0), Vec2::new(473.0, 340.0)),
        651.111
    );

    assert_delta!(
        aurora.path(Vec2::new(854.0, 143.0), Vec2::new(574.0, 493.0)),
        650.665
    );

    assert_delta!(
        aurora.path(Vec2::new(577.0, 591.0), Vec2::new(475.0, 172.0)),
        654.576
    );

    assert_delta!(
        aurora.path(Vec2::new(646.0, 304.0), Vec2::new(26.0, 188.0)),
        655.421
    );

    assert_delta!(
        aurora.path(Vec2::new(274.0, 235.0), Vec2::new(880.0, 235.0)),
        655.593
    );

    assert_delta!(
        aurora.path(Vec2::new(333.0, 467.0), Vec2::new(207.0, 73.0)),
        653.804
    );

    assert_delta!(
        aurora.path(Vec2::new(154.0, 369.0), Vec2::new(757.0, 396.0)),
        651.763
    );

    assert_delta!(
        aurora.path(Vec2::new(768.0, 284.0), Vec2::new(711.0, 607.0)),
        656.174
    );

    assert_delta!(
        aurora.path(Vec2::new(858.0, 273.0), Vec2::new(408.0, 618.0)),
        656.619
    );

    assert_delta!(
        aurora.path(Vec2::new(960.0, 551.0), Vec2::new(626.0, 202.0)),
        656.165
    );

    assert_delta!(
        aurora.path(Vec2::new(289.0, 122.0), Vec2::new(522.0, 569.0)),
        656.577
    );

    assert_delta!(
        aurora.path(Vec2::new(66.0, 206.0), Vec2::new(354.0, 662.0)),
        661.249
    );

    assert_delta!(
        aurora.path(Vec2::new(121.0, 192.0), Vec2::new(549.0, 473.0)),
        649.779
    );

    assert_delta!(
        aurora.path(Vec2::new(324.0, 148.0), Vec2::new(924.0, 375.0)),
        658.641
    );

    assert_delta!(
        aurora.path(Vec2::new(557.0, 548.0), Vec2::new(263.0, 133.0)),
        664.913
    );

    assert_delta!(
        aurora.path(Vec2::new(250.0, 223.0), Vec2::new(882.0, 311.0)),
        660.583
    );

    assert_delta!(
        aurora.path(Vec2::new(343.0, 110.0), Vec2::new(520.0, 519.0)),
        663.48
    );

    assert_delta!(
        aurora.path(Vec2::new(896.0, 364.0), Vec2::new(299.0, 396.0)),
        668.357
    );

    assert_delta!(
        aurora.path(Vec2::new(403.0, 648.0), Vec2::new(466.0, 353.0)),
        656.348
    );

    assert_delta!(
        aurora.path(Vec2::new(898.0, 368.0), Vec2::new(288.0, 182.0)),
        665.376
    );

    assert_delta!(
        aurora.path(Vec2::new(805.0, 505.0), Vec2::new(645.0, 146.0)),
        655.025
    );

    assert_delta!(
        aurora.path(Vec2::new(25.0, 358.0), Vec2::new(630.0, 171.0)),
        660.694
    );

    assert_delta!(
        aurora.path(Vec2::new(401.0, 230.0), Vec2::new(196.0, 593.0)),
        660.711
    );

    assert_delta!(
        aurora.path(Vec2::new(391.0, 713.0), Vec2::new(297.0, 367.0)),
        668.91
    );

    assert_delta!(
        aurora.path(Vec2::new(345.0, 191.0), Vec2::new(953.0, 347.0)),
        653.09
    );

    assert_delta!(
        aurora.path(Vec2::new(780.0, 327.0), Vec2::new(368.0, 671.0)),
        669.915
    );

    assert_delta!(
        aurora.path(Vec2::new(680.0, 599.0), Vec2::new(41.0, 626.0)),
        678.015
    );

    assert_delta!(
        aurora.path(Vec2::new(944.0, 657.0), Vec2::new(300.0, 624.0)),
        665.041
    );

    assert_delta!(
        aurora.path(Vec2::new(391.0, 430.0), Vec2::new(740.0, 248.0)),
        659.051
    );

    assert_delta!(
        aurora.path(Vec2::new(206.0, 151.0), Vec2::new(850.0, 182.0)),
        671.16
    );

    assert_delta!(
        aurora.path(Vec2::new(304.0, 575.0), Vec2::new(959.0, 638.0)),
        671.046
    );

    assert_delta!(
        aurora.path(Vec2::new(190.0, 359.0), Vec2::new(717.0, 45.0)),
        657.756
    );

    assert_delta!(
        aurora.path(Vec2::new(513.0, 311.0), Vec2::new(239.0, 698.0)),
        668.062
    );

    assert_delta!(
        aurora.path(Vec2::new(487.0, 399.0), Vec2::new(218.0, 697.0)),
        672.813
    );

    assert_delta!(
        aurora.path(Vec2::new(32.0, 182.0), Vec2::new(664.0, 310.0)),
        668.572
    );

    assert_delta!(
        aurora.path(Vec2::new(162.0, 446.0), Vec2::new(773.0, 622.0)),
        661.114
    );

    assert_delta!(
        aurora.path(Vec2::new(260.0, 200.0), Vec2::new(901.0, 312.0)),
        667.948
    );

    assert_delta!(
        aurora.path(Vec2::new(222.0, 568.0), Vec2::new(225.0, 81.0)),
        666.445
    );

    assert_delta!(
        aurora.path(Vec2::new(202.0, 205.0), Vec2::new(278.0, 663.0)),
        668.332
    );

    assert_delta!(
        aurora.path(Vec2::new(414.0, 608.0), Vec2::new(400.0, 241.0)),
        667.676
    );

    assert_delta!(
        aurora.path(Vec2::new(176.0, 169.0), Vec2::new(750.0, 344.0)),
        666.794
    );

    assert_delta!(
        aurora.path(Vec2::new(734.0, 212.0), Vec2::new(103.0, 123.0)),
        659.807
    );

    assert_delta!(
        aurora.path(Vec2::new(770.0, 564.0), Vec2::new(842.0, 124.0)),
        673.305
    );

    assert_delta!(
        aurora.path(Vec2::new(618.0, 540.0), Vec2::new(853.0, 151.0)),
        667.329
    );

    assert_delta!(
        aurora.path(Vec2::new(286.0, 89.0), Vec2::new(415.0, 544.0)),
        672.039
    );

    assert_delta!(
        aurora.path(Vec2::new(174.0, 293.0), Vec2::new(642.0, 657.0)),
        671.255
    );

    assert_delta!(
        aurora.path(Vec2::new(387.0, 439.0), Vec2::new(932.0, 340.0)),
        670.096
    );

    assert_delta!(
        aurora.path(Vec2::new(730.0, 654.0), Vec2::new(198.0, 282.0)),
        666.629
    );

    assert_delta!(
        aurora.path(Vec2::new(879.0, 424.0), Vec2::new(283.0, 172.0)),
        670.186
    );

    assert_delta!(
        aurora.path(Vec2::new(463.0, 390.0), Vec2::new(47.0, 655.0)),
        676.792
    );

    assert_delta!(
        aurora.path(Vec2::new(345.0, 672.0), Vec2::new(696.0, 395.0)),
        671.079
    );

    assert_delta!(
        aurora.path(Vec2::new(423.0, 464.0), Vec2::new(1014.0, 331.0)),
        675.96
    );

    assert_delta!(
        aurora.path(Vec2::new(402.0, 139.0), Vec2::new(1009.0, 375.0)),
        673.38
    );

    assert_delta!(
        aurora.path(Vec2::new(189.0, 622.0), Vec2::new(349.0, 179.0)),
        672.533
    );

    assert_delta!(
        aurora.path(Vec2::new(27.0, 581.0), Vec2::new(266.0, 193.0)),
        671.221
    );

    assert_delta!(
        aurora.path(Vec2::new(798.0, 615.0), Vec2::new(228.0, 398.0)),
        675.316
    );

    assert_delta!(
        aurora.path(Vec2::new(776.0, 671.0), Vec2::new(470.0, 303.0)),
        670.519
    );

    assert_delta!(
        aurora.path(Vec2::new(39.0, 541.0), Vec2::new(608.0, 381.0)),
        672.232
    );

    assert_delta!(
        aurora.path(Vec2::new(683.0, 613.0), Vec2::new(53.0, 491.0)),
        673.844
    );

    assert_delta!(
        aurora.path(Vec2::new(134.0, 503.0), Vec2::new(658.0, 719.0)),
        668.871
    );

    assert_delta!(
        aurora.path(Vec2::new(454.0, 268.0), Vec2::new(770.0, 640.0)),
        670.215
    );

    assert_delta!(
        aurora.path(Vec2::new(511.0, 202.0), Vec2::new(156.0, 640.0)),
        677.233
    );

    assert_delta!(
        aurora.path(Vec2::new(866.0, 173.0), Vec2::new(475.0, 581.0)),
        682.586
    );

    assert_delta!(
        aurora.path(Vec2::new(811.0, 185.0), Vec2::new(935.0, 588.0)),
        677.211
    );

    assert_delta!(
        aurora.path(Vec2::new(792.0, 145.0), Vec2::new(810.0, 605.0)),
        680.471
    );

    assert_delta!(
        aurora.path(Vec2::new(232.0, 698.0), Vec2::new(740.0, 401.0)),
        676.492
    );

    assert_delta!(
        aurora.path(Vec2::new(902.0, 444.0), Vec2::new(307.0, 734.0)),
        681.434
    );

    assert_delta!(
        aurora.path(Vec2::new(527.0, 562.0), Vec2::new(317.0, 92.0)),
        683.475
    );

    assert_delta!(
        aurora.path(Vec2::new(679.0, 711.0), Vec2::new(501.0, 290.0)),
        678.346
    );

    assert_delta!(
        aurora.path(Vec2::new(507.0, 395.0), Vec2::new(283.0, 692.0)),
        676.487
    );

    assert_delta!(
        aurora.path(Vec2::new(523.0, 155.0), Vec2::new(137.0, 546.0)),
        676.999
    );

    assert_delta!(
        aurora.path(Vec2::new(936.0, 345.0), Vec2::new(372.0, 475.0)),
        683.141
    );

    assert_delta!(
        aurora.path(Vec2::new(806.0, 501.0), Vec2::new(618.0, 102.0)),
        676.816
    );

    assert_delta!(
        aurora.path(Vec2::new(564.0, 179.0), Vec2::new(730.0, 513.0)),
        672.945
    );

    assert_delta!(
        aurora.path(Vec2::new(765.0, 690.0), Vec2::new(204.0, 510.0)),
        677.596
    );

    assert_delta!(
        aurora.path(Vec2::new(828.0, 178.0), Vec2::new(151.0, 176.0)),
        695.05
    );

    assert_delta!(
        aurora.path(Vec2::new(96.0, 231.0), Vec2::new(593.0, 610.0)),
        685.515
    );

    assert_delta!(
        aurora.path(Vec2::new(256.0, 198.0), Vec2::new(159.0, 648.0)),
        682.829
    );

    assert_delta!(
        aurora.path(Vec2::new(644.0, 451.0), Vec2::new(246.0, 181.0)),
        676.794
    );

    assert_delta!(
        aurora.path(Vec2::new(647.0, 232.0), Vec2::new(37.0, 349.0)),
        681.545
    );

    assert_delta!(
        aurora.path(Vec2::new(286.0, 141.0), Vec2::new(145.0, 601.0)),
        677.933
    );

    assert_delta!(
        aurora.path(Vec2::new(636.0, 252.0), Vec2::new(256.0, 568.0)),
        679.307
    );

    assert_delta!(
        aurora.path(Vec2::new(330.0, 235.0), Vec2::new(623.0, 608.0)),
        686.662
    );

    assert_delta!(
        aurora.path(Vec2::new(414.0, 702.0), Vec2::new(890.0, 359.0)),
        690.577
    );

    assert_delta!(
        aurora.path(Vec2::new(169.0, 556.0), Vec2::new(827.0, 613.0)),
        683.494
    );

    assert_delta!(
        aurora.path(Vec2::new(310.0, 522.0), Vec2::new(913.0, 413.0)),
        680.118
    );

    assert_delta!(
        aurora.path(Vec2::new(1001.0, 336.0), Vec2::new(405.0, 489.0)),
        684.693
    );

    assert_delta!(
        aurora.path(Vec2::new(63.0, 537.0), Vec2::new(652.0, 339.0)),
        692.85
    );

    assert_delta!(
        aurora.path(Vec2::new(936.0, 662.0), Vec2::new(268.0, 617.0)),
        689.424
    );

    assert_delta!(
        aurora.path(Vec2::new(631.0, 638.0), Vec2::new(338.0, 286.0)),
        689.769
    );

    assert_delta!(
        aurora.path(Vec2::new(134.0, 571.0), Vec2::new(213.0, 59.0)),
        681.562
    );

    assert_delta!(
        aurora.path(Vec2::new(176.0, 660.0), Vec2::new(541.0, 202.0)),
        686.274
    );

    assert_delta!(
        aurora.path(Vec2::new(942.0, 627.0), Vec2::new(326.0, 340.0)),
        684.397
    );

    assert_delta!(
        aurora.path(Vec2::new(27.0, 180.0), Vec2::new(213.0, 678.0)),
        697.061
    );

    assert_delta!(
        aurora.path(Vec2::new(700.0, 249.0), Vec2::new(46.0, 162.0)),
        699.122
    );

    assert_delta!(
        aurora.path(Vec2::new(825.0, 109.0), Vec2::new(132.0, 174.0)),
        702.454
    );

    assert_delta!(
        aurora.path(Vec2::new(528.0, 714.0), Vec2::new(173.0, 289.0)),
        694.684
    );

    assert_delta!(
        aurora.path(Vec2::new(491.0, 613.0), Vec2::new(21.0, 188.0)),
        694.214
    );

    assert_delta!(
        aurora.path(Vec2::new(505.0, 656.0), Vec2::new(132.0, 231.0)),
        697.758
    );

    assert_delta!(
        aurora.path(Vec2::new(803.0, 686.0), Vec2::new(232.0, 424.0)),
        683.383
    );

    assert_delta!(
        aurora.path(Vec2::new(996.0, 412.0), Vec2::new(363.0, 510.0)),
        694.416
    );

    assert_delta!(
        aurora.path(Vec2::new(57.0, 138.0), Vec2::new(127.0, 629.0)),
        694.438
    );

    assert_delta!(
        aurora.path(Vec2::new(689.0, 256.0), Vec2::new(528.0, 457.0)),
        687.42
    );

    assert_delta!(
        aurora.path(Vec2::new(319.0, 620.0), Vec2::new(356.0, 228.0)),
        690.749
    );

    assert_delta!(
        aurora.path(Vec2::new(487.0, 581.0), Vec2::new(148.0, 157.0)),
        696.203
    );

    assert_delta!(
        aurora.path(Vec2::new(435.0, 636.0), Vec2::new(398.0, 212.0)),
        696.656
    );

    assert_delta!(
        aurora.path(Vec2::new(801.0, 372.0), Vec2::new(183.0, 112.0)),
        692.669
    );

    assert_delta!(
        aurora.path(Vec2::new(321.0, 232.0), Vec2::new(637.0, 601.0)),
        684.886
    );

    assert_delta!(
        aurora.path(Vec2::new(1018.0, 422.0), Vec2::new(424.0, 145.0)),
        688.184
    );

    assert_delta!(
        aurora.path(Vec2::new(767.0, 603.0), Vec2::new(141.0, 431.0)),
        695.197
    );

    assert_delta!(
        aurora.path(Vec2::new(902.0, 321.0), Vec2::new(243.0, 188.0)),
        691.174
    );

    assert_delta!(
        aurora.path(Vec2::new(264.0, 701.0), Vec2::new(171.0, 238.0)),
        691.11
    );

    assert_delta!(
        aurora.path(Vec2::new(643.0, 690.0), Vec2::new(85.0, 551.0)),
        694.865
    );

    assert_delta!(
        aurora.path(Vec2::new(725.0, 222.0), Vec2::new(839.0, 621.0)),
        695.528
    );

    assert_delta!(
        aurora.path(Vec2::new(747.0, 94.0), Vec2::new(171.0, 364.0)),
        688.49
    );

    assert_delta!(
        aurora.path(Vec2::new(83.0, 184.0), Vec2::new(579.0, 443.0)),
        692.901
    );

    assert_delta!(
        aurora.path(Vec2::new(738.0, 78.0), Vec2::new(433.0, 470.0)),
        692.834
    );

    assert_delta!(
        aurora.path(Vec2::new(282.0, 521.0), Vec2::new(636.0, 120.0)),
        693.04
    );

    assert_delta!(
        aurora.path(Vec2::new(656.0, 455.0), Vec2::new(0.0, 571.0)),
        690.596
    );

    assert_delta!(
        aurora.path(Vec2::new(18.0, 194.0), Vec2::new(586.0, 576.0)),
        699.407
    );

    assert_delta!(
        aurora.path(Vec2::new(597.0, 32.0), Vec2::new(907.0, 493.0)),
        697.264
    );

    assert_delta!(
        aurora.path(Vec2::new(32.0, 561.0), Vec2::new(668.0, 620.0)),
        695.396
    );

    assert_delta!(
        aurora.path(Vec2::new(124.0, 316.0), Vec2::new(763.0, 555.0)),
        698.124
    );

    assert_delta!(
        aurora.path(Vec2::new(385.0, 727.0), Vec2::new(657.0, 304.0)),
        692.112
    );

    assert_delta!(
        aurora.path(Vec2::new(91.0, 573.0), Vec2::new(569.0, 398.0)),
        695.415
    );

    assert_delta!(
        aurora.path(Vec2::new(682.0, 677.0), Vec2::new(60.0, 554.0)),
        698.146
    );

    assert_delta!(
        aurora.path(Vec2::new(967.0, 353.0), Vec2::new(421.0, 708.0)),
        703.533
    );

    assert_delta!(
        aurora.path(Vec2::new(428.0, 658.0), Vec2::new(516.0, 351.0)),
        696.039
    );

    assert_delta!(
        aurora.path(Vec2::new(142.0, 504.0), Vec2::new(821.0, 549.0)),
        700.64
    );

    assert_delta!(
        aurora.path(Vec2::new(176.0, 699.0), Vec2::new(846.0, 689.0)),
        700.941
    );

    assert_delta!(
        aurora.path(Vec2::new(146.0, 650.0), Vec2::new(135.0, 141.0)),
        703.551
    );

    assert_delta!(
        aurora.path(Vec2::new(684.0, 696.0), Vec2::new(483.0, 239.0)),
        702.649
    );

    assert_delta!(
        aurora.path(Vec2::new(461.0, 468.0), Vec2::new(724.0, 252.0)),
        699.606
    );

    assert_delta!(
        aurora.path(Vec2::new(228.0, 587.0), Vec2::new(287.0, 89.0)),
        707.808
    );

    assert_delta!(
        aurora.path(Vec2::new(321.0, 344.0), Vec2::new(1014.0, 316.0)),
        712.769
    );

    assert_delta!(
        aurora.path(Vec2::new(623.0, 329.0), Vec2::new(313.0, 605.0)),
        710.605
    );

    assert_delta!(
        aurora.path(Vec2::new(428.0, 397.0), Vec2::new(663.0, 706.0)),
        713.81
    );

    assert_delta!(
        aurora.path(Vec2::new(518.0, 260.0), Vec2::new(71.0, 657.0)),
        705.892
    );

    assert_delta!(
        aurora.path(Vec2::new(977.0, 438.0), Vec2::new(398.0, 130.0)),
        702.47
    );

    assert_delta!(
        aurora.path(Vec2::new(162.0, 673.0), Vec2::new(197.0, 136.0)),
        708.689
    );

    assert_delta!(
        aurora.path(Vec2::new(138.0, 682.0), Vec2::new(156.0, 161.0)),
        707.073
    );

    assert_delta!(
        aurora.path(Vec2::new(978.0, 398.0), Vec2::new(329.0, 347.0)),
        701.174
    );

    assert_delta!(
        aurora.path(Vec2::new(147.0, 658.0), Vec2::new(798.0, 479.0)),
        722.778
    );

    assert_delta!(
        aurora.path(Vec2::new(723.0, 59.0), Vec2::new(825.0, 517.0)),
        704.514
    );

    assert_delta!(
        aurora.path(Vec2::new(101.0, 331.0), Vec2::new(749.0, 537.0)),
        703.759
    );

    assert_delta!(
        aurora.path(Vec2::new(564.0, 599.0), Vec2::new(400.0, 176.0)),
        712.55
    );

    assert_delta!(
        aurora.path(Vec2::new(422.0, 617.0), Vec2::new(438.0, 221.0)),
        712.313
    );

    assert_delta!(
        aurora.path(Vec2::new(752.0, 609.0), Vec2::new(486.0, 146.0)),
        706.424
    );

    assert_delta!(
        aurora.path(Vec2::new(64.0, 150.0), Vec2::new(625.0, 386.0)),
        700.56
    );

    assert_delta!(
        aurora.path(Vec2::new(208.0, 490.0), Vec2::new(695.0, 220.0)),
        700.359
    );

    assert_delta!(
        aurora.path(Vec2::new(397.0, 209.0), Vec2::new(278.0, 636.0)),
        704.343
    );

    assert_delta!(
        aurora.path(Vec2::new(862.0, 576.0), Vec2::new(645.0, 193.0)),
        712.266
    );

    assert_delta!(
        aurora.path(Vec2::new(552.0, 557.0), Vec2::new(803.0, 136.0)),
        712.013
    );

    assert_delta!(
        aurora.path(Vec2::new(447.0, 399.0), Vec2::new(323.0, 716.0)),
        717.968
    );

    assert_delta!(
        aurora.path(Vec2::new(75.0, 290.0), Vec2::new(740.0, 238.0)),
        716.383
    );

    assert_delta!(
        aurora.path(Vec2::new(309.0, 191.0), Vec2::new(595.0, 654.0)),
        715.922
    );

    assert_delta!(
        aurora.path(Vec2::new(126.0, 559.0), Vec2::new(243.0, 26.0)),
        707.503
    );

    assert_delta!(
        aurora.path(Vec2::new(202.0, 584.0), Vec2::new(439.0, 148.0)),
        711.178
    );

    assert_delta!(
        aurora.path(Vec2::new(766.0, 209.0), Vec2::new(204.0, 433.0)),
        708.887
    );

    assert_delta!(
        aurora.path(Vec2::new(262.0, 314.0), Vec2::new(891.0, 623.0)),
        704.644
    );

    assert_delta!(
        aurora.path(Vec2::new(330.0, 50.0), Vec2::new(298.0, 538.0)),
        713.444
    );

    assert_delta!(
        aurora.path(Vec2::new(145.0, 400.0), Vec2::new(723.0, 99.0)),
        699.759
    );

    assert_delta!(
        aurora.path(Vec2::new(422.0, 233.0), Vec2::new(841.0, 467.0)),
        710.718
    );

    assert_delta!(
        aurora.path(Vec2::new(206.0, 585.0), Vec2::new(272.0, 67.0)),
        722.026
    );

    assert_delta!(
        aurora.path(Vec2::new(660.0, 536.0), Vec2::new(750.0, 265.0)),
        716.548
    );

    assert_delta!(
        aurora.path(Vec2::new(844.0, 205.0), Vec2::new(378.0, 544.0)),
        714.032
    );

    assert_delta!(
        aurora.path(Vec2::new(761.0, 423.0), Vec2::new(178.0, 676.0)),
        712.05
    );

    assert_delta!(
        aurora.path(Vec2::new(642.0, 430.0), Vec2::new(207.0, 569.0)),
        709.976
    );

    assert_delta!(
        aurora.path(Vec2::new(217.0, 438.0), Vec2::new(849.0, 392.0)),
        715.351
    );

    assert_delta!(
        aurora.path(Vec2::new(848.0, 561.0), Vec2::new(805.0, 77.0)),
        715.403
    );

    assert_delta!(
        aurora.path(Vec2::new(347.0, 181.0), Vec2::new(564.0, 646.0)),
        716.683
    );

    assert_delta!(
        aurora.path(Vec2::new(385.0, 634.0), Vec2::new(606.0, 266.0)),
        714.728
    );

    assert_delta!(
        aurora.path(Vec2::new(826.0, 181.0), Vec2::new(142.0, 265.0)),
        728.65
    );

    assert_delta!(
        aurora.path(Vec2::new(153.0, 487.0), Vec2::new(791.0, 346.0)),
        723.459
    );

    assert_delta!(
        aurora.path(Vec2::new(828.0, 619.0), Vec2::new(142.0, 558.0)),
        715.737
    );

    assert_delta!(
        aurora.path(Vec2::new(748.0, 604.0), Vec2::new(504.0, 212.0)),
        712.14
    );

    assert_delta!(
        aurora.path(Vec2::new(818.0, 551.0), Vec2::new(766.0, 64.0)),
        720.59
    );

    assert_delta!(
        aurora.path(Vec2::new(142.0, 436.0), Vec2::new(729.0, 132.0)),
        717.587
    );

    assert_delta!(
        aurora.path(Vec2::new(447.0, 80.0), Vec2::new(807.0, 532.0)),
        715.198
    );

    assert_delta!(
        aurora.path(Vec2::new(825.0, 583.0), Vec2::new(746.0, 100.0)),
        723.068
    );

    assert_delta!(
        aurora.path(Vec2::new(746.0, 122.0), Vec2::new(887.0, 571.0)),
        717.993
    );

    assert_delta!(
        aurora.path(Vec2::new(680.0, 640.0), Vec2::new(887.0, 164.0)),
        724.58
    );

    assert_delta!(
        aurora.path(Vec2::new(184.0, 86.0), Vec2::new(818.0, 380.0)),
        716.259
    );

    assert_delta!(
        aurora.path(Vec2::new(888.0, 398.0), Vec2::new(205.0, 343.0)),
        721.585
    );

    assert_delta!(
        aurora.path(Vec2::new(960.0, 409.0), Vec2::new(369.0, 92.0)),
        715.224
    );

    assert_delta!(
        aurora.path(Vec2::new(871.0, 244.0), Vec2::new(180.0, 320.0)),
        724.339
    );

    assert_delta!(
        aurora.path(Vec2::new(139.0, 704.0), Vec2::new(335.0, 214.0)),
        721.547
    );

    assert_delta!(
        aurora.path(Vec2::new(759.0, 152.0), Vec2::new(70.0, 307.0)),
        733.942
    );

    assert_delta!(
        aurora.path(Vec2::new(43.0, 579.0), Vec2::new(665.0, 696.0)),
        726.932
    );

    assert_delta!(
        aurora.path(Vec2::new(584.0, 201.0), Vec2::new(433.0, 611.0)),
        726.876
    );

    assert_delta!(
        aurora.path(Vec2::new(76.0, 616.0), Vec2::new(120.0, 119.0)),
        723.932
    );

    assert_delta!(
        aurora.path(Vec2::new(790.0, 203.0), Vec2::new(757.0, 683.0)),
        724.515
    );

    assert_delta!(
        aurora.path(Vec2::new(370.0, 691.0), Vec2::new(826.0, 431.0)),
        733.655
    );

    assert_delta!(
        aurora.path(Vec2::new(800.0, 299.0), Vec2::new(96.0, 426.0)),
        732.144
    );

    assert_delta!(
        aurora.path(Vec2::new(75.0, 432.0), Vec2::new(566.0, 697.0)),
        719.148
    );

    assert_delta!(
        aurora.path(Vec2::new(906.0, 127.0), Vec2::new(270.0, 304.0)),
        730.597
    );

    assert_delta!(
        aurora.path(Vec2::new(347.0, 141.0), Vec2::new(195.0, 639.0)),
        724.611
    );

    assert_delta!(
        aurora.path(Vec2::new(20.0, 524.0), Vec2::new(717.0, 626.0)),
        727.536
    );

    assert_delta!(
        aurora.path(Vec2::new(190.0, 282.0), Vec2::new(821.0, 616.0)),
        723.315
    );

    assert_delta!(
        aurora.path(Vec2::new(657.0, 409.0), Vec2::new(57.0, 538.0)),
        722.147
    );

    assert_delta!(
        aurora.path(Vec2::new(466.0, 498.0), Vec2::new(734.0, 243.0)),
        726.36
    );

    assert_delta!(
        aurora.path(Vec2::new(974.0, 400.0), Vec2::new(326.0, 186.0)),
        717.958
    );

    assert_delta!(
        aurora.path(Vec2::new(123.0, 601.0), Vec2::new(827.0, 603.0)),
        741.317
    );

    assert_delta!(
        aurora.path(Vec2::new(86.0, 125.0), Vec2::new(481.0, 565.0)),
        731.59
    );

    assert_delta!(
        aurora.path(Vec2::new(790.0, 156.0), Vec2::new(899.0, 620.0)),
        731.447
    );

    assert_delta!(
        aurora.path(Vec2::new(746.0, 234.0), Vec2::new(867.0, 619.0)),
        733.36
    );

    assert_delta!(
        aurora.path(Vec2::new(143.0, 439.0), Vec2::new(741.0, 140.0)),
        727.75
    );

    assert_delta!(
        aurora.path(Vec2::new(604.0, 105.0), Vec2::new(844.0, 557.0)),
        729.482
    );

    assert_delta!(
        aurora.path(Vec2::new(195.0, 575.0), Vec2::new(585.0, 117.0)),
        730.345
    );

    assert_delta!(
        aurora.path(Vec2::new(732.0, 82.0), Vec2::new(772.0, 562.0)),
        734.128
    );

    assert_delta!(
        aurora.path(Vec2::new(760.0, 702.0), Vec2::new(133.0, 701.0)),
        736.195
    );

    assert_delta!(
        aurora.path(Vec2::new(688.0, 219.0), Vec2::new(132.0, 485.0)),
        723.859
    );

    assert_delta!(
        aurora.path(Vec2::new(322.0, 66.0), Vec2::new(609.0, 510.0)),
        731.034
    );

    assert_delta!(
        aurora.path(Vec2::new(483.0, 504.0), Vec2::new(719.0, 247.0)),
        734.021
    );

    assert_delta!(
        aurora.path(Vec2::new(61.0, 658.0), Vec2::new(296.0, 156.0)),
        735.773
    );

    assert_delta!(
        aurora.path(Vec2::new(876.0, 572.0), Vec2::new(630.0, 184.0)),
        734.392
    );

    assert_delta!(
        aurora.path(Vec2::new(403.0, 157.0), Vec2::new(824.0, 588.0)),
        734.08
    );

    assert_delta!(
        aurora.path(Vec2::new(611.0, 725.0), Vec2::new(483.0, 327.0)),
        744.141
    );

    assert_delta!(
        aurora.path(Vec2::new(783.0, 537.0), Vec2::new(359.0, 212.0)),
        734.291
    );

    assert_delta!(
        aurora.path(Vec2::new(709.0, 187.0), Vec2::new(126.0, 478.0)),
        735.059
    );

    assert_delta!(
        aurora.path(Vec2::new(43.0, 595.0), Vec2::new(413.0, 151.0)),
        733.207
    );

    assert_delta!(
        aurora.path(Vec2::new(46.0, 649.0), Vec2::new(268.0, 160.0)),
        735.498
    );

    assert_delta!(
        aurora.path(Vec2::new(589.0, 417.0), Vec2::new(393.0, 659.0)),
        734.069
    );

    assert_delta!(
        aurora.path(Vec2::new(310.0, 653.0), Vec2::new(296.0, 168.0)),
        738.114
    );

    assert_delta!(
        aurora.path(Vec2::new(836.0, 104.0), Vec2::new(838.0, 627.0)),
        738.484
    );

    assert_delta!(
        aurora.path(Vec2::new(123.0, 549.0), Vec2::new(624.0, 439.0)),
        737.617
    );

    assert_delta!(
        aurora.path(Vec2::new(472.0, 663.0), Vec2::new(78.0, 222.0)),
        733.605
    );

    assert_delta!(
        aurora.path(Vec2::new(303.0, 488.0), Vec2::new(886.0, 258.0)),
        732.206
    );

    assert_delta!(
        aurora.path(Vec2::new(300.0, 206.0), Vec2::new(820.0, 503.0)),
        737.31
    );

    assert_delta!(
        aurora.path(Vec2::new(782.0, 58.0), Vec2::new(997.0, 577.0)),
        735.945
    );

    assert_delta!(
        aurora.path(Vec2::new(863.0, 531.0), Vec2::new(722.0, 58.0)),
        739.436
    );

    assert_delta!(
        aurora.path(Vec2::new(864.0, 111.0), Vec2::new(927.0, 569.0)),
        738.587
    );

    assert_delta!(
        aurora.path(Vec2::new(742.0, 70.0), Vec2::new(505.0, 444.0)),
        732.529
    );

    assert_delta!(
        aurora.path(Vec2::new(13.0, 351.0), Vec2::new(714.0, 346.0)),
        739.577
    );

    assert_delta!(
        aurora.path(Vec2::new(24.0, 378.0), Vec2::new(690.0, 599.0)),
        736.535
    );

    assert_delta!(
        aurora.path(Vec2::new(756.0, 498.0), Vec2::new(670.0, 106.0)),
        743.014
    );

    assert_delta!(
        aurora.path(Vec2::new(163.0, 125.0), Vec2::new(619.0, 516.0)),
        735.239
    );

    assert_delta!(
        aurora.path(Vec2::new(874.0, 501.0), Vec2::new(195.0, 651.0)),
        752.015
    );

    assert_delta!(
        aurora.path(Vec2::new(69.0, 450.0), Vec2::new(656.0, 75.0)),
        735.656
    );

    assert_delta!(
        aurora.path(Vec2::new(807.0, 685.0), Vec2::new(130.0, 469.0)),
        734.49
    );

    assert_delta!(
        aurora.path(Vec2::new(878.0, 525.0), Vec2::new(434.0, 74.0)),
        737.183
    );

    assert_delta!(
        aurora.path(Vec2::new(846.0, 154.0), Vec2::new(120.0, 246.0)),
        764.437
    );

    assert_delta!(
        aurora.path(Vec2::new(524.0, 269.0), Vec2::new(584.0, 720.0)),
        746.733
    );

    assert_delta!(
        aurora.path(Vec2::new(61.0, 569.0), Vec2::new(591.0, 395.0)),
        743.949
    );

    assert_delta!(
        aurora.path(Vec2::new(142.0, 714.0), Vec2::new(864.0, 634.0)),
        751.243
    );

    assert_delta!(
        aurora.path(Vec2::new(567.0, 662.0), Vec2::new(351.0, 159.0)),
        748.816
    );

    assert_delta!(
        aurora.path(Vec2::new(121.0, 122.0), Vec2::new(823.0, 246.0)),
        739.884
    );

    assert_delta!(
        aurora.path(Vec2::new(97.0, 144.0), Vec2::new(615.0, 435.0)),
        747.803
    );

    assert_delta!(
        aurora.path(Vec2::new(162.0, 207.0), Vec2::new(717.0, 567.0)),
        735.83
    );

    assert_delta!(
        aurora.path(Vec2::new(703.0, 142.0), Vec2::new(928.0, 582.0)),
        742.786
    );

    assert_delta!(
        aurora.path(Vec2::new(910.0, 522.0), Vec2::new(393.0, 89.0)),
        741.367
    );

    assert_delta!(
        aurora.path(Vec2::new(749.0, 234.0), Vec2::new(63.0, 325.0)),
        743.953
    );

    assert_delta!(
        aurora.path(Vec2::new(747.0, 424.0), Vec2::new(35.0, 538.0)),
        740.238
    );

    assert_delta!(
        aurora.path(Vec2::new(293.0, 647.0), Vec2::new(547.0, 153.0)),
        748.603
    );

    assert_delta!(
        aurora.path(Vec2::new(290.0, 148.0), Vec2::new(984.0, 366.0)),
        742.861
    );

    assert_delta!(
        aurora.path(Vec2::new(935.0, 630.0), Vec2::new(219.0, 691.0)),
        751.093
    );

    assert_delta!(
        aurora.path(Vec2::new(863.0, 432.0), Vec2::new(177.0, 314.0)),
        739.758
    );

    assert_delta!(
        aurora.path(Vec2::new(38.0, 632.0), Vec2::new(748.0, 716.0)),
        759.416
    );

    assert_delta!(
        aurora.path(Vec2::new(289.0, 151.0), Vec2::new(933.0, 474.0)),
        743.734
    );

    assert_delta!(
        aurora.path(Vec2::new(343.0, 179.0), Vec2::new(688.0, 590.0)),
        743.15
    );

    assert_delta!(
        aurora.path(Vec2::new(57.0, 634.0), Vec2::new(414.0, 165.0)),
        748.88
    );

    assert_delta!(
        aurora.path(Vec2::new(751.0, 355.0), Vec2::new(134.0, 521.0)),
        746.126
    );

    assert_delta!(
        aurora.path(Vec2::new(96.0, 481.0), Vec2::new(787.0, 347.0)),
        760.282
    );

    assert_delta!(
        aurora.path(Vec2::new(816.0, 491.0), Vec2::new(88.0, 482.0)),
        753.376
    );

    assert_delta!(
        aurora.path(Vec2::new(823.0, 161.0), Vec2::new(518.0, 617.0)),
        752.396
    );

    assert_delta!(
        aurora.path(Vec2::new(1.0, 657.0), Vec2::new(207.0, 118.0)),
        754.04
    );

    assert_delta!(
        aurora.path(Vec2::new(63.0, 665.0), Vec2::new(313.0, 137.0)),
        756.231
    );

    assert_delta!(
        aurora.path(Vec2::new(666.0, 85.0), Vec2::new(39.0, 427.0)),
        750.919
    );

    assert_delta!(
        aurora.path(Vec2::new(279.0, 641.0), Vec2::new(766.0, 396.0)),
        755.911
    );

    assert_delta!(
        aurora.path(Vec2::new(561.0, 636.0), Vec2::new(812.0, 144.0)),
        752.731
    );

    assert_delta!(
        aurora.path(Vec2::new(833.0, 493.0), Vec2::new(168.0, 454.0)),
        747.262
    );

    assert_delta!(
        aurora.path(Vec2::new(674.0, 379.0), Vec2::new(77.0, 551.0)),
        750.534
    );

    assert_delta!(
        aurora.path(Vec2::new(856.0, 459.0), Vec2::new(119.0, 546.0)),
        762.504
    );

    assert_delta!(
        aurora.path(Vec2::new(935.0, 598.0), Vec2::new(214.0, 469.0)),
        753.018
    );

    assert_delta!(
        aurora.path(Vec2::new(669.0, 522.0), Vec2::new(153.0, 174.0)),
        746.721
    );

    assert_delta!(
        aurora.path(Vec2::new(201.0, 344.0), Vec2::new(952.0, 306.0)),
        764.864
    );

    assert_delta!(
        aurora.path(Vec2::new(61.0, 163.0), Vec2::new(654.0, 460.0)),
        746.031
    );

    assert_delta!(
        aurora.path(Vec2::new(442.0, 694.0), Vec2::new(923.0, 267.0)),
        757.996
    );

    assert_delta!(
        aurora.path(Vec2::new(448.0, 151.0), Vec2::new(262.0, 637.0)),
        755.258
    );

    assert_delta!(
        aurora.path(Vec2::new(849.0, 513.0), Vec2::new(115.0, 580.0)),
        758.004
    );

    assert_delta!(
        aurora.path(Vec2::new(400.0, 226.0), Vec2::new(37.0, 675.0)),
        760.259
    );

    assert_delta!(
        aurora.path(Vec2::new(259.0, 184.0), Vec2::new(324.0, 688.0)),
        757.819
    );

    assert_delta!(
        aurora.path(Vec2::new(88.0, 427.0), Vec2::new(721.0, 128.0)),
        754.697
    );

    assert_delta!(
        aurora.path(Vec2::new(692.0, 615.0), Vec2::new(850.0, 132.0)),
        764.572
    );

    assert_delta!(
        aurora.path(Vec2::new(171.0, 664.0), Vec2::new(638.0, 293.0)),
        761.036
    );

    assert_delta!(
        aurora.path(Vec2::new(869.0, 345.0), Vec2::new(154.0, 206.0)),
        770.079
    );

    assert_delta!(
        aurora.path(Vec2::new(854.0, 155.0), Vec2::new(465.0, 429.0)),
        756.429
    );

    assert_delta!(
        aurora.path(Vec2::new(45.0, 187.0), Vec2::new(796.0, 193.0)),
        776.824
    );

    assert_delta!(
        aurora.path(Vec2::new(834.0, 712.0), Vec2::new(139.0, 569.0)),
        752.973
    );

    assert_delta!(
        aurora.path(Vec2::new(209.0, 76.0), Vec2::new(500.0, 585.0)),
        764.763
    );

    assert_delta!(
        aurora.path(Vec2::new(167.0, 290.0), Vec2::new(873.0, 183.0)),
        763.988
    );

    assert_delta!(
        aurora.path(Vec2::new(359.0, 282.0), Vec2::new(812.0, 554.0)),
        757.016
    );

    assert_delta!(
        aurora.path(Vec2::new(361.0, 145.0), Vec2::new(170.0, 681.0)),
        768.188
    );

    assert_delta!(
        aurora.path(Vec2::new(915.0, 288.0), Vec2::new(182.0, 162.0)),
        763.705
    );

    assert_delta!(
        aurora.path(Vec2::new(743.0, 162.0), Vec2::new(909.0, 640.0)),
        762.428
    );

    assert_delta!(
        aurora.path(Vec2::new(568.0, 663.0), Vec2::new(558.0, 190.0)),
        765.991
    );

    assert_delta!(
        aurora.path(Vec2::new(411.0, 637.0), Vec2::new(991.0, 301.0)),
        769.401
    );

    assert_delta!(
        aurora.path(Vec2::new(859.0, 258.0), Vec2::new(116.0, 282.0)),
        769.241
    );

    assert_delta!(
        aurora.path(Vec2::new(632.0, 216.0), Vec2::new(845.0, 638.0)),
        771.128
    );

    assert_delta!(
        aurora.path(Vec2::new(410.0, 190.0), Vec2::new(833.0, 636.0)),
        762.815
    );

    assert_delta!(
        aurora.path(Vec2::new(627.0, 588.0), Vec2::new(365.0, 165.0)),
        758.877
    );

    assert_delta!(
        aurora.path(Vec2::new(862.0, 418.0), Vec2::new(151.0, 284.0)),
        756.604
    );

    assert_delta!(
        aurora.path(Vec2::new(368.0, 494.0), Vec2::new(765.0, 67.0)),
        764.23
    );

    assert_delta!(
        aurora.path(Vec2::new(882.0, 532.0), Vec2::new(366.0, 262.0)),
        763.192
    );

    assert_delta!(
        aurora.path(Vec2::new(142.0, 501.0), Vec2::new(753.0, 162.0)),
        780.337
    );

    assert_delta!(
        aurora.path(Vec2::new(494.0, 508.0), Vec2::new(740.0, 82.0)),
        762.84
    );

    assert_delta!(
        aurora.path(Vec2::new(378.0, 694.0), Vec2::new(322.0, 201.0)),
        771.95
    );

    assert_delta!(
        aurora.path(Vec2::new(406.0, 610.0), Vec2::new(458.0, 67.0)),
        776.348
    );

    assert_delta!(
        aurora.path(Vec2::new(696.0, 76.0), Vec2::new(815.0, 597.0)),
        774.397
    );

    assert_delta!(
        aurora.path(Vec2::new(417.0, 239.0), Vec2::new(547.0, 682.0)),
        770.817
    );

    assert_delta!(
        aurora.path(Vec2::new(280.0, 575.0), Vec2::new(277.0, 30.0)),
        769.487
    );

    assert_delta!(
        aurora.path(Vec2::new(201.0, 377.0), Vec2::new(838.0, 702.0)),
        765.708
    );

    assert_delta!(
        aurora.path(Vec2::new(773.0, 655.0), Vec2::new(522.0, 155.0)),
        768.509
    );

    assert_delta!(
        aurora.path(Vec2::new(801.0, 433.0), Vec2::new(68.0, 370.0)),
        771.243
    );

    assert_delta!(
        aurora.path(Vec2::new(907.0, 538.0), Vec2::new(164.0, 511.0)),
        769.993
    );

    assert_delta!(
        aurora.path(Vec2::new(779.0, 624.0), Vec2::new(585.0, 207.0)),
        768.016
    );

    assert_delta!(
        aurora.path(Vec2::new(289.0, 746.0), Vec2::new(704.0, 279.0)),
        780.097
    );

    assert_delta!(
        aurora.path(Vec2::new(465.0, 584.0), Vec2::new(647.0, 71.0)),
        776.097
    );

    assert_delta!(
        aurora.path(Vec2::new(223.0, 397.0), Vec2::new(955.0, 367.0)),
        785.554
    );

    assert_delta!(
        aurora.path(Vec2::new(726.0, 187.0), Vec2::new(473.0, 592.0)),
        770.797
    );

    assert_delta!(
        aurora.path(Vec2::new(697.0, 318.0), Vec2::new(252.0, 636.0)),
        784.393
    );

    assert_delta!(
        aurora.path(Vec2::new(224.0, 120.0), Vec2::new(100.0, 694.0)),
        771.919
    );

    assert_delta!(
        aurora.path(Vec2::new(466.0, 632.0), Vec2::new(140.0, 130.0)),
        777.416
    );

    assert_delta!(
        aurora.path(Vec2::new(237.0, 140.0), Vec2::new(361.0, 671.0)),
        780.241
    );

    assert_delta!(
        aurora.path(Vec2::new(970.0, 348.0), Vec2::new(359.0, 741.0)),
        776.86
    );

    assert_delta!(
        aurora.path(Vec2::new(150.0, 263.0), Vec2::new(905.0, 331.0)),
        783.426
    );

    assert_delta!(
        aurora.path(Vec2::new(856.0, 680.0), Vec2::new(187.0, 354.0)),
        774.501
    );

    assert_delta!(
        aurora.path(Vec2::new(116.0, 422.0), Vec2::new(733.0, 267.0)),
        763.463
    );

    assert_delta!(
        aurora.path(Vec2::new(334.0, 224.0), Vec2::new(918.0, 529.0)),
        773.184
    );

    assert_delta!(
        aurora.path(Vec2::new(906.0, 316.0), Vec2::new(276.0, 553.0)),
        772.252
    );

    assert_delta!(
        aurora.path(Vec2::new(15.0, 477.0), Vec2::new(651.0, 695.0)),
        775.985
    );

    assert_delta!(
        aurora.path(Vec2::new(431.0, 606.0), Vec2::new(101.0, 114.0)),
        779.39
    );

    assert_delta!(
        aurora.path(Vec2::new(169.0, 296.0), Vec2::new(876.0, 584.0)),
        772.223
    );

    assert_delta!(
        aurora.path(Vec2::new(630.0, 103.0), Vec2::new(611.0, 589.0)),
        773.523
    );

    assert_delta!(
        aurora.path(Vec2::new(65.0, 494.0), Vec2::new(757.0, 371.0)),
        773.633
    );

    assert_delta!(
        aurora.path(Vec2::new(125.0, 134.0), Vec2::new(590.0, 596.0)),
        784.664
    );

    assert_delta!(
        aurora.path(Vec2::new(563.0, 553.0), Vec2::new(927.0, 130.0)),
        778.087
    );

    assert_delta!(
        aurora.path(Vec2::new(109.0, 510.0), Vec2::new(849.0, 689.0)),
        773.372
    );

    assert_delta!(
        aurora.path(Vec2::new(193.0, 536.0), Vec2::new(674.0, 129.0)),
        773.548
    );

    assert_delta!(
        aurora.path(Vec2::new(212.0, 691.0), Vec2::new(606.0, 407.0)),
        783.267
    );

    assert_delta!(
        aurora.path(Vec2::new(27.0, 391.0), Vec2::new(749.0, 499.0)),
        775.272
    );

    assert_delta!(
        aurora.path(Vec2::new(89.0, 505.0), Vec2::new(783.0, 703.0)),
        773.531
    );

    assert_delta!(
        aurora.path(Vec2::new(163.0, 703.0), Vec2::new(586.0, 420.0)),
        778.91
    );

    assert_delta!(
        aurora.path(Vec2::new(726.0, 686.0), Vec2::new(811.0, 179.0)),
        786.745
    );

    assert_delta!(
        aurora.path(Vec2::new(336.0, 665.0), Vec2::new(608.0, 406.0)),
        782.168
    );

    assert_delta!(
        aurora.path(Vec2::new(937.0, 644.0), Vec2::new(207.0, 460.0)),
        781.285
    );

    assert_delta!(
        aurora.path(Vec2::new(636.0, 53.0), Vec2::new(310.0, 574.0)),
        782.684
    );

    assert_delta!(
        aurora.path(Vec2::new(764.0, 235.0), Vec2::new(301.0, 476.0)),
        777.682
    );

    assert_delta!(
        aurora.path(Vec2::new(571.0, 549.0), Vec2::new(195.0, 74.0)),
        784.066
    );

    assert_delta!(
        aurora.path(Vec2::new(126.0, 228.0), Vec2::new(828.0, 478.0)),
        777.353
    );

    assert_delta!(
        aurora.path(Vec2::new(248.0, 557.0), Vec2::new(912.0, 425.0)),
        783.548
    );

    assert_delta!(
        aurora.path(Vec2::new(502.0, 139.0), Vec2::new(354.0, 641.0)),
        782.584
    );

    assert_delta!(
        aurora.path(Vec2::new(829.0, 100.0), Vec2::new(523.0, 586.0)),
        786.654
    );

    assert_delta!(
        aurora.path(Vec2::new(643.0, 136.0), Vec2::new(508.0, 636.0)),
        787.74
    );

    assert_delta!(
        aurora.path(Vec2::new(732.0, 607.0), Vec2::new(394.0, 197.0)),
        785.399
    );

    assert_delta!(
        aurora.path(Vec2::new(163.0, 388.0), Vec2::new(878.0, 494.0)),
        778.183
    );

    assert_delta!(
        aurora.path(Vec2::new(190.0, 69.0), Vec2::new(156.0, 647.0)),
        785.474
    );

    assert_delta!(
        aurora.path(Vec2::new(742.0, 318.0), Vec2::new(101.0, 509.0)),
        785.202
    );

    assert_delta!(
        aurora.path(Vec2::new(61.0, 164.0), Vec2::new(703.0, 414.0)),
        776.055
    );

    assert_delta!(
        aurora.path(Vec2::new(266.0, 210.0), Vec2::new(763.0, 496.0)),
        777.107
    );

    assert_delta!(
        aurora.path(Vec2::new(389.0, 704.0), Vec2::new(303.0, 307.0)),
        796.234
    );

    assert_delta!(
        aurora.path(Vec2::new(56.0, 477.0), Vec2::new(819.0, 446.0)),
        786.195
    );

    assert_delta!(
        aurora.path(Vec2::new(860.0, 107.0), Vec2::new(472.0, 459.0)),
        787.235
    );

    assert_delta!(
        aurora.path(Vec2::new(389.0, 478.0), Vec2::new(830.0, 108.0)),
        791.234
    );

    assert_delta!(
        aurora.path(Vec2::new(462.0, 175.0), Vec2::new(824.0, 716.0)),
        793.267
    );

    assert_delta!(
        aurora.path(Vec2::new(322.0, 51.0), Vec2::new(599.0, 592.0)),
        794.362
    );

    assert_delta!(
        aurora.path(Vec2::new(554.0, 594.0), Vec2::new(751.0, 107.0)),
        791.632
    );

    assert_delta!(
        aurora.path(Vec2::new(739.0, 67.0), Vec2::new(39.0, 372.0)),
        784.99
    );

    assert_delta!(
        aurora.path(Vec2::new(186.0, 575.0), Vec2::new(816.0, 367.0)),
        790.847
    );

    assert_delta!(
        aurora.path(Vec2::new(267.0, 17.0), Vec2::new(967.0, 315.0)),
        784.347
    );

    assert_delta!(
        aurora.path(Vec2::new(598.0, 160.0), Vec2::new(900.0, 616.0)),
        788.498
    );

    assert_delta!(
        aurora.path(Vec2::new(745.0, 178.0), Vec2::new(653.0, 651.0)),
        790.096
    );

    assert_delta!(
        aurora.path(Vec2::new(44.0, 685.0), Vec2::new(247.0, 85.0)),
        798.323
    );

    assert_delta!(
        aurora.path(Vec2::new(241.0, 629.0), Vec2::new(601.0, 146.0)),
        793.24
    );

    assert_delta!(
        aurora.path(Vec2::new(842.0, 176.0), Vec2::new(88.0, 318.0)),
        806.381
    );

    assert_delta!(
        aurora.path(Vec2::new(479.0, 239.0), Vec2::new(591.0, 726.0)),
        796.617
    );

    assert_delta!(
        aurora.path(Vec2::new(24.0, 456.0), Vec2::new(724.0, 561.0)),
        789.647
    );

    assert_delta!(
        aurora.path(Vec2::new(318.0, 156.0), Vec2::new(403.0, 631.0)),
        791.8
    );

    assert_delta!(
        aurora.path(Vec2::new(655.0, 135.0), Vec2::new(788.0, 625.0)),
        788.659
    );

    assert_delta!(
        aurora.path(Vec2::new(738.0, 52.0), Vec2::new(827.0, 609.0)),
        794.599
    );

    assert_delta!(
        aurora.path(Vec2::new(703.0, 104.0), Vec2::new(67.0, 475.0)),
        784.015
    );

    assert_delta!(
        aurora.path(Vec2::new(910.0, 259.0), Vec2::new(136.0, 178.0)),
        799.261
    );

    assert_delta!(
        aurora.path(Vec2::new(81.0, 398.0), Vec2::new(728.0, 53.0)),
        791.277
    );

    assert_delta!(
        aurora.path(Vec2::new(226.0, 473.0), Vec2::new(953.0, 394.0)),
        793.963
    );

    assert_delta!(
        aurora.path(Vec2::new(524.0, 630.0), Vec2::new(710.0, 146.0)),
        797.845
    );

    assert_delta!(
        aurora.path(Vec2::new(635.0, 605.0), Vec2::new(418.0, 244.0)),
        787.687
    );

    assert_delta!(
        aurora.path(Vec2::new(128.0, 450.0), Vec2::new(785.0, 183.0)),
        798.318
    );

    assert_delta!(
        aurora.path(Vec2::new(921.0, 421.0), Vec2::new(164.0, 353.0)),
        800.321
    );

    assert_delta!(
        aurora.path(Vec2::new(713.0, 671.0), Vec2::new(676.0, 250.0)),
        797.463
    );

    assert_delta!(
        aurora.path(Vec2::new(391.0, 193.0), Vec2::new(478.0, 673.0)),
        804.09
    );

    assert_delta!(
        aurora.path(Vec2::new(1005.0, 595.0), Vec2::new(701.0, 154.0)),
        792.776
    );

    assert_delta!(
        aurora.path(Vec2::new(120.0, 150.0), Vec2::new(547.0, 600.0)),
        801.565
    );

    assert_delta!(
        aurora.path(Vec2::new(109.0, 326.0), Vec2::new(844.0, 440.0)),
        796.331
    );

    assert_delta!(
        aurora.path(Vec2::new(709.0, 629.0), Vec2::new(832.0, 84.0)),
        803.879
    );

    assert_delta!(
        aurora.path(Vec2::new(200.0, 180.0), Vec2::new(453.0, 681.0)),
        798.43
    );

    assert_delta!(
        aurora.path(Vec2::new(101.0, 165.0), Vec2::new(894.0, 242.0)),
        811.021
    );

    assert_delta!(
        aurora.path(Vec2::new(509.0, 215.0), Vec2::new(340.0, 669.0)),
        799.511
    );

    assert_delta!(
        aurora.path(Vec2::new(835.0, 318.0), Vec2::new(60.0, 393.0)),
        803.981
    );

    assert_delta!(
        aurora.path(Vec2::new(73.0, 623.0), Vec2::new(322.0, 46.0)),
        807.462
    );

    assert_delta!(
        aurora.path(Vec2::new(215.0, 591.0), Vec2::new(990.0, 616.0)),
        808.599
    );

    assert_delta!(
        aurora.path(Vec2::new(316.0, 85.0), Vec2::new(28.0, 631.0)),
        804.592
    );

    assert_delta!(
        aurora.path(Vec2::new(180.0, 413.0), Vec2::new(905.0, 471.0)),
        795.09
    );

    assert_delta!(
        aurora.path(Vec2::new(103.0, 443.0), Vec2::new(751.0, 68.0)),
        795.031
    );

    assert_delta!(
        aurora.path(Vec2::new(148.0, 385.0), Vec2::new(844.0, 666.0)),
        800.41
    );

    assert_delta!(
        aurora.path(Vec2::new(884.0, 153.0), Vec2::new(106.0, 311.0)),
        822.316
    );

    assert_delta!(
        aurora.path(Vec2::new(732.0, 521.0), Vec2::new(488.0, 51.0)),
        801.101
    );

    assert_delta!(
        aurora.path(Vec2::new(172.0, 652.0), Vec2::new(596.0, 127.0)),
        803.854
    );

    assert_delta!(
        aurora.path(Vec2::new(489.0, 209.0), Vec2::new(264.0, 732.0)),
        806.518
    );

    assert_delta!(
        aurora.path(Vec2::new(330.0, 273.0), Vec2::new(671.0, 717.0)),
        809.472
    );

    assert_delta!(
        aurora.path(Vec2::new(771.0, 426.0), Vec2::new(33.0, 664.0)),
        801.109
    );

    assert_delta!(
        aurora.path(Vec2::new(419.0, 222.0), Vec2::new(856.0, 548.0)),
        804.135
    );

    assert_delta!(
        aurora.path(Vec2::new(312.0, 673.0), Vec2::new(446.0, 158.0)),
        808.4
    );

    assert_delta!(
        aurora.path(Vec2::new(276.0, 614.0), Vec2::new(367.0, 87.0)),
        808.781
    );

    assert_delta!(
        aurora.path(Vec2::new(222.0, 65.0), Vec2::new(356.0, 621.0)),
        805.102
    );

    assert_delta!(
        aurora.path(Vec2::new(154.0, 555.0), Vec2::new(794.0, 296.0)),
        801.113
    );

    assert_delta!(
        aurora.path(Vec2::new(781.0, 216.0), Vec2::new(349.0, 595.0)),
        806.74
    );

    assert_delta!(
        aurora.path(Vec2::new(292.0, 674.0), Vec2::new(234.0, 62.0)),
        807.282
    );

    assert_delta!(
        aurora.path(Vec2::new(642.0, 714.0), Vec2::new(807.0, 176.0)),
        812.637
    );

    assert_delta!(
        aurora.path(Vec2::new(190.0, 523.0), Vec2::new(863.0, 385.0)),
        812.721
    );

    assert_delta!(
        aurora.path(Vec2::new(261.0, 316.0), Vec2::new(1006.0, 590.0)),
        804.976
    );

    assert_delta!(
        aurora.path(Vec2::new(174.0, 241.0), Vec2::new(411.0, 695.0)),
        805.028
    );

    assert_delta!(
        aurora.path(Vec2::new(800.0, 138.0), Vec2::new(166.0, 499.0)),
        811.926
    );

    assert_delta!(
        aurora.path(Vec2::new(569.0, 656.0), Vec2::new(612.0, 197.0)),
        808.309
    );

    assert_delta!(
        aurora.path(Vec2::new(791.0, 380.0), Vec2::new(318.0, 696.0)),
        810.652
    );

    assert_delta!(
        aurora.path(Vec2::new(169.0, 188.0), Vec2::new(690.0, 606.0)),
        805.863
    );

    assert_delta!(
        aurora.path(Vec2::new(192.0, 516.0), Vec2::new(779.0, 126.0)),
        809.786
    );

    assert_delta!(
        aurora.path(Vec2::new(807.0, 726.0), Vec2::new(118.0, 396.0)),
        808.414
    );

    assert_delta!(
        aurora.path(Vec2::new(114.0, 422.0), Vec2::new(890.0, 245.0)),
        816.325
    );

    assert_delta!(
        aurora.path(Vec2::new(402.0, 171.0), Vec2::new(940.0, 614.0)),
        809.104
    );

    assert_delta!(
        aurora.path(Vec2::new(853.0, 511.0), Vec2::new(293.0, 84.0)),
        807.84
    );

    assert_delta!(
        aurora.path(Vec2::new(227.0, 155.0), Vec2::new(315.0, 721.0)),
        816.37
    );

    assert_delta!(
        aurora.path(Vec2::new(24.0, 654.0), Vec2::new(585.0, 387.0)),
        809.841
    );

    assert_delta!(
        aurora.path(Vec2::new(899.0, 555.0), Vec2::new(479.0, 52.0)),
        817.004
    );

    assert_delta!(
        aurora.path(Vec2::new(767.0, 526.0), Vec2::new(168.0, 238.0)),
        809.545
    );

    assert_delta!(
        aurora.path(Vec2::new(690.0, 110.0), Vec2::new(369.0, 590.0)),
        814.191
    );

    assert_delta!(
        aurora.path(Vec2::new(333.0, 541.0), Vec2::new(743.0, 251.0)),
        812.785
    );

    assert_delta!(
        aurora.path(Vec2::new(643.0, 131.0), Vec2::new(828.0, 657.0)),
        816.101
    );

    assert_delta!(
        aurora.path(Vec2::new(338.0, 746.0), Vec2::new(1019.0, 428.0)),
        813.889
    );

    assert_delta!(
        aurora.path(Vec2::new(116.0, 434.0), Vec2::new(872.0, 389.0)),
        815.274
    );

    assert_delta!(
        aurora.path(Vec2::new(694.0, 690.0), Vec2::new(552.0, 146.0)),
        817.379
    );

    assert_delta!(
        aurora.path(Vec2::new(2.0, 582.0), Vec2::new(614.0, 200.0)),
        811.781
    );

    assert_delta!(
        aurora.path(Vec2::new(863.0, 622.0), Vec2::new(86.0, 468.0)),
        817.818
    );

    assert_delta!(
        aurora.path(Vec2::new(965.0, 567.0), Vec2::new(201.0, 357.0)),
        818.133
    );

    assert_delta!(
        aurora.path(Vec2::new(160.0, 584.0), Vec2::new(828.0, 344.0)),
        815.89
    );

    assert_delta!(
        aurora.path(Vec2::new(95.0, 500.0), Vec2::new(898.0, 577.0)),
        824.9
    );

    assert_delta!(
        aurora.path(Vec2::new(336.0, 516.0), Vec2::new(910.0, 170.0)),
        815.916
    );

    assert_delta!(
        aurora.path(Vec2::new(183.0, 417.0), Vec2::new(972.0, 284.0)),
        827.715
    );

    assert_delta!(
        aurora.path(Vec2::new(310.0, 589.0), Vec2::new(442.0, 64.0)),
        819.289
    );

    assert_delta!(
        aurora.path(Vec2::new(448.0, 201.0), Vec2::new(663.0, 673.0)),
        824.22
    );

    assert_delta!(
        aurora.path(Vec2::new(682.0, 626.0), Vec2::new(220.0, 198.0)),
        812.117
    );

    assert_delta!(
        aurora.path(Vec2::new(362.0, 239.0), Vec2::new(963.0, 549.0)),
        815.571
    );

    assert_delta!(
        aurora.path(Vec2::new(27.0, 175.0), Vec2::new(833.0, 200.0)),
        830.694
    );

    assert_delta!(
        aurora.path(Vec2::new(840.0, 235.0), Vec2::new(240.0, 524.0)),
        819.65
    );

    assert_delta!(
        aurora.path(Vec2::new(627.0, 147.0), Vec2::new(232.0, 634.0)),
        825.337
    );

    assert_delta!(
        aurora.path(Vec2::new(584.0, 174.0), Vec2::new(791.0, 688.0)),
        824.525
    );

    assert_delta!(
        aurora.path(Vec2::new(598.0, 609.0), Vec2::new(117.0, 176.0)),
        826.125
    );

    assert_delta!(
        aurora.path(Vec2::new(731.0, 61.0), Vec2::new(569.0, 593.0)),
        827.645
    );

    assert_delta!(
        aurora.path(Vec2::new(810.0, 577.0), Vec2::new(11.0, 554.0)),
        831.355
    );

    assert_delta!(
        aurora.path(Vec2::new(435.0, 106.0), Vec2::new(511.0, 652.0)),
        834.216
    );

    assert_delta!(
        aurora.path(Vec2::new(806.0, 700.0), Vec2::new(327.0, 268.0)),
        822.886
    );

    assert_delta!(
        aurora.path(Vec2::new(111.0, 272.0), Vec2::new(781.0, 678.0)),
        822.422
    );

    assert_delta!(
        aurora.path(Vec2::new(459.0, 55.0), Vec2::new(561.0, 617.0)),
        824.329
    );

    assert_delta!(
        aurora.path(Vec2::new(766.0, 563.0), Vec2::new(225.0, 235.0)),
        821.762
    );

    assert_delta!(
        aurora.path(Vec2::new(765.0, 566.0), Vec2::new(58.0, 445.0)),
        822.746
    );

    assert_delta!(
        aurora.path(Vec2::new(218.0, 494.0), Vec2::new(973.0, 374.0)),
        830.965
    );

    assert_delta!(
        aurora.path(Vec2::new(258.0, 395.0), Vec2::new(991.0, 434.0)),
        826.694
    );

    assert_delta!(
        aurora.path(Vec2::new(4.0, 563.0), Vec2::new(640.0, 122.0)),
        824.564
    );

    assert_delta!(
        aurora.path(Vec2::new(328.0, 108.0), Vec2::new(712.0, 564.0)),
        824.613
    );

    assert_delta!(
        aurora.path(Vec2::new(871.0, 560.0), Vec2::new(68.0, 548.0)),
        834.2
    );

    assert_delta!(
        aurora.path(Vec2::new(990.0, 416.0), Vec2::new(235.0, 479.0)),
        826.094
    );

    assert_delta!(
        aurora.path(Vec2::new(673.0, 74.0), Vec2::new(200.0, 589.0)),
        827.608
    );

    assert_delta!(
        aurora.path(Vec2::new(445.0, 183.0), Vec2::new(998.0, 631.0)),
        822.71
    );

    assert_delta!(
        aurora.path(Vec2::new(815.0, 411.0), Vec2::new(37.0, 381.0)),
        826.113
    );

    assert_delta!(
        aurora.path(Vec2::new(851.0, 368.0), Vec2::new(275.0, 618.0)),
        835.956
    );

    assert_delta!(
        aurora.path(Vec2::new(184.0, 671.0), Vec2::new(951.0, 467.0)),
        840.312
    );

    assert_delta!(
        aurora.path(Vec2::new(671.0, 245.0), Vec2::new(458.0, 630.0)),
        833.314
    );

    assert_delta!(
        aurora.path(Vec2::new(650.0, 142.0), Vec2::new(550.0, 659.0)),
        837.833
    );

    assert_delta!(
        aurora.path(Vec2::new(845.0, 234.0), Vec2::new(347.0, 660.0)),
        835.072
    );

    assert_delta!(
        aurora.path(Vec2::new(16.0, 406.0), Vec2::new(777.0, 647.0)),
        824.615
    );

    assert_delta!(
        aurora.path(Vec2::new(139.0, 596.0), Vec2::new(919.0, 530.0)),
        834.986
    );

    assert_delta!(
        aurora.path(Vec2::new(130.0, 102.0), Vec2::new(205.0, 683.0)),
        836.915
    );

    assert_delta!(
        aurora.path(Vec2::new(135.0, 429.0), Vec2::new(941.0, 292.0)),
        841.851
    );

    assert_delta!(
        aurora.path(Vec2::new(44.0, 666.0), Vec2::new(649.0, 338.0)),
        848.395
    );

    assert_delta!(
        aurora.path(Vec2::new(892.0, 121.0), Vec2::new(593.0, 655.0)),
        836.481
    );

    assert_delta!(
        aurora.path(Vec2::new(891.0, 289.0), Vec2::new(333.0, 714.0)),
        835.262
    );

    assert_delta!(
        aurora.path(Vec2::new(471.0, 637.0), Vec2::new(856.0, 135.0)),
        838.911
    );

    assert_delta!(
        aurora.path(Vec2::new(730.0, 261.0), Vec2::new(288.0, 555.0)),
        835.407
    );

    assert_delta!(
        aurora.path(Vec2::new(602.0, 656.0), Vec2::new(185.0, 123.0)),
        840.842
    );

    assert_delta!(
        aurora.path(Vec2::new(742.0, 94.0), Vec2::new(502.0, 582.0)),
        834.949
    );

    assert_delta!(
        aurora.path(Vec2::new(775.0, 83.0), Vec2::new(371.0, 513.0)),
        833.295
    );

    assert_delta!(
        aurora.path(Vec2::new(97.0, 390.0), Vec2::new(855.0, 136.0)),
        849.319
    );

    assert_delta!(
        aurora.path(Vec2::new(793.0, 723.0), Vec2::new(15.0, 514.0)),
        833.775
    );

    assert_delta!(
        aurora.path(Vec2::new(12.0, 649.0), Vec2::new(623.0, 283.0)),
        840.518
    );

    assert_delta!(
        aurora.path(Vec2::new(388.0, 113.0), Vec2::new(851.0, 646.0)),
        835.125
    );

    assert_delta!(
        aurora.path(Vec2::new(704.0, 293.0), Vec2::new(152.0, 672.0)),
        845.296
    );

    assert_delta!(
        aurora.path(Vec2::new(153.0, 353.0), Vec2::new(926.0, 611.0)),
        838.012
    );

    assert_delta!(
        aurora.path(Vec2::new(616.0, 173.0), Vec2::new(508.0, 659.0)),
        846.701
    );

    assert_delta!(
        aurora.path(Vec2::new(914.0, 478.0), Vec2::new(136.0, 724.0)),
        849.666
    );

    assert_delta!(
        aurora.path(Vec2::new(186.0, 491.0), Vec2::new(824.0, 209.0)),
        837.497
    );

    assert_delta!(
        aurora.path(Vec2::new(731.0, 249.0), Vec2::new(475.0, 638.0)),
        835.744
    );

    assert_delta!(
        aurora.path(Vec2::new(252.0, 590.0), Vec2::new(684.0, 243.0)),
        835.521
    );

    assert_delta!(
        aurora.path(Vec2::new(21.0, 534.0), Vec2::new(782.0, 343.0)),
        846.53
    );

    assert_delta!(
        aurora.path(Vec2::new(160.0, 528.0), Vec2::new(716.0, 151.0)),
        842.423
    );

    assert_delta!(
        aurora.path(Vec2::new(48.0, 433.0), Vec2::new(737.0, 46.0)),
        839.675
    );

    assert_delta!(
        aurora.path(Vec2::new(217.0, 91.0), Vec2::new(951.0, 464.0)),
        836.113
    );

    assert_delta!(
        aurora.path(Vec2::new(824.0, 264.0), Vec2::new(370.0, 666.0)),
        847.26
    );

    assert_delta!(
        aurora.path(Vec2::new(536.0, 707.0), Vec2::new(366.0, 157.0)),
        847.501
    );

    assert_delta!(
        aurora.path(Vec2::new(85.0, 455.0), Vec2::new(876.0, 244.0)),
        845.899
    );

    assert_delta!(
        aurora.path(Vec2::new(126.0, 702.0), Vec2::new(436.0, 146.0)),
        844.3
    );

    assert_delta!(
        aurora.path(Vec2::new(208.0, 679.0), Vec2::new(788.0, 350.0)),
        843.456
    );

    assert_delta!(
        aurora.path(Vec2::new(767.0, 260.0), Vec2::new(13.0, 386.0)),
        840.46
    );

    assert_delta!(
        aurora.path(Vec2::new(585.0, 148.0), Vec2::new(274.0, 697.0)),
        840.362
    );

    assert_delta!(
        aurora.path(Vec2::new(111.0, 657.0), Vec2::new(263.0, 24.0)),
        849.159
    );

    assert_delta!(
        aurora.path(Vec2::new(595.0, 681.0), Vec2::new(840.0, 165.0)),
        846.135
    );

    assert_delta!(
        aurora.path(Vec2::new(831.0, 688.0), Vec2::new(610.0, 118.0)),
        846.725
    );

    assert_delta!(
        aurora.path(Vec2::new(863.0, 220.0), Vec2::new(340.0, 713.0)),
        850.467
    );

    assert_delta!(
        aurora.path(Vec2::new(910.0, 623.0), Vec2::new(164.0, 419.0)),
        838.546
    );

    assert_delta!(
        aurora.path(Vec2::new(37.0, 473.0), Vec2::new(849.0, 618.0)),
        851.089
    );

    assert_delta!(
        aurora.path(Vec2::new(320.0, 52.0), Vec2::new(706.0, 536.0)),
        841.934
    );

    assert_delta!(
        aurora.path(Vec2::new(839.0, 273.0), Vec2::new(272.0, 626.0)),
        848.35
    );

    assert_delta!(
        aurora.path(Vec2::new(125.0, 104.0), Vec2::new(527.0, 617.0)),
        850.307
    );

    assert_delta!(
        aurora.path(Vec2::new(26.0, 197.0), Vec2::new(883.0, 143.0)),
        872.744
    );

    assert_delta!(
        aurora.path(Vec2::new(202.0, 701.0), Vec2::new(941.0, 405.0)),
        855.231
    );

    assert_delta!(
        aurora.path(Vec2::new(396.0, 662.0), Vec2::new(787.0, 258.0)),
        852.654
    );

    assert_delta!(
        aurora.path(Vec2::new(1001.0, 322.0), Vec2::new(206.0, 106.0)),
        844.661
    );

    assert_delta!(
        aurora.path(Vec2::new(561.0, 652.0), Vec2::new(268.0, 50.0)),
        853.912
    );

    assert_delta!(
        aurora.path(Vec2::new(600.0, 152.0), Vec2::new(780.0, 686.0)),
        849.856
    );

    assert_delta!(
        aurora.path(Vec2::new(179.0, 357.0), Vec2::new(962.0, 599.0)),
        846.456
    );

    assert_delta!(
        aurora.path(Vec2::new(622.0, 255.0), Vec2::new(136.0, 735.0)),
        846.78
    );

    assert_delta!(
        aurora.path(Vec2::new(851.0, 669.0), Vec2::new(634.0, 117.0)),
        848.382
    );

    assert_delta!(
        aurora.path(Vec2::new(303.0, 50.0), Vec2::new(912.0, 530.0)),
        843.874
    );

    assert_delta!(
        aurora.path(Vec2::new(926.0, 591.0), Vec2::new(376.0, 241.0)),
        847.603
    );

    assert_delta!(
        aurora.path(Vec2::new(872.0, 559.0), Vec2::new(101.0, 226.0)),
        848.502
    );

    assert_delta!(
        aurora.path(Vec2::new(380.0, 79.0), Vec2::new(648.0, 633.0)),
        853.052
    );

    assert_delta!(
        aurora.path(Vec2::new(982.0, 564.0), Vec2::new(213.0, 345.0)),
        848.372
    );

    assert_delta!(
        aurora.path(Vec2::new(35.0, 320.0), Vec2::new(871.0, 318.0)),
        859.241
    );

    assert_delta!(
        aurora.path(Vec2::new(487.0, 47.0), Vec2::new(897.0, 590.0)),
        855.591
    );

    assert_delta!(
        aurora.path(Vec2::new(967.0, 360.0), Vec2::new(165.0, 133.0)),
        850.887
    );

    assert_delta!(
        aurora.path(Vec2::new(445.0, 626.0), Vec2::new(831.0, 122.0)),
        852.745
    );

    assert_delta!(
        aurora.path(Vec2::new(217.0, 621.0), Vec2::new(651.0, 112.0)),
        852.612
    );

    assert_delta!(
        aurora.path(Vec2::new(312.0, 510.0), Vec2::new(810.0, 103.0)),
        854.266
    );

    assert_delta!(
        aurora.path(Vec2::new(82.0, 365.0), Vec2::new(894.0, 423.0)),
        857.783
    );

    assert_delta!(
        aurora.path(Vec2::new(915.0, 280.0), Vec2::new(69.0, 324.0)),
        865.32
    );

    assert_delta!(
        aurora.path(Vec2::new(877.0, 262.0), Vec2::new(259.0, 575.0)),
        852.767
    );

    assert_delta!(
        aurora.path(Vec2::new(990.0, 291.0), Vec2::new(145.0, 332.0)),
        862.848
    );

    assert_delta!(
        aurora.path(Vec2::new(596.0, 662.0), Vec2::new(106.0, 157.0)),
        857.789
    );

    assert_delta!(
        aurora.path(Vec2::new(1020.0, 303.0), Vec2::new(279.0, 508.0)),
        857.278
    );

    assert_delta!(
        aurora.path(Vec2::new(807.0, 100.0), Vec2::new(373.0, 517.0)),
        858.409
    );

    assert_delta!(
        aurora.path(Vec2::new(442.0, 658.0), Vec2::new(420.0, 205.0)),
        864.418
    );

    assert_delta!(
        aurora.path(Vec2::new(27.0, 440.0), Vec2::new(844.0, 297.0)),
        866.495
    );

    assert_delta!(
        aurora.path(Vec2::new(222.0, 586.0), Vec2::new(730.0, 227.0)),
        856.371
    );

    assert_delta!(
        aurora.path(Vec2::new(935.0, 564.0), Vec2::new(105.0, 468.0)),
        856.26
    );

    assert_delta!(
        aurora.path(Vec2::new(33.0, 497.0), Vec2::new(873.0, 608.0)),
        870.469
    );

    assert_delta!(
        aurora.path(Vec2::new(873.0, 378.0), Vec2::new(40.0, 379.0)),
        868.815
    );

    assert_delta!(
        aurora.path(Vec2::new(26.0, 594.0), Vec2::new(678.0, 279.0)),
        859.465
    );

    assert_delta!(
        aurora.path(Vec2::new(5.0, 632.0), Vec2::new(645.0, 364.0)),
        853.965
    );

    assert_delta!(
        aurora.path(Vec2::new(717.0, 50.0), Vec2::new(941.0, 607.0)),
        862.472
    );

    assert_delta!(
        aurora.path(Vec2::new(787.0, 107.0), Vec2::new(339.0, 545.0)),
        857.178
    );

    assert_delta!(
        aurora.path(Vec2::new(836.0, 654.0), Vec2::new(108.0, 209.0)),
        857.599
    );

    assert_delta!(
        aurora.path(Vec2::new(77.0, 170.0), Vec2::new(913.0, 314.0)),
        865.426
    );

    assert_delta!(
        aurora.path(Vec2::new(698.0, 675.0), Vec2::new(406.0, 213.0)),
        859.011
    );

    assert_delta!(
        aurora.path(Vec2::new(230.0, 551.0), Vec2::new(973.0, 437.0)),
        861.276
    );

    assert_delta!(
        aurora.path(Vec2::new(336.0, 531.0), Vec2::new(764.0, 79.0)),
        861.172
    );

    assert_delta!(
        aurora.path(Vec2::new(693.0, 621.0), Vec2::new(664.0, 87.0)),
        863.373
    );

    assert_delta!(
        aurora.path(Vec2::new(794.0, 716.0), Vec2::new(308.0, 238.0)),
        862.795
    );

    assert_delta!(
        aurora.path(Vec2::new(566.0, 653.0), Vec2::new(661.0, 118.0)),
        859.511
    );

    assert_delta!(
        aurora.path(Vec2::new(348.0, 653.0), Vec2::new(604.0, 109.0)),
        863.824
    );

    assert_delta!(
        aurora.path(Vec2::new(903.0, 137.0), Vec2::new(181.0, 450.0)),
        870.11
    );

    assert_delta!(
        aurora.path(Vec2::new(826.0, 108.0), Vec2::new(466.0, 648.0)),
        863.943
    );

    assert_delta!(
        aurora.path(Vec2::new(181.0, 689.0), Vec2::new(777.0, 384.0)),
        865.801
    );

    assert_delta!(
        aurora.path(Vec2::new(974.0, 404.0), Vec2::new(162.0, 284.0)),
        864.012
    );

    assert_delta!(
        aurora.path(Vec2::new(351.0, 116.0), Vec2::new(932.0, 595.0)),
        864.161
    );

    assert_delta!(
        aurora.path(Vec2::new(782.0, 83.0), Vec2::new(687.0, 674.0)),
        873.644
    );

    assert_delta!(
        aurora.path(Vec2::new(642.0, 626.0), Vec2::new(59.0, 183.0)),
        870.017
    );

    assert_delta!(
        aurora.path(Vec2::new(708.0, 83.0), Vec2::new(625.0, 636.0)),
        872.761
    );

    assert_delta!(
        aurora.path(Vec2::new(975.0, 587.0), Vec2::new(124.0, 506.0)),
        877.18
    );

    assert_delta!(
        aurora.path(Vec2::new(681.0, 601.0), Vec2::new(82.0, 165.0)),
        860.919
    );

    assert_delta!(
        aurora.path(Vec2::new(51.0, 398.0), Vec2::new(860.0, 224.0)),
        877.405
    );

    assert_delta!(
        aurora.path(Vec2::new(158.0, 255.0), Vec2::new(977.0, 464.0)),
        867.571
    );

    assert_delta!(
        aurora.path(Vec2::new(796.0, 93.0), Vec2::new(676.0, 684.0)),
        871.55
    );

    assert_delta!(
        aurora.path(Vec2::new(975.0, 580.0), Vec2::new(122.0, 560.0)),
        875.655
    );

    assert_delta!(
        aurora.path(Vec2::new(619.0, 173.0), Vec2::new(335.0, 672.0)),
        869.388
    );

    assert_delta!(
        aurora.path(Vec2::new(50.0, 642.0), Vec2::new(599.0, 180.0)),
        871.439
    );

    assert_delta!(
        aurora.path(Vec2::new(663.0, 125.0), Vec2::new(784.0, 679.0)),
        875.38
    );

    assert_delta!(
        aurora.path(Vec2::new(345.0, 241.0), Vec2::new(619.0, 737.0)),
        879.752
    );

    assert_delta!(
        aurora.path(Vec2::new(759.0, 172.0), Vec2::new(260.0, 581.0)),
        882.386
    );

    assert_delta!(
        aurora.path(Vec2::new(146.0, 651.0), Vec2::new(816.0, 322.0)),
        871.7
    );

    assert_delta!(
        aurora.path(Vec2::new(39.0, 550.0), Vec2::new(709.0, 111.0)),
        863.178
    );

    assert_delta!(
        aurora.path(Vec2::new(761.0, 93.0), Vec2::new(241.0, 560.0)),
        869.443
    );

    assert_delta!(
        aurora.path(Vec2::new(708.0, 175.0), Vec2::new(33.0, 573.0)),
        877.0
    );

    assert_delta!(
        aurora.path(Vec2::new(746.0, 149.0), Vec2::new(158.0, 540.0)),
        874.233
    );

    assert_delta!(
        aurora.path(Vec2::new(351.0, 738.0), Vec2::new(350.0, 258.0)),
        877.066
    );

    assert_delta!(
        aurora.path(Vec2::new(908.0, 584.0), Vec2::new(58.0, 552.0)),
        884.749
    );

    assert_delta!(
        aurora.path(Vec2::new(647.0, 723.0), Vec2::new(868.0, 147.0)),
        879.643
    );

    assert_delta!(
        aurora.path(Vec2::new(56.0, 667.0), Vec2::new(598.0, 165.0)),
        880.736
    );

    assert_delta!(
        aurora.path(Vec2::new(130.0, 685.0), Vec2::new(777.0, 400.0)),
        873.798
    );

    assert_delta!(
        aurora.path(Vec2::new(20.0, 402.0), Vec2::new(823.0, 645.0)),
        872.457
    );

    assert_delta!(
        aurora.path(Vec2::new(266.0, 98.0), Vec2::new(796.0, 575.0)),
        870.671
    );

    assert_delta!(
        aurora.path(Vec2::new(262.0, 582.0), Vec2::new(723.0, 61.0)),
        872.43
    );

    assert_delta!(
        aurora.path(Vec2::new(201.0, 576.0), Vec2::new(582.0, 30.0)),
        879.565
    );

    assert_delta!(
        aurora.path(Vec2::new(703.0, 260.0), Vec2::new(459.0, 672.0)),
        875.169
    );

    assert_delta!(
        aurora.path(Vec2::new(189.0, 467.0), Vec2::new(997.0, 399.0)),
        873.677
    );

    assert_delta!(
        aurora.path(Vec2::new(15.0, 352.0), Vec2::new(882.0, 253.0)),
        891.072
    );

    assert_delta!(
        aurora.path(Vec2::new(430.0, 224.0), Vec2::new(858.0, 629.0)),
        881.87
    );

    assert_delta!(
        aurora.path(Vec2::new(459.0, 665.0), Vec2::new(314.0, 92.0)),
        891.913
    );

    assert_delta!(
        aurora.path(Vec2::new(317.0, 620.0), Vec2::new(687.0, 163.0)),
        886.015
    );

    assert_delta!(
        aurora.path(Vec2::new(358.0, 699.0), Vec2::new(305.0, 93.0)),
        883.091
    );

    assert_delta!(
        aurora.path(Vec2::new(121.0, 375.0), Vec2::new(991.0, 291.0)),
        887.857
    );

    assert_delta!(
        aurora.path(Vec2::new(766.0, 713.0), Vec2::new(787.0, 77.0)),
        887.072
    );

    assert_delta!(
        aurora.path(Vec2::new(605.0, 173.0), Vec2::new(522.0, 708.0)),
        885.674
    );

    assert_delta!(
        aurora.path(Vec2::new(23.0, 632.0), Vec2::new(586.0, 161.0)),
        875.728
    );

    assert_delta!(
        aurora.path(Vec2::new(718.0, 91.0), Vec2::new(189.0, 594.0)),
        877.376
    );

    assert_delta!(
        aurora.path(Vec2::new(724.0, 637.0), Vec2::new(174.0, 146.0)),
        879.025
    );

    assert_delta!(
        aurora.path(Vec2::new(334.0, 57.0), Vec2::new(251.0, 711.0)),
        888.721
    );

    assert_delta!(
        aurora.path(Vec2::new(136.0, 131.0), Vec2::new(943.0, 391.0)),
        879.822
    );

    assert_delta!(
        aurora.path(Vec2::new(505.0, 663.0), Vec2::new(754.0, 267.0)),
        885.578
    );

    assert_delta!(
        aurora.path(Vec2::new(31.0, 450.0), Vec2::new(821.0, 401.0)),
        879.519
    );

    assert_delta!(
        aurora.path(Vec2::new(13.0, 190.0), Vec2::new(807.0, 544.0)),
        876.332
    );

    assert_delta!(
        aurora.path(Vec2::new(231.0, 679.0), Vec2::new(455.0, 58.0)),
        893.715
    );

    assert_delta!(
        aurora.path(Vec2::new(754.0, 697.0), Vec2::new(609.0, 160.0)),
        884.499
    );

    assert_delta!(
        aurora.path(Vec2::new(782.0, 68.0), Vec2::new(481.0, 615.0)),
        885.976
    );

    assert_delta!(
        aurora.path(Vec2::new(927.0, 334.0), Vec2::new(96.0, 468.0)),
        890.27
    );

    assert_delta!(
        aurora.path(Vec2::new(772.0, 215.0), Vec2::new(382.0, 626.0)),
        885.467
    );

    assert_delta!(
        aurora.path(Vec2::new(181.0, 527.0), Vec2::new(1023.0, 605.0)),
        886.561
    );

    assert_delta!(
        aurora.path(Vec2::new(993.0, 299.0), Vec2::new(290.0, 657.0)),
        896.545
    );

    assert_delta!(
        aurora.path(Vec2::new(731.0, 381.0), Vec2::new(4.0, 612.0)),
        881.403
    );

    assert_delta!(
        aurora.path(Vec2::new(43.0, 178.0), Vec2::new(650.0, 624.0)),
        889.061
    );

    assert_delta!(
        aurora.path(Vec2::new(807.0, 84.0), Vec2::new(278.0, 533.0)),
        882.031
    );

    assert_delta!(
        aurora.path(Vec2::new(650.0, 192.0), Vec2::new(365.0, 615.0)),
        881.064
    );

    assert_delta!(
        aurora.path(Vec2::new(224.0, 61.0), Vec2::new(619.0, 577.0)),
        882.891
    );

    assert_delta!(
        aurora.path(Vec2::new(290.0, 640.0), Vec2::new(986.0, 298.0)),
        896.034
    );

    assert_delta!(
        aurora.path(Vec2::new(607.0, 117.0), Vec2::new(1016.0, 613.0)),
        879.512
    );

    assert_delta!(
        aurora.path(Vec2::new(98.0, 703.0), Vec2::new(594.0, 180.0)),
        890.441
    );

    assert_delta!(
        aurora.path(Vec2::new(177.0, 148.0), Vec2::new(383.0, 715.0)),
        890.264
    );

    assert_delta!(
        aurora.path(Vec2::new(284.0, 730.0), Vec2::new(594.0, 192.0)),
        894.09
    );

    assert_delta!(
        aurora.path(Vec2::new(857.0, 324.0), Vec2::new(30.0, 514.0)),
        896.472
    );

    assert_delta!(
        aurora.path(Vec2::new(39.0, 188.0), Vec2::new(912.0, 305.0)),
        902.743
    );

    assert_delta!(
        aurora.path(Vec2::new(397.0, 605.0), Vec2::new(878.0, 152.0)),
        888.26
    );

    assert_delta!(
        aurora.path(Vec2::new(140.0, 250.0), Vec2::new(1014.0, 297.0)),
        898.081
    );

    assert_delta!(
        aurora.path(Vec2::new(607.0, 713.0), Vec2::new(105.0, 213.0)),
        893.597
    );

    assert_delta!(
        aurora.path(Vec2::new(185.0, 634.0), Vec2::new(701.0, 115.0)),
        889.437
    );

    assert_delta!(
        aurora.path(Vec2::new(900.0, 470.0), Vec2::new(36.0, 494.0)),
        896.375
    );

    assert_delta!(
        aurora.path(Vec2::new(984.0, 625.0), Vec2::new(701.0, 67.0)),
        893.75
    );

    assert_delta!(
        aurora.path(Vec2::new(244.0, 716.0), Vec2::new(924.0, 330.0)),
        901.896
    );

    assert_delta!(
        aurora.path(Vec2::new(879.0, 542.0), Vec2::new(19.0, 628.0)),
        897.147
    );

    assert_delta!(
        aurora.path(Vec2::new(388.0, 264.0), Vec2::new(407.0, 717.0)),
        897.799
    );

    assert_delta!(
        aurora.path(Vec2::new(152.0, 570.0), Vec2::new(894.0, 308.0)),
        886.808
    );

    assert_delta!(
        aurora.path(Vec2::new(637.0, 690.0), Vec2::new(348.0, 175.0)),
        894.243
    );

    assert_delta!(
        aurora.path(Vec2::new(946.0, 647.0), Vec2::new(307.0, 268.0)),
        888.517
    );

    assert_delta!(
        aurora.path(Vec2::new(181.0, 695.0), Vec2::new(353.0, 121.0)),
        895.829
    );

    assert_delta!(
        aurora.path(Vec2::new(106.0, 597.0), Vec2::new(978.0, 623.0)),
        903.431
    );

    assert_delta!(
        aurora.path(Vec2::new(835.0, 379.0), Vec2::new(148.0, 663.0)),
        892.344
    );

    assert_delta!(
        aurora.path(Vec2::new(995.0, 633.0), Vec2::new(180.0, 298.0)),
        890.108
    );

    assert_delta!(
        aurora.path(Vec2::new(763.0, 673.0), Vec2::new(16.0, 439.0)),
        893.83
    );

    assert_delta!(
        aurora.path(Vec2::new(843.0, 368.0), Vec2::new(152.0, 667.0)),
        900.312
    );

    assert_delta!(
        aurora.path(Vec2::new(19.0, 633.0), Vec2::new(739.0, 304.0)),
        895.488
    );

    assert_delta!(
        aurora.path(Vec2::new(867.0, 367.0), Vec2::new(192.0, 655.0)),
        901.562
    );

    assert_delta!(
        aurora.path(Vec2::new(112.0, 477.0), Vec2::new(984.0, 612.0)),
        904.843
    );

    assert_delta!(
        aurora.path(Vec2::new(78.0, 336.0), Vec2::new(892.0, 620.0)),
        891.612
    );

    assert_delta!(
        aurora.path(Vec2::new(952.0, 599.0), Vec2::new(82.0, 531.0)),
        901.449
    );

    assert_delta!(
        aurora.path(Vec2::new(235.0, 635.0), Vec2::new(624.0, 50.0)),
        902.652
    );

    assert_delta!(
        aurora.path(Vec2::new(889.0, 163.0), Vec2::new(358.0, 666.0)),
        907.67
    );

    assert_delta!(
        aurora.path(Vec2::new(408.0, 685.0), Vec2::new(291.0, 86.0)),
        904.762
    );

    assert_delta!(
        aurora.path(Vec2::new(649.0, 706.0), Vec2::new(739.0, 107.0)),
        903.56
    );

    assert_delta!(
        aurora.path(Vec2::new(871.0, 686.0), Vec2::new(376.0, 91.0)),
        899.012
    );

    assert_delta!(
        aurora.path(Vec2::new(42.0, 656.0), Vec2::new(769.0, 380.0)),
        896.515
    );

    assert_delta!(
        aurora.path(Vec2::new(563.0, 695.0), Vec2::new(447.0, 140.0)),
        898.006
    );

    assert_delta!(
        aurora.path(Vec2::new(219.0, 201.0), Vec2::new(816.0, 601.0)),
        897.739
    );

    assert_delta!(
        aurora.path(Vec2::new(531.0, 713.0), Vec2::new(731.0, 224.0)),
        901.315
    );

    assert_delta!(
        aurora.path(Vec2::new(690.0, 140.0), Vec2::new(512.0, 684.0)),
        898.337
    );

    assert_delta!(
        aurora.path(Vec2::new(673.0, 694.0), Vec2::new(758.0, 76.0)),
        906.397
    );

    assert_delta!(
        aurora.path(Vec2::new(930.0, 588.0), Vec2::new(336.0, 56.0)),
        899.331
    );

    assert_delta!(
        aurora.path(Vec2::new(185.0, 705.0), Vec2::new(909.0, 374.0)),
        916.128
    );

    assert_delta!(
        aurora.path(Vec2::new(815.0, 472.0), Vec2::new(108.0, 195.0)),
        906.041
    );

    assert_delta!(
        aurora.path(Vec2::new(722.0, 57.0), Vec2::new(197.0, 609.0)),
        904.248
    );

    assert_delta!(
        aurora.path(Vec2::new(33.0, 509.0), Vec2::new(787.0, 199.0)),
        907.722
    );

    assert_delta!(
        aurora.path(Vec2::new(36.0, 681.0), Vec2::new(928.0, 608.0)),
        925.866
    );

    assert_delta!(
        aurora.path(Vec2::new(958.0, 603.0), Vec2::new(308.0, 308.0)),
        898.636
    );

    assert_delta!(
        aurora.path(Vec2::new(859.0, 484.0), Vec2::new(8.0, 359.0)),
        906.059
    );

    assert_delta!(
        aurora.path(Vec2::new(713.0, 259.0), Vec2::new(75.0, 496.0)),
        897.479
    );

    assert_delta!(
        aurora.path(Vec2::new(576.0, 706.0), Vec2::new(743.0, 236.0)),
        907.127
    );

    assert_delta!(
        aurora.path(Vec2::new(188.0, 181.0), Vec2::new(341.0, 732.0)),
        902.097
    );

    assert_delta!(
        aurora.path(Vec2::new(36.0, 442.0), Vec2::new(889.0, 252.0)),
        918.387
    );

    assert_delta!(
        aurora.path(Vec2::new(924.0, 162.0), Vec2::new(387.0, 641.0)),
        910.014
    );

    assert_delta!(
        aurora.path(Vec2::new(721.0, 223.0), Vec2::new(273.0, 631.0)),
        906.289
    );

    assert_delta!(
        aurora.path(Vec2::new(276.0, 270.0), Vec2::new(1008.0, 602.0)),
        901.5
    );

    assert_delta!(
        aurora.path(Vec2::new(11.0, 181.0), Vec2::new(821.0, 563.0)),
        899.808
    );

    assert_delta!(
        aurora.path(Vec2::new(792.0, 598.0), Vec2::new(282.0, 43.0)),
        905.102
    );

    assert_delta!(
        aurora.path(Vec2::new(786.0, 259.0), Vec2::new(16.0, 444.0)),
        910.057
    );

    assert_delta!(
        aurora.path(Vec2::new(355.0, 610.0), Vec2::new(676.0, 117.0)),
        903.274
    );

    assert_delta!(
        aurora.path(Vec2::new(499.0, 683.0), Vec2::new(772.0, 246.0)),
        904.315
    );

    assert_delta!(
        aurora.path(Vec2::new(140.0, 541.0), Vec2::new(923.0, 384.0)),
        911.702
    );

    assert_delta!(
        aurora.path(Vec2::new(824.0, 124.0), Vec2::new(270.0, 540.0)),
        907.684
    );

    assert_delta!(
        aurora.path(Vec2::new(110.0, 273.0), Vec2::new(1002.0, 365.0)),
        923.216
    );

    assert_delta!(
        aurora.path(Vec2::new(441.0, 695.0), Vec2::new(423.0, 91.0)),
        922.328
    );

    assert_delta!(
        aurora.path(Vec2::new(730.0, 688.0), Vec2::new(134.0, 196.0)),
        910.519
    );

    assert_delta!(
        aurora.path(Vec2::new(298.0, 121.0), Vec2::new(883.0, 622.0)),
        909.531
    );

    assert_delta!(
        aurora.path(Vec2::new(137.0, 113.0), Vec2::new(447.0, 670.0)),
        910.983
    );

    assert_delta!(
        aurora.path(Vec2::new(744.0, 644.0), Vec2::new(332.0, 94.0)),
        909.436
    );

    assert_delta!(
        aurora.path(Vec2::new(663.0, 676.0), Vec2::new(245.0, 160.0)),
        916.703
    );

    assert_delta!(
        aurora.path(Vec2::new(734.0, 146.0), Vec2::new(320.0, 597.0)),
        915.57
    );

    assert_delta!(
        aurora.path(Vec2::new(120.0, 632.0), Vec2::new(844.0, 322.0)),
        906.742
    );

    assert_delta!(
        aurora.path(Vec2::new(715.0, 552.0), Vec2::new(198.0, 73.0)),
        908.876
    );

    assert_delta!(
        aurora.path(Vec2::new(829.0, 664.0), Vec2::new(508.0, 34.0)),
        921.456
    );

    assert_delta!(
        aurora.path(Vec2::new(68.0, 569.0), Vec2::new(835.0, 313.0)),
        909.053
    );

    assert_delta!(
        aurora.path(Vec2::new(323.0, 175.0), Vec2::new(765.0, 715.0)),
        918.319
    );

    assert_delta!(
        aurora.path(Vec2::new(186.0, 70.0), Vec2::new(998.0, 414.0)),
        907.793
    );

    assert_delta!(
        aurora.path(Vec2::new(160.0, 576.0), Vec2::new(910.0, 278.0)),
        908.755
    );

    assert_delta!(
        aurora.path(Vec2::new(889.0, 364.0), Vec2::new(12.0, 182.0)),
        932.431
    );

    assert_delta!(
        aurora.path(Vec2::new(146.0, 696.0), Vec2::new(647.0, 88.0)),
        918.124
    );

    assert_delta!(
        aurora.path(Vec2::new(380.0, 637.0), Vec2::new(667.0, 163.0)),
        919.247
    );

    assert_delta!(
        aurora.path(Vec2::new(69.0, 488.0), Vec2::new(935.0, 421.0)),
        918.877
    );

    assert_delta!(
        aurora.path(Vec2::new(918.0, 551.0), Vec2::new(14.0, 598.0)),
        928.754
    );

    assert_delta!(
        aurora.path(Vec2::new(252.0, 724.0), Vec2::new(939.0, 312.0)),
        926.417
    );

    assert_delta!(
        aurora.path(Vec2::new(426.0, 690.0), Vec2::new(700.0, 180.0)),
        917.882
    );

    assert_delta!(
        aurora.path(Vec2::new(28.0, 507.0), Vec2::new(940.0, 564.0)),
        930.844
    );

    assert_delta!(
        aurora.path(Vec2::new(883.0, 576.0), Vec2::new(16.0, 391.0)),
        917.343
    );

    assert_delta!(
        aurora.path(Vec2::new(380.0, 129.0), Vec2::new(791.0, 733.0)),
        927.124
    );

    assert_delta!(
        aurora.path(Vec2::new(693.0, 118.0), Vec2::new(178.0, 671.0)),
        918.549
    );

    assert_delta!(
        aurora.path(Vec2::new(706.0, 81.0), Vec2::new(150.0, 629.0)),
        919.708
    );

    assert_delta!(
        aurora.path(Vec2::new(738.0, 50.0), Vec2::new(746.0, 660.0)),
        925.589
    );

    assert_delta!(
        aurora.path(Vec2::new(100.0, 597.0), Vec2::new(724.0, 168.0)),
        928.468
    );

    assert_delta!(
        aurora.path(Vec2::new(529.0, 703.0), Vec2::new(870.0, 121.0)),
        928.03
    );

    assert_delta!(
        aurora.path(Vec2::new(593.0, 717.0), Vec2::new(452.0, 169.0)),
        926.867
    );

    assert_delta!(
        aurora.path(Vec2::new(949.0, 646.0), Vec2::new(71.0, 453.0)),
        925.038
    );

    assert_delta!(
        aurora.path(Vec2::new(623.0, 221.0), Vec2::new(128.0, 726.0)),
        924.834
    );

    assert_delta!(
        aurora.path(Vec2::new(31.0, 645.0), Vec2::new(637.0, 178.0)),
        925.962
    );

    assert_delta!(
        aurora.path(Vec2::new(864.0, 186.0), Vec2::new(190.0, 551.0)),
        921.604
    );

    assert_delta!(
        aurora.path(Vec2::new(850.0, 359.0), Vec2::new(117.0, 668.0)),
        931.373
    );

    assert_delta!(
        aurora.path(Vec2::new(1008.0, 387.0), Vec2::new(133.0, 157.0)),
        929.572
    );

    assert_delta!(
        aurora.path(Vec2::new(219.0, 220.0), Vec2::new(650.0, 715.0)),
        933.76
    );

    assert_delta!(
        aurora.path(Vec2::new(121.0, 422.0), Vec2::new(981.0, 400.0)),
        922.686
    );

    assert_delta!(
        aurora.path(Vec2::new(19.0, 357.0), Vec2::new(873.0, 180.0)),
        927.62
    );

    assert_delta!(
        aurora.path(Vec2::new(923.0, 274.0), Vec2::new(11.0, 161.0)),
        941.495
    );

    assert_delta!(
        aurora.path(Vec2::new(183.0, 708.0), Vec2::new(654.0, 172.0)),
        937.367
    );

    assert_delta!(
        aurora.path(Vec2::new(887.0, 653.0), Vec2::new(31.0, 392.0)),
        924.575
    );

    assert_delta!(
        aurora.path(Vec2::new(277.0, 126.0), Vec2::new(903.0, 618.0)),
        924.113
    );

    assert_delta!(
        aurora.path(Vec2::new(867.0, 479.0), Vec2::new(125.0, 185.0)),
        931.005
    );

    assert_delta!(
        aurora.path(Vec2::new(19.0, 370.0), Vec2::new(879.0, 614.0)),
        930.969
    );

    assert_delta!(
        aurora.path(Vec2::new(77.0, 162.0), Vec2::new(747.0, 635.0)),
        922.251
    );

    assert_delta!(
        aurora.path(Vec2::new(952.0, 276.0), Vec2::new(214.0, 585.0)),
        921.092
    );

    assert_delta!(
        aurora.path(Vec2::new(959.0, 642.0), Vec2::new(73.0, 463.0)),
        931.176
    );

    assert_delta!(
        aurora.path(Vec2::new(388.0, 212.0), Vec2::new(615.0, 731.0)),
        937.626
    );

    assert_delta!(
        aurora.path(Vec2::new(943.0, 608.0), Vec2::new(23.0, 521.0)),
        943.609
    );

    assert_delta!(
        aurora.path(Vec2::new(86.0, 557.0), Vec2::new(736.0, 161.0)),
        937.368
    );

    assert_delta!(
        aurora.path(Vec2::new(335.0, 678.0), Vec2::new(899.0, 280.0)),
        931.922
    );

    assert_delta!(
        aurora.path(Vec2::new(975.0, 633.0), Vec2::new(68.0, 684.0)),
        936.786
    );

    assert_delta!(
        aurora.path(Vec2::new(952.0, 455.0), Vec2::new(58.0, 601.0)),
        932.185
    );

    assert_delta!(
        aurora.path(Vec2::new(512.0, 41.0), Vec2::new(56.0, 638.0)),
        935.507
    );

    assert_delta!(
        aurora.path(Vec2::new(83.0, 460.0), Vec2::new(889.0, 136.0)),
        937.151
    );

    assert_delta!(
        aurora.path(Vec2::new(896.0, 534.0), Vec2::new(189.0, 248.0)),
        930.747
    );

    assert_delta!(
        aurora.path(Vec2::new(761.0, 149.0), Vec2::new(556.0, 707.0)),
        930.898
    );

    assert_delta!(
        aurora.path(Vec2::new(816.0, 418.0), Vec2::new(44.0, 595.0)),
        928.164
    );

    assert_delta!(
        aurora.path(Vec2::new(969.0, 273.0), Vec2::new(68.0, 454.0)),
        942.777
    );

    assert_delta!(
        aurora.path(Vec2::new(125.0, 703.0), Vec2::new(663.0, 139.0)),
        935.318
    );

    assert_delta!(
        aurora.path(Vec2::new(45.0, 484.0), Vec2::new(904.0, 420.0)),
        941.597
    );

    assert_delta!(
        aurora.path(Vec2::new(903.0, 305.0), Vec2::new(112.0, 510.0)),
        930.391
    );

    assert_delta!(
        aurora.path(Vec2::new(744.0, 238.0), Vec2::new(118.0, 503.0)),
        925.752
    );

    assert_delta!(
        aurora.path(Vec2::new(9.0, 632.0), Vec2::new(787.0, 369.0)),
        928.191
    );

    assert_delta!(
        aurora.path(Vec2::new(320.0, 265.0), Vec2::new(992.0, 598.0)),
        930.328
    );

    assert_delta!(
        aurora.path(Vec2::new(43.0, 370.0), Vec2::new(977.0, 283.0)),
        949.478
    );

    assert_delta!(
        aurora.path(Vec2::new(703.0, 174.0), Vec2::new(415.0, 699.0)),
        932.79
    );

    assert_delta!(
        aurora.path(Vec2::new(437.0, 67.0), Vec2::new(660.0, 713.0)),
        946.789
    );

    assert_delta!(
        aurora.path(Vec2::new(841.0, 233.0), Vec2::new(28.0, 508.0)),
        946.75
    );

    assert_delta!(
        aurora.path(Vec2::new(736.0, 628.0), Vec2::new(597.0, 31.0)),
        940.314
    );

    assert_delta!(
        aurora.path(Vec2::new(195.0, 666.0), Vec2::new(943.0, 338.0)),
        954.407
    );

    assert_delta!(
        aurora.path(Vec2::new(873.0, 580.0), Vec2::new(198.0, 233.0)),
        928.666
    );

    assert_delta!(
        aurora.path(Vec2::new(205.0, 495.0), Vec2::new(923.0, 149.0)),
        943.375
    );

    assert_delta!(
        aurora.path(Vec2::new(913.0, 551.0), Vec2::new(22.0, 366.0)),
        944.993
    );

    assert_delta!(
        aurora.path(Vec2::new(741.0, 716.0), Vec2::new(277.0, 141.0)),
        940.238
    );

    assert_delta!(
        aurora.path(Vec2::new(438.0, 658.0), Vec2::new(336.0, 72.0)),
        939.028
    );

    assert_delta!(
        aurora.path(Vec2::new(130.0, 646.0), Vec2::new(696.0, 70.0)),
        939.846
    );

    assert_delta!(
        aurora.path(Vec2::new(35.0, 459.0), Vec2::new(879.0, 413.0)),
        948.28
    );

    assert_delta!(
        aurora.path(Vec2::new(135.0, 642.0), Vec2::new(734.0, 140.0)),
        951.166
    );

    assert_delta!(
        aurora.path(Vec2::new(304.0, 658.0), Vec2::new(817.0, 232.0)),
        942.483
    );

    assert_delta!(
        aurora.path(Vec2::new(262.0, 33.0), Vec2::new(748.0, 599.0)),
        940.324
    );

    assert_delta!(
        aurora.path(Vec2::new(848.0, 125.0), Vec2::new(4.0, 449.0)),
        959.249
    );

    assert_delta!(
        aurora.path(Vec2::new(859.0, 616.0), Vec2::new(218.0, 240.0)),
        937.642
    );

    assert_delta!(
        aurora.path(Vec2::new(836.0, 93.0), Vec2::new(565.0, 709.0)),
        946.308
    );

    assert_delta!(
        aurora.path(Vec2::new(797.0, 543.0), Vec2::new(91.0, 153.0)),
        935.854
    );

    assert_delta!(
        aurora.path(Vec2::new(926.0, 610.0), Vec2::new(66.0, 354.0)),
        936.27
    );

    assert_delta!(
        aurora.path(Vec2::new(880.0, 363.0), Vec2::new(30.0, 535.0)),
        950.006
    );

    assert_delta!(
        aurora.path(Vec2::new(902.0, 419.0), Vec2::new(46.0, 581.0)),
        950.845
    );

    assert_delta!(
        aurora.path(Vec2::new(137.0, 311.0), Vec2::new(1011.0, 609.0)),
        940.191
    );

    assert_delta!(
        aurora.path(Vec2::new(160.0, 621.0), Vec2::new(754.0, 99.0)),
        947.641
    );

    assert_delta!(
        aurora.path(Vec2::new(992.0, 297.0), Vec2::new(69.0, 175.0)),
        954.016
    );

    assert_delta!(
        aurora.path(Vec2::new(536.0, 27.0), Vec2::new(326.0, 624.0)),
        949.066
    );

    assert_delta!(
        aurora.path(Vec2::new(599.0, 207.0), Vec2::new(405.0, 706.0)),
        949.642
    );

    assert_delta!(
        aurora.path(Vec2::new(638.0, 707.0), Vec2::new(406.0, 84.0)),
        951.753
    );

    assert_delta!(
        aurora.path(Vec2::new(971.0, 414.0), Vec2::new(91.0, 211.0)),
        956.628
    );

    assert_delta!(
        aurora.path(Vec2::new(835.0, 709.0), Vec2::new(33.0, 463.0)),
        941.099
    );

    assert_delta!(
        aurora.path(Vec2::new(563.0, 178.0), Vec2::new(361.0, 722.0)),
        950.743
    );

    assert_delta!(
        aurora.path(Vec2::new(258.0, 148.0), Vec2::new(627.0, 701.0)),
        955.041
    );

    assert_delta!(
        aurora.path(Vec2::new(428.0, 246.0), Vec2::new(765.0, 682.0)),
        946.053
    );

    assert_delta!(
        aurora.path(Vec2::new(118.0, 447.0), Vec2::new(1017.0, 373.0)),
        958.489
    );

    assert_delta!(
        aurora.path(Vec2::new(13.0, 504.0), Vec2::new(807.0, 180.0)),
        946.135
    );

    assert_delta!(
        aurora.path(Vec2::new(958.0, 616.0), Vec2::new(305.0, 92.0)),
        948.846
    );

    assert_delta!(
        aurora.path(Vec2::new(990.0, 366.0), Vec2::new(116.0, 508.0)),
        955.236
    );

    assert_delta!(
        aurora.path(Vec2::new(130.0, 509.0), Vec2::new(1000.0, 351.0)),
        959.488
    );

    assert_delta!(
        aurora.path(Vec2::new(820.0, 213.0), Vec2::new(317.0, 660.0)),
        951.569
    );

    assert_delta!(
        aurora.path(Vec2::new(876.0, 449.0), Vec2::new(16.0, 465.0)),
        943.985
    );

    assert_delta!(
        aurora.path(Vec2::new(573.0, 697.0), Vec2::new(386.0, 88.0)),
        950.723
    );

    assert_delta!(
        aurora.path(Vec2::new(95.0, 134.0), Vec2::new(541.0, 709.0)),
        957.867
    );

    assert_delta!(
        aurora.path(Vec2::new(54.0, 374.0), Vec2::new(947.0, 580.0)),
        953.564
    );

    assert_delta!(
        aurora.path(Vec2::new(826.0, 616.0), Vec2::new(152.0, 153.0)),
        954.121
    );

    assert_delta!(
        aurora.path(Vec2::new(72.0, 577.0), Vec2::new(719.0, 225.0)),
        949.135
    );

    assert_delta!(
        aurora.path(Vec2::new(62.0, 528.0), Vec2::new(908.0, 339.0)),
        960.329
    );

    assert_delta!(
        aurora.path(Vec2::new(222.0, 235.0), Vec2::new(947.0, 557.0)),
        951.591
    );

    assert_delta!(
        aurora.path(Vec2::new(808.0, 140.0), Vec2::new(254.0, 611.0)),
        961.662
    );

    assert_delta!(
        aurora.path(Vec2::new(756.0, 698.0), Vec2::new(28.0, 198.0)),
        953.558
    );

    assert_delta!(
        aurora.path(Vec2::new(931.0, 568.0), Vec2::new(199.0, 208.0)),
        963.295
    );

    assert_delta!(
        aurora.path(Vec2::new(275.0, 568.0), Vec2::new(814.0, 109.0)),
        951.806
    );

    assert_delta!(
        aurora.path(Vec2::new(756.0, 671.0), Vec2::new(349.0, 66.0)),
        961.606
    );

    assert_delta!(
        aurora.path(Vec2::new(434.0, 123.0), Vec2::new(379.0, 722.0)),
        962.82
    );

    assert_delta!(
        aurora.path(Vec2::new(397.0, 254.0), Vec2::new(731.0, 682.0)),
        955.612
    );

    assert_delta!(
        aurora.path(Vec2::new(882.0, 278.0), Vec2::new(206.0, 704.0)),
        960.196
    );

    assert_delta!(
        aurora.path(Vec2::new(196.0, 589.0), Vec2::new(835.0, 155.0)),
        969.113
    );

    assert_delta!(
        aurora.path(Vec2::new(225.0, 170.0), Vec2::new(760.0, 691.0)),
        957.338
    );

    assert_delta!(
        aurora.path(Vec2::new(27.0, 200.0), Vec2::new(750.0, 684.0)),
        952.049
    );

    assert_delta!(
        aurora.path(Vec2::new(999.0, 358.0), Vec2::new(118.0, 509.0)),
        965.04
    );

    assert_delta!(
        aurora.path(Vec2::new(914.0, 532.0), Vec2::new(14.0, 635.0)),
        959.281
    );

    assert_delta!(
        aurora.path(Vec2::new(831.0, 235.0), Vec2::new(7.0, 473.0)),
        962.378
    );

    assert_delta!(
        aurora.path(Vec2::new(945.0, 407.0), Vec2::new(42.0, 159.0)),
        967.566
    );

    assert_delta!(
        aurora.path(Vec2::new(49.0, 517.0), Vec2::new(938.0, 263.0)),
        967.541
    );

    assert_delta!(
        aurora.path(Vec2::new(350.0, 621.0), Vec2::new(747.0, 148.0)),
        966.31
    );

    assert_delta!(
        aurora.path(Vec2::new(734.0, 258.0), Vec2::new(103.0, 594.0)),
        958.748
    );

    assert_delta!(
        aurora.path(Vec2::new(172.0, 640.0), Vec2::new(909.0, 254.0)),
        955.79
    );

    assert_delta!(
        aurora.path(Vec2::new(600.0, 709.0), Vec2::new(242.0, 180.0)),
        964.472
    );

    assert_delta!(
        aurora.path(Vec2::new(65.0, 185.0), Vec2::new(978.0, 377.0)),
        967.203
    );

    assert_delta!(
        aurora.path(Vec2::new(125.0, 474.0), Vec2::new(1015.0, 421.0)),
        961.996
    );

    assert_delta!(
        aurora.path(Vec2::new(24.0, 172.0), Vec2::new(942.0, 362.0)),
        970.023
    );

    assert_delta!(
        aurora.path(Vec2::new(926.0, 380.0), Vec2::new(139.0, 698.0)),
        971.334
    );

    assert_delta!(
        aurora.path(Vec2::new(993.0, 632.0), Vec2::new(61.0, 467.0)),
        971.354
    );

    assert_delta!(
        aurora.path(Vec2::new(173.0, 226.0), Vec2::new(888.0, 673.0)),
        960.863
    );

    assert_delta!(
        aurora.path(Vec2::new(8.0, 365.0), Vec2::new(919.0, 582.0)),
        967.566
    );

    assert_delta!(
        aurora.path(Vec2::new(790.0, 90.0), Vec2::new(138.0, 560.0)),
        961.117
    );

    assert_delta!(
        aurora.path(Vec2::new(352.0, 715.0), Vec2::new(812.0, 169.0)),
        969.844
    );

    assert_delta!(
        aurora.path(Vec2::new(207.0, 709.0), Vec2::new(679.0, 69.0)),
        972.769
    );

    assert_delta!(
        aurora.path(Vec2::new(891.0, 172.0), Vec2::new(217.0, 587.0)),
        963.013
    );

    assert_delta!(
        aurora.path(Vec2::new(969.0, 631.0), Vec2::new(28.0, 493.0)),
        974.411
    );

    assert_delta!(
        aurora.path(Vec2::new(500.0, 47.0), Vec2::new(230.0, 719.0)),
        970.065
    );

    assert_delta!(
        aurora.path(Vec2::new(854.0, 200.0), Vec2::new(322.0, 679.0)),
        968.262
    );

    assert_delta!(
        aurora.path(Vec2::new(324.0, 742.0), Vec2::new(61.0, 147.0)),
        968.343
    );

    assert_delta!(
        aurora.path(Vec2::new(738.0, 260.0), Vec2::new(88.0, 567.0)),
        964.753
    );

    assert_delta!(
        aurora.path(Vec2::new(203.0, 694.0), Vec2::new(717.0, 153.0)),
        978.264
    );

    assert_delta!(
        aurora.path(Vec2::new(663.0, 87.0), Vec2::new(153.0, 735.0)),
        969.579
    );

    assert_delta!(
        aurora.path(Vec2::new(548.0, 703.0), Vec2::new(365.0, 66.0)),
        977.028
    );

    assert_delta!(
        aurora.path(Vec2::new(877.0, 178.0), Vec2::new(52.0, 481.0)),
        973.749
    );

    assert_delta!(
        aurora.path(Vec2::new(930.0, 622.0), Vec2::new(31.0, 365.0)),
        968.832
    );

    assert_delta!(
        aurora.path(Vec2::new(26.0, 632.0), Vec2::new(947.0, 459.0)),
        966.846
    );

    assert_delta!(
        aurora.path(Vec2::new(888.0, 410.0), Vec2::new(8.0, 512.0)),
        969.213
    );

    assert_delta!(
        aurora.path(Vec2::new(308.0, 52.0), Vec2::new(961.0, 618.0)),
        967.038
    );

    assert_delta!(
        aurora.path(Vec2::new(48.0, 546.0), Vec2::new(846.0, 153.0)),
        984.055
    );

    assert_delta!(
        aurora.path(Vec2::new(833.0, 641.0), Vec2::new(139.0, 184.0)),
        964.505
    );

    assert_delta!(
        aurora.path(Vec2::new(349.0, 718.0), Vec2::new(935.0, 148.0)),
        976.227
    );

    assert_delta!(
        aurora.path(Vec2::new(1.0, 442.0), Vec2::new(927.0, 381.0)),
        986.434
    );

    assert_delta!(
        aurora.path(Vec2::new(710.0, 157.0), Vec2::new(80.0, 660.0)),
        980.388
    );

    assert_delta!(
        aurora.path(Vec2::new(950.0, 440.0), Vec2::new(139.0, 633.0)),
        988.294
    );

    assert_delta!(
        aurora.path(Vec2::new(803.0, 156.0), Vec2::new(203.0, 635.0)),
        984.952
    );

    assert_delta!(
        aurora.path(Vec2::new(797.0, 265.0), Vec2::new(7.0, 520.0)),
        969.203
    );

    assert_delta!(
        aurora.path(Vec2::new(513.0, 143.0), Vec2::new(312.0, 755.0)),
        977.564
    );

    assert_delta!(
        aurora.path(Vec2::new(1006.0, 588.0), Vec2::new(51.0, 541.0)),
        982.381
    );

    assert_delta!(
        aurora.path(Vec2::new(1012.0, 444.0), Vec2::new(58.0, 516.0)),
        982.302
    );

    assert_delta!(
        aurora.path(Vec2::new(124.0, 713.0), Vec2::new(983.0, 361.0)),
        992.807
    );

    assert_delta!(
        aurora.path(Vec2::new(725.0, 127.0), Vec2::new(243.0, 707.0)),
        982.288
    );

    assert_delta!(
        aurora.path(Vec2::new(782.0, 195.0), Vec2::new(275.0, 646.0)),
        979.082
    );

    assert_delta!(
        aurora.path(Vec2::new(596.0, 728.0), Vec2::new(791.0, 78.0)),
        977.525
    );

    assert_delta!(
        aurora.path(Vec2::new(979.0, 589.0), Vec2::new(269.0, 100.0)),
        968.371
    );

    assert_delta!(
        aurora.path(Vec2::new(625.0, 128.0), Vec2::new(403.0, 713.0)),
        979.638
    );

    assert_delta!(
        aurora.path(Vec2::new(795.0, 263.0), Vec2::new(261.0, 642.0)),
        981.232
    );

    assert_delta!(
        aurora.path(Vec2::new(106.0, 612.0), Vec2::new(738.0, 282.0)),
        977.24
    );

    assert_delta!(
        aurora.path(Vec2::new(106.0, 624.0), Vec2::new(760.0, 119.0)),
        983.087
    );

    assert_delta!(
        aurora.path(Vec2::new(803.0, 152.0), Vec2::new(123.0, 599.0)),
        990.046
    );

    assert_delta!(
        aurora.path(Vec2::new(42.0, 566.0), Vec2::new(793.0, 275.0)),
        980.163
    );

    assert_delta!(
        aurora.path(Vec2::new(979.0, 363.0), Vec2::new(83.0, 496.0)),
        985.647
    );

    assert_delta!(
        aurora.path(Vec2::new(857.0, 598.0), Vec2::new(185.0, 131.0)),
        978.562
    );

    assert_delta!(
        aurora.path(Vec2::new(823.0, 127.0), Vec2::new(8.0, 502.0)),
        971.566
    );

    assert_delta!(
        aurora.path(Vec2::new(833.0, 105.0), Vec2::new(397.0, 683.0)),
        985.181
    );

    assert_delta!(
        aurora.path(Vec2::new(398.0, 125.0), Vec2::new(369.0, 713.0)),
        989.761
    );

    assert_delta!(
        aurora.path(Vec2::new(140.0, 585.0), Vec2::new(799.0, 87.0)),
        976.889
    );

    assert_delta!(
        aurora.path(Vec2::new(834.0, 329.0), Vec2::new(24.0, 646.0)),
        971.856
    );

    assert_delta!(
        aurora.path(Vec2::new(702.0, 214.0), Vec2::new(171.0, 700.0)),
        978.893
    );

    assert_delta!(
        aurora.path(Vec2::new(849.0, 430.0), Vec2::new(28.0, 598.0)),
        978.599
    );

    assert_delta!(
        aurora.path(Vec2::new(792.0, 97.0), Vec2::new(131.0, 603.0)),
        986.148
    );

    assert_delta!(
        aurora.path(Vec2::new(650.0, 57.0), Vec2::new(722.0, 685.0)),
        990.38
    );

    assert_delta!(
        aurora.path(Vec2::new(964.0, 349.0), Vec2::new(135.0, 675.0)),
        995.156
    );

    assert_delta!(
        aurora.path(Vec2::new(283.0, 685.0), Vec2::new(580.0, 44.0)),
        984.101
    );

    assert_delta!(
        aurora.path(Vec2::new(20.0, 171.0), Vec2::new(769.0, 718.0)),
        981.948
    );

    assert_delta!(
        aurora.path(Vec2::new(1016.0, 381.0), Vec2::new(142.0, 680.0)),
        994.584
    );

    assert_delta!(
        aurora.path(Vec2::new(939.0, 357.0), Vec2::new(99.0, 602.0)),
        986.219
    );

    assert_delta!(
        aurora.path(Vec2::new(748.0, 145.0), Vec2::new(287.0, 693.0)),
        991.334
    );

    assert_delta!(
        aurora.path(Vec2::new(339.0, 612.0), Vec2::new(757.0, 82.0)),
        978.567
    );

    assert_delta!(
        aurora.path(Vec2::new(282.0, 692.0), Vec2::new(743.0, 126.0)),
        990.931
    );

    assert_delta!(
        aurora.path(Vec2::new(391.0, 721.0), Vec2::new(374.0, 107.0)),
        997.016
    );

    assert_delta!(
        aurora.path(Vec2::new(301.0, 97.0), Vec2::new(745.0, 671.0)),
        991.751
    );

    assert_delta!(
        aurora.path(Vec2::new(678.0, 249.0), Vec2::new(178.0, 722.0)),
        987.037
    );

    assert_delta!(
        aurora.path(Vec2::new(198.0, 681.0), Vec2::new(728.0, 210.0)),
        991.765
    );

    assert_delta!(
        aurora.path(Vec2::new(699.0, 163.0), Vec2::new(289.0, 725.0)),
        995.027
    );

    assert_delta!(
        aurora.path(Vec2::new(899.0, 339.0), Vec2::new(54.0, 578.0)),
        984.014
    );

    assert_delta!(
        aurora.path(Vec2::new(959.0, 641.0), Vec2::new(245.0, 234.0)),
        995.78
    );

    assert_delta!(
        aurora.path(Vec2::new(924.0, 640.0), Vec2::new(238.0, 134.0)),
        986.483
    );

    assert_delta!(
        aurora.path(Vec2::new(183.0, 92.0), Vec2::new(528.0, 706.0)),
        991.404
    );

    assert_delta!(
        aurora.path(Vec2::new(2.0, 609.0), Vec2::new(957.0, 460.0)),
        991.871
    );

    assert_delta!(
        aurora.path(Vec2::new(1016.0, 581.0), Vec2::new(121.0, 327.0)),
        983.865
    );

    assert_delta!(
        aurora.path(Vec2::new(945.0, 569.0), Vec2::new(245.0, 26.0)),
        984.542
    );

    assert_delta!(
        aurora.path(Vec2::new(744.0, 175.0), Vec2::new(58.0, 628.0)),
        1000.5947
    );

    assert_delta!(
        aurora.path(Vec2::new(89.0, 481.0), Vec2::new(1009.0, 431.0)),
        996.728
    );

    assert_delta!(
        aurora.path(Vec2::new(262.0, 726.0), Vec2::new(710.0, 212.0)),
        991.659
    );

    assert_delta!(
        aurora.path(Vec2::new(277.0, 728.0), Vec2::new(713.0, 176.0)),
        1000.6979
    );

    assert_delta!(
        aurora.path(Vec2::new(510.0, 46.0), Vec2::new(532.0, 699.0)),
        995.914
    );

    assert_delta!(
        aurora.path(Vec2::new(233.0, 198.0), Vec2::new(919.0, 657.0)),
        999.093
    );

    assert_delta!(
        aurora.path(Vec2::new(737.0, 232.0), Vec2::new(351.0, 639.0)),
        988.227
    );

    assert_delta!(
        aurora.path(Vec2::new(367.0, 704.0), Vec2::new(782.0, 155.0)),
        998.706
    );

    assert_delta!(
        aurora.path(Vec2::new(688.0, 210.0), Vec2::new(82.0, 687.0)),
        995.845
    );

    assert_delta!(
        aurora.path(Vec2::new(996.0, 423.0), Vec2::new(79.0, 573.0)),
        999.01
    );

    assert_delta!(
        aurora.path(Vec2::new(373.0, 685.0), Vec2::new(715.0, 140.0)),
        998.232
    );

    assert_delta!(
        aurora.path(Vec2::new(816.0, 120.0), Vec2::new(381.0, 702.0)),
        1000.6705
    );

    assert_delta!(
        aurora.path(Vec2::new(298.0, 48.0), Vec2::new(776.0, 678.0)),
        996.3369
    );

    assert_delta!(
        aurora.path(Vec2::new(1008.0, 404.0), Vec2::new(116.0, 714.0)),
        1009.6711
    );

    assert_delta!(
        aurora.path(Vec2::new(87.0, 444.0), Vec2::new(955.0, 439.0)),
        993.6116
    );

    assert_delta!(
        aurora.path(Vec2::new(133.0, 671.0), Vec2::new(901.0, 276.0)),
        991.90805
    );

    assert_delta!(
        aurora.path(Vec2::new(997.0, 402.0), Vec2::new(63.0, 406.0)),
        994.78382
    );

    assert_delta!(
        aurora.path(Vec2::new(103.0, 554.0), Vec2::new(802.0, 214.0)),
        997.61857
    );

    assert_delta!(
        aurora.path(Vec2::new(777.0, 244.0), Vec2::new(192.0, 666.0)),
        999.93201
    );

    assert_delta!(
        aurora.path(Vec2::new(564.0, 30.0), Vec2::new(3.0, 603.0)),
        994.40793
    );

    assert_delta!(
        aurora.path(Vec2::new(154.0, 93.0), Vec2::new(733.0, 621.0)),
        993.39839
    );

    assert_delta!(
        aurora.path(Vec2::new(696.0, 144.0), Vec2::new(1.0, 609.0)),
        997.58055
    );

    assert_delta!(
        aurora.path(Vec2::new(968.0, 355.0), Vec2::new(39.0, 511.0)),
        1007.6115
    );

    assert_delta!(
        aurora.path(Vec2::new(791.0, 96.0), Vec2::new(116.0, 609.0)),
        1000.729
    );

    assert_delta!(
        aurora.path(Vec2::new(78.0, 541.0), Vec2::new(981.0, 422.0)),
        1000.9445
    );

    assert_delta!(
        aurora.path(Vec2::new(842.0, 117.0), Vec2::new(242.0, 613.0)),
        1002.2897
    );

    assert_delta!(
        aurora.path(Vec2::new(731.0, 707.0), Vec2::new(77.0, 178.0)),
        994.75795
    );

    assert_delta!(
        aurora.path(Vec2::new(730.0, 44.0), Vec2::new(215.0, 683.0)),
        1005.657
    );

    assert_delta!(
        aurora.path(Vec2::new(371.0, 125.0), Vec2::new(596.0, 719.0)),
        1006.3103
    );

    assert_delta!(
        aurora.path(Vec2::new(3.0, 161.0), Vec2::new(882.0, 621.0)),
        996.874
    );

    assert_delta!(
        aurora.path(Vec2::new(688.0, 68.0), Vec2::new(34.0, 653.0)),
        1003.2607
    );

    assert_delta!(
        aurora.path(Vec2::new(33.0, 193.0), Vec2::new(938.0, 538.0)),
        997.98276
    );

    assert_delta!(
        aurora.path(Vec2::new(990.0, 335.0), Vec2::new(16.0, 173.0)),
        1008.9775
    );

    assert_delta!(
        aurora.path(Vec2::new(1021.0, 302.0), Vec2::new(37.0, 198.0)),
        1014.8737
    );

    assert_delta!(
        aurora.path(Vec2::new(724.0, 47.0), Vec2::new(519.0, 699.0)),
        1002.3511
    );

    assert_delta!(
        aurora.path(Vec2::new(350.0, 737.0), Vec2::new(708.0, 181.0)),
        998.59063
    );

    assert_delta!(
        aurora.path(Vec2::new(954.0, 572.0), Vec2::new(14.0, 330.0)),
        1002.4995
    );

    assert_delta!(
        aurora.path(Vec2::new(427.0, 688.0), Vec2::new(495.0, 37.0)),
        1018.9221
    );

    assert_delta!(
        aurora.path(Vec2::new(1006.0, 336.0), Vec2::new(126.0, 715.0)),
        1024.5258
    );

    assert_delta!(
        aurora.path(Vec2::new(839.0, 96.0), Vec2::new(103.0, 513.0)),
        1002.4129
    );

    assert_delta!(
        aurora.path(Vec2::new(738.0, 167.0), Vec2::new(56.0, 668.0)),
        1021.4663
    );

    assert_delta!(
        aurora.path(Vec2::new(997.0, 301.0), Vec2::new(17.0, 180.0)),
        1016.2511
    );

    assert_delta!(
        aurora.path(Vec2::new(154.0, 142.0), Vec2::new(860.0, 637.0)),
        999.89383
    );

    assert_delta!(
        aurora.path(Vec2::new(601.0, 41.0), Vec2::new(742.0, 678.0)),
        1012.7972
    );

    assert_delta!(
        aurora.path(Vec2::new(768.0, 588.0), Vec2::new(142.0, 109.0)),
        999.31555
    );

    assert_delta!(
        aurora.path(Vec2::new(363.0, 662.0), Vec2::new(736.0, 120.0)),
        1006.4688
    );

    assert_delta!(
        aurora.path(Vec2::new(901.0, 655.0), Vec2::new(208.0, 222.0)),
        1002.1547
    );

    assert_delta!(
        aurora.path(Vec2::new(959.0, 286.0), Vec2::new(101.0, 552.0)),
        1003.2709
    );

    assert_delta!(
        aurora.path(Vec2::new(592.0, 685.0), Vec2::new(523.0, 28.0)),
        1009.886
    );

    assert_delta!(
        aurora.path(Vec2::new(392.0, 692.0), Vec2::new(620.0, 54.0)),
        1012.9085
    );

    assert_delta!(
        aurora.path(Vec2::new(743.0, 675.0), Vec2::new(58.0, 152.0)),
        1009.1445
    );

    assert_delta!(
        aurora.path(Vec2::new(318.0, 659.0), Vec2::new(789.0, 146.0)),
        1020.0086
    );

    assert_delta!(
        aurora.path(Vec2::new(813.0, 204.0), Vec2::new(114.0, 605.0)),
        1006.599
    );

    assert_delta!(
        aurora.path(Vec2::new(284.0, 698.0), Vec2::new(785.0, 220.0)),
        1010.17
    );

    assert_delta!(
        aurora.path(Vec2::new(991.0, 325.0), Vec2::new(12.0, 442.0)),
        1024.4687
    );

    assert_delta!(
        aurora.path(Vec2::new(49.0, 582.0), Vec2::new(753.0, 109.0)),
        1007.5878
    );

    assert_delta!(
        aurora.path(Vec2::new(276.0, 29.0), Vec2::new(589.0, 676.0)),
        1009.2622
    );

    assert_delta!(
        aurora.path(Vec2::new(981.0, 397.0), Vec2::new(30.0, 425.0)),
        1018.7957
    );

    assert_delta!(
        aurora.path(Vec2::new(204.0, 150.0), Vec2::new(951.0, 615.0)),
        1017.1941
    );

    assert_delta!(
        aurora.path(Vec2::new(381.0, 669.0), Vec2::new(602.0, 36.0)),
        1015.8513
    );

    assert_delta!(
        aurora.path(Vec2::new(732.0, 264.0), Vec2::new(63.0, 623.0)),
        1011.1207
    );

    assert_delta!(
        aurora.path(Vec2::new(287.0, 38.0), Vec2::new(657.0, 689.0)),
        1020.4903
    );

    assert_delta!(
        aurora.path(Vec2::new(116.0, 615.0), Vec2::new(962.0, 325.0)),
        1010.8377
    );

    assert_delta!(
        aurora.path(Vec2::new(79.0, 682.0), Vec2::new(705.0, 101.0)),
        1014.833
    );

    assert_delta!(
        aurora.path(Vec2::new(880.0, 580.0), Vec2::new(62.0, 183.0)),
        1004.4071
    );

    assert_delta!(
        aurora.path(Vec2::new(281.0, 93.0), Vec2::new(604.0, 715.0)),
        1016.5928
    );

    assert_delta!(
        aurora.path(Vec2::new(997.0, 420.0), Vec2::new(89.0, 659.0)),
        1028.8545
    );

    assert_delta!(
        aurora.path(Vec2::new(876.0, 166.0), Vec2::new(117.0, 554.0)),
        1014.7948
    );

    assert_delta!(
        aurora.path(Vec2::new(735.0, 254.0), Vec2::new(90.0, 662.0)),
        1018.8879
    );

    assert_delta!(
        aurora.path(Vec2::new(726.0, 92.0), Vec2::new(171.0, 695.0)),
        1013.5684
    );

    assert_delta!(
        aurora.path(Vec2::new(884.0, 142.0), Vec2::new(284.0, 583.0)),
        1027.3267
    );

    assert_delta!(
        aurora.path(Vec2::new(328.0, 667.0), Vec2::new(788.0, 158.0)),
        1030.0145
    );

    assert_delta!(
        aurora.path(Vec2::new(981.0, 575.0), Vec2::new(237.0, 65.0)),
        1012.0565
    );

    assert_delta!(
        aurora.path(Vec2::new(884.0, 131.0), Vec2::new(371.0, 653.0)),
        1021.6322
    );

    assert_delta!(
        aurora.path(Vec2::new(392.0, 686.0), Vec2::new(708.0, 108.0)),
        1018.3937
    );

    assert_delta!(
        aurora.path(Vec2::new(5.0, 526.0), Vec2::new(947.0, 358.0)),
        1028.2151
    );

    assert_delta!(
        aurora.path(Vec2::new(162.0, 621.0), Vec2::new(822.0, 128.0)),
        1018.9767
    );

    assert_delta!(
        aurora.path(Vec2::new(242.0, 90.0), Vec2::new(338.0, 748.0)),
        1018.9908
    );

    assert_delta!(
        aurora.path(Vec2::new(627.0, 738.0), Vec2::new(120.0, 171.0)),
        1026.9651
    );

    assert_delta!(
        aurora.path(Vec2::new(980.0, 408.0), Vec2::new(1.0, 376.0)),
        1028.7121
    );

    assert_delta!(
        aurora.path(Vec2::new(130.0, 598.0), Vec2::new(980.0, 292.0)),
        1015.4153
    );

    assert_delta!(
        aurora.path(Vec2::new(829.0, 179.0), Vec2::new(162.0, 636.0)),
        1029.1984
    );

    assert_delta!(
        aurora.path(Vec2::new(686.0, 705.0), Vec2::new(538.0, 32.0)),
        1023.8319
    );

    assert_delta!(
        aurora.path(Vec2::new(833.0, 158.0), Vec2::new(158.0, 645.0)),
        1034.0333
    );

    assert_delta!(
        aurora.path(Vec2::new(109.0, 174.0), Vec2::new(783.0, 708.0)),
        1017.7859
    );

    assert_delta!(
        aurora.path(Vec2::new(165.0, 134.0), Vec2::new(983.0, 561.0)),
        1018.7645
    );

    assert_delta!(
        aurora.path(Vec2::new(932.0, 606.0), Vec2::new(188.0, 122.0)),
        1021.7925
    );

    assert_delta!(
        aurora.path(Vec2::new(622.0, 734.0), Vec2::new(648.0, 61.0)),
        1032.5187
    );

    assert_delta!(
        aurora.path(Vec2::new(67.0, 682.0), Vec2::new(741.0, 156.0)),
        1034.5237
    );

    assert_delta!(
        aurora.path(Vec2::new(541.0, 720.0), Vec2::new(739.0, 46.0)),
        1028.0938
    );

    assert_delta!(
        aurora.path(Vec2::new(1000.0, 305.0), Vec2::new(130.0, 633.0)),
        1042.2494
    );

    assert_delta!(
        aurora.path(Vec2::new(341.0, 748.0), Vec2::new(378.0, 132.0)),
        1031.5485
    );

    assert_delta!(
        aurora.path(Vec2::new(274.0, 746.0), Vec2::new(700.0, 264.0)),
        1026.2939
    );

    assert_delta!(
        aurora.path(Vec2::new(886.0, 138.0), Vec2::new(375.0, 724.0)),
        1020.6364
    );

    assert_delta!(
        aurora.path(Vec2::new(944.0, 423.0), Vec2::new(42.0, 498.0)),
        1029.9101
    );

    assert_delta!(
        aurora.path(Vec2::new(874.0, 687.0), Vec2::new(254.0, 63.0)),
        1024.2445
    );

    assert_delta!(
        aurora.path(Vec2::new(874.0, 378.0), Vec2::new(24.0, 678.0)),
        1025.7853
    );

    assert_delta!(
        aurora.path(Vec2::new(799.0, 196.0), Vec2::new(272.0, 677.0)),
        1028.992
    );

    assert_delta!(
        aurora.path(Vec2::new(372.0, 666.0), Vec2::new(796.0, 90.0)),
        1035.3108
    );

    assert_delta!(
        aurora.path(Vec2::new(511.0, 43.0), Vec2::new(635.0, 715.0)),
        1036.4987
    );

    assert_delta!(
        aurora.path(Vec2::new(285.0, 702.0), Vec2::new(758.0, 85.0)),
        1025.3019
    );

    assert_delta!(
        aurora.path(Vec2::new(53.0, 200.0), Vec2::new(970.0, 633.0)),
        1018.7662
    );

    assert_delta!(
        aurora.path(Vec2::new(470.0, 52.0), Vec2::new(385.0, 719.0)),
        1039.9285
    );

    assert_delta!(
        aurora.path(Vec2::new(946.0, 635.0), Vec2::new(222.0, 123.0)),
        1028.4326
    );

    assert_delta!(
        aurora.path(Vec2::new(942.0, 564.0), Vec2::new(147.0, 144.0)),
        1029.0294
    );

    assert_delta!(
        aurora.path(Vec2::new(635.0, 739.0), Vec2::new(155.0, 120.0)),
        1033.738
    );

    assert_delta!(
        aurora.path(Vec2::new(629.0, 736.0), Vec2::new(186.0, 110.0)),
        1040.1032
    );

    assert_delta!(
        aurora.path(Vec2::new(76.0, 663.0), Vec2::new(759.0, 130.0)),
        1038.0811
    );

    assert_delta!(
        aurora.path(Vec2::new(625.0, 728.0), Vec2::new(81.0, 147.0)),
        1036.1352
    );

    assert_delta!(
        aurora.path(Vec2::new(580.0, 685.0), Vec2::new(181.0, 79.0)),
        1038.3029
    );

    assert_delta!(
        aurora.path(Vec2::new(913.0, 533.0), Vec2::new(106.0, 123.0)),
        1030.9363
    );

    assert_delta!(
        aurora.path(Vec2::new(71.0, 132.0), Vec2::new(621.0, 724.0)),
        1036.9646
    );

    assert_delta!(
        aurora.path(Vec2::new(49.0, 510.0), Vec2::new(1020.0, 312.0)),
        1042.9785
    );

    assert_delta!(
        aurora.path(Vec2::new(800.0, 196.0), Vec2::new(119.0, 657.0)),
        1042.7508
    );

    assert_delta!(
        aurora.path(Vec2::new(751.0, 691.0), Vec2::new(74.0, 187.0)),
        1028.1199
    );

    assert_delta!(
        aurora.path(Vec2::new(914.0, 248.0), Vec2::new(151.0, 699.0)),
        1025.585
    );

    assert_delta!(
        aurora.path(Vec2::new(817.0, 245.0), Vec2::new(189.0, 695.0)),
        1033.2575
    );

    assert_delta!(
        aurora.path(Vec2::new(999.0, 304.0), Vec2::new(26.0, 521.0)),
        1047.6106
    );

    assert_delta!(
        aurora.path(Vec2::new(995.0, 341.0), Vec2::new(7.0, 483.0)),
        1042.689
    );

    assert_delta!(
        aurora.path(Vec2::new(593.0, 721.0), Vec2::new(192.0, 131.0)),
        1039.5355
    );

    assert_delta!(
        aurora.path(Vec2::new(189.0, 114.0), Vec2::new(736.0, 685.0)),
        1037.6433
    );

    assert_delta!(
        aurora.path(Vec2::new(75.0, 686.0), Vec2::new(752.0, 175.0)),
        1049.59
    );

    assert_delta!(
        aurora.path(Vec2::new(742.0, 126.0), Vec2::new(181.0, 727.0)),
        1040.2008
    );

    assert_delta!(
        aurora.path(Vec2::new(135.0, 680.0), Vec2::new(794.0, 190.0)),
        1043.9505
    );

    assert_delta!(
        aurora.path(Vec2::new(630.0, 60.0), Vec2::new(398.0, 707.0)),
        1043.8218
    );

    assert_delta!(
        aurora.path(Vec2::new(814.0, 181.0), Vec2::new(127.0, 638.0)),
        1041.7375
    );

    assert_delta!(
        aurora.path(Vec2::new(704.0, 66.0), Vec2::new(0.0, 614.0)),
        1029.4557
    );

    assert_delta!(
        aurora.path(Vec2::new(714.0, 230.0), Vec2::new(18.0, 669.0)),
        1038.0273
    );

    assert_delta!(
        aurora.path(Vec2::new(429.0, 703.0), Vec2::new(521.0, 32.0)),
        1045.7003
    );

    assert_delta!(
        aurora.path(Vec2::new(856.0, 156.0), Vec2::new(228.0, 642.0)),
        1057.4308
    );

    assert_delta!(
        aurora.path(Vec2::new(899.0, 159.0), Vec2::new(109.0, 544.0)),
        1041.8984
    );

    assert_delta!(
        aurora.path(Vec2::new(916.0, 561.0), Vec2::new(142.0, 97.0)),
        1033.6165
    );

    assert_delta!(
        aurora.path(Vec2::new(189.0, 137.0), Vec2::new(953.0, 623.0)),
        1040.3236
    );

    assert_delta!(
        aurora.path(Vec2::new(154.0, 149.0), Vec2::new(924.0, 615.0)),
        1038.5857
    );

    assert_delta!(
        aurora.path(Vec2::new(304.0, 654.0), Vec2::new(808.0, 120.0)),
        1039.4372
    );

    assert_delta!(
        aurora.path(Vec2::new(396.0, 110.0), Vec2::new(335.0, 738.0)),
        1046.1984
    );

    assert_delta!(
        aurora.path(Vec2::new(763.0, 279.0), Vec2::new(53.0, 592.0)),
        1036.1649
    );

    assert_delta!(
        aurora.path(Vec2::new(40.0, 649.0), Vec2::new(748.0, 98.0)),
        1043.5326
    );

    assert_delta!(
        aurora.path(Vec2::new(115.0, 114.0), Vec2::new(671.0, 718.0)),
        1045.7259
    );

    assert_delta!(
        aurora.path(Vec2::new(801.0, 699.0), Vec2::new(102.0, 134.0)),
        1037.3738
    );

    assert_delta!(
        aurora.path(Vec2::new(148.0, 668.0), Vec2::new(972.0, 294.0)),
        1050.5965
    );

    assert_delta!(
        aurora.path(Vec2::new(870.0, 125.0), Vec2::new(129.0, 590.0)),
        1051.3635
    );

    assert_delta!(
        aurora.path(Vec2::new(0.0, 473.0), Vec2::new(905.0, 171.0)),
        1048.11
    );

    assert_delta!(
        aurora.path(Vec2::new(812.0, 271.0), Vec2::new(118.0, 649.0)),
        1044.3065
    );

    assert_delta!(
        aurora.path(Vec2::new(34.0, 460.0), Vec2::new(1008.0, 345.0)),
        1055.8405
    );

    assert_delta!(
        aurora.path(Vec2::new(113.0, 644.0), Vec2::new(790.0, 76.0)),
        1040.8908
    );

    assert_delta!(
        aurora.path(Vec2::new(655.0, 720.0), Vec2::new(261.0, 29.0)),
        1048.328
    );

    assert_delta!(
        aurora.path(Vec2::new(395.0, 627.0), Vec2::new(791.0, 62.0)),
        1045.6776
    );

    assert_delta!(
        aurora.path(Vec2::new(214.0, 642.0), Vec2::new(865.0, 147.0)),
        1058.5512
    );

    assert_delta!(
        aurora.path(Vec2::new(89.0, 561.0), Vec2::new(886.0, 172.0)),
        1046.5966
    );

    assert_delta!(
        aurora.path(Vec2::new(117.0, 147.0), Vec2::new(856.0, 694.0)),
        1044.3686
    );

    assert_delta!(
        aurora.path(Vec2::new(220.0, 73.0), Vec2::new(902.0, 655.0)),
        1046.2102
    );

    assert_delta!(
        aurora.path(Vec2::new(609.0, 715.0), Vec2::new(236.0, 83.0)),
        1049.9614
    );

    assert_delta!(
        aurora.path(Vec2::new(608.0, 711.0), Vec2::new(83.0, 137.0)),
        1047.461
    );

    assert_delta!(
        aurora.path(Vec2::new(71.0, 530.0), Vec2::new(1015.0, 318.0)),
        1054.8325
    );

    assert_delta!(
        aurora.path(Vec2::new(112.0, 173.0), Vec2::new(934.0, 561.0)),
        1051.85
    );

    assert_delta!(
        aurora.path(Vec2::new(1002.0, 425.0), Vec2::new(74.0, 688.0)),
        1057.3649
    );

    assert_delta!(
        aurora.path(Vec2::new(361.0, 735.0), Vec2::new(465.0, 63.0)),
        1057.2282
    );

    assert_delta!(
        aurora.path(Vec2::new(1014.0, 306.0), Vec2::new(116.0, 674.0)),
        1062.6103
    );

    assert_delta!(
        aurora.path(Vec2::new(588.0, 721.0), Vec2::new(659.0, 66.0)),
        1050.7028
    );

    assert_delta!(
        aurora.path(Vec2::new(809.0, 102.0), Vec2::new(58.0, 567.0)),
        1050.5933
    );

    assert_delta!(
        aurora.path(Vec2::new(68.0, 550.0), Vec2::new(899.0, 162.0)),
        1059.9533
    );

    assert_delta!(
        aurora.path(Vec2::new(857.0, 690.0), Vec2::new(76.0, 160.0)),
        1046.0208
    );

    assert_delta!(
        aurora.path(Vec2::new(836.0, 250.0), Vec2::new(136.0, 703.0)),
        1046.3291
    );

    assert_delta!(
        aurora.path(Vec2::new(736.0, 123.0), Vec2::new(14.0, 655.0)),
        1051.9877
    );

    assert_delta!(
        aurora.path(Vec2::new(32.0, 165.0), Vec2::new(928.0, 650.0)),
        1042.3754
    );

    assert_delta!(
        aurora.path(Vec2::new(329.0, 743.0), Vec2::new(414.0, 79.0)),
        1059.1344
    );

    assert_delta!(
        aurora.path(Vec2::new(869.0, 235.0), Vec2::new(16.0, 639.0)),
        1049.7446
    );

    assert_delta!(
        aurora.path(Vec2::new(841.0, 144.0), Vec2::new(127.0, 658.0)),
        1073.0762
    );

    assert_delta!(
        aurora.path(Vec2::new(148.0, 563.0), Vec2::new(940.0, 153.0)),
        1047.8525
    );

    assert_delta!(
        aurora.path(Vec2::new(179.0, 130.0), Vec2::new(970.0, 606.0)),
        1052.5179
    );

    assert_delta!(
        aurora.path(Vec2::new(172.0, 697.0), Vec2::new(770.0, 238.0)),
        1052.5848
    );

    assert_delta!(
        aurora.path(Vec2::new(48.0, 664.0), Vec2::new(927.0, 329.0)),
        1057.0439
    );

    assert_delta!(
        aurora.path(Vec2::new(677.0, 714.0), Vec2::new(555.0, 19.0)),
        1057.1375
    );

    assert_delta!(
        aurora.path(Vec2::new(130.0, 656.0), Vec2::new(820.0, 88.0)),
        1060.0573
    );

    assert_delta!(
        aurora.path(Vec2::new(838.0, 99.0), Vec2::new(83.0, 561.0)),
        1053.9903
    );

    assert_delta!(
        aurora.path(Vec2::new(384.0, 699.0), Vec2::new(547.0, 27.0)),
        1057.9248
    );

    assert_delta!(
        aurora.path(Vec2::new(796.0, 98.0), Vec2::new(309.0, 695.0)),
        1056.4009
    );

    assert_delta!(
        aurora.path(Vec2::new(940.0, 433.0), Vec2::new(15.0, 167.0)),
        1066.2878
    );

    assert_delta!(
        aurora.path(Vec2::new(0.0, 607.0), Vec2::new(873.0, 235.0)),
        1054.2336
    );

    assert_delta!(
        aurora.path(Vec2::new(753.0, 76.0), Vec2::new(132.0, 724.0)),
        1055.0198
    );

    assert_delta!(
        aurora.path(Vec2::new(872.0, 136.0), Vec2::new(229.0, 635.0)),
        1071.2331
    );

    assert_delta!(
        aurora.path(Vec2::new(363.0, 102.0), Vec2::new(335.0, 745.0)),
        1068.0746
    );

    assert_delta!(
        aurora.path(Vec2::new(960.0, 587.0), Vec2::new(11.0, 168.0)),
        1052.2824
    );

    assert_delta!(
        aurora.path(Vec2::new(776.0, 198.0), Vec2::new(141.0, 727.0)),
        1063.5421
    );

    assert_delta!(
        aurora.path(Vec2::new(582.0, 34.0), Vec2::new(638.0, 703.0)),
        1063.2197
    );

    assert_delta!(
        aurora.path(Vec2::new(354.0, 707.0), Vec2::new(908.0, 139.0)),
        1058.5008
    );

    assert_delta!(
        aurora.path(Vec2::new(1017.0, 433.0), Vec2::new(34.0, 424.0)),
        1064.2754
    );

    assert_delta!(
        aurora.path(Vec2::new(347.0, 731.0), Vec2::new(168.0, 97.0)),
        1060.2302
    );

    assert_delta!(
        aurora.path(Vec2::new(725.0, 278.0), Vec2::new(14.0, 642.0)),
        1057.5821
    );

    assert_delta!(
        aurora.path(Vec2::new(752.0, 138.0), Vec2::new(1.0, 631.0)),
        1063.6049
    );

    assert_delta!(
        aurora.path(Vec2::new(8.0, 581.0), Vec2::new(881.0, 133.0)),
        1071.9477
    );

    assert_delta!(
        aurora.path(Vec2::new(821.0, 147.0), Vec2::new(280.0, 724.0)),
        1062.9317
    );

    assert_delta!(
        aurora.path(Vec2::new(746.0, 248.0), Vec2::new(288.0, 733.0)),
        1061.3282
    );

    assert_delta!(
        aurora.path(Vec2::new(762.0, 253.0), Vec2::new(111.0, 706.0)),
        1061.167
    );

    assert_delta!(
        aurora.path(Vec2::new(756.0, 227.0), Vec2::new(14.0, 621.0)),
        1061.1829
    );

    assert_delta!(
        aurora.path(Vec2::new(318.0, 722.0), Vec2::new(861.0, 125.0)),
        1068.3091
    );

    assert_delta!(
        aurora.path(Vec2::new(69.0, 650.0), Vec2::new(942.0, 268.0)),
        1060.3116
    );

    assert_delta!(
        aurora.path(Vec2::new(964.0, 435.0), Vec2::new(49.0, 451.0)),
        1069.0434
    );

    assert_delta!(
        aurora.path(Vec2::new(939.0, 571.0), Vec2::new(103.0, 174.0)),
        1070.1654
    );

    assert_delta!(
        aurora.path(Vec2::new(832.0, 205.0), Vec2::new(141.0, 726.0)),
        1071.6588
    );

    assert_delta!(
        aurora.path(Vec2::new(76.0, 130.0), Vec2::new(927.0, 564.0)),
        1062.6823
    );

    assert_delta!(
        aurora.path(Vec2::new(926.0, 256.0), Vec2::new(37.0, 615.0)),
        1057.5907
    );

    assert_delta!(
        aurora.path(Vec2::new(780.0, 725.0), Vec2::new(99.0, 202.0)),
        1063.6161
    );

    assert_delta!(
        aurora.path(Vec2::new(980.0, 588.0), Vec2::new(155.0, 165.0)),
        1064.7545
    );

    assert_delta!(
        aurora.path(Vec2::new(18.0, 587.0), Vec2::new(984.0, 439.0)),
        1067.1807
    );

    assert_delta!(
        aurora.path(Vec2::new(40.0, 627.0), Vec2::new(756.0, 60.0)),
        1060.2935
    );

    assert_delta!(
        aurora.path(Vec2::new(80.0, 687.0), Vec2::new(769.0, 120.0)),
        1076.2361
    );

    assert_delta!(
        aurora.path(Vec2::new(23.0, 657.0), Vec2::new(931.0, 354.0)),
        1071.8706
    );

    assert_delta!(
        aurora.path(Vec2::new(337.0, 746.0), Vec2::new(160.0, 103.0)),
        1066.6171
    );

    assert_delta!(
        aurora.path(Vec2::new(808.0, 118.0), Vec2::new(335.0, 666.0)),
        1069.7688
    );

    assert_delta!(
        aurora.path(Vec2::new(897.0, 654.0), Vec2::new(114.0, 192.0)),
        1063.7989
    );

    assert_delta!(
        aurora.path(Vec2::new(34.0, 677.0), Vec2::new(1020.0, 583.0)),
        1082.4165
    );

    assert_delta!(
        aurora.path(Vec2::new(987.0, 623.0), Vec2::new(204.0, 100.0)),
        1067.4429
    );

    assert_delta!(
        aurora.path(Vec2::new(751.0, 78.0), Vec2::new(96.0, 692.0)),
        1068.0877
    );

    assert_delta!(
        aurora.path(Vec2::new(65.0, 141.0), Vec2::new(900.0, 674.0)),
        1063.7702
    );

    assert_delta!(
        aurora.path(Vec2::new(366.0, 727.0), Vec2::new(684.0, 122.0)),
        1074.1009
    );

    assert_delta!(
        aurora.path(Vec2::new(57.0, 448.0), Vec2::new(1002.0, 428.0)),
        1077.3664
    );

    assert_delta!(
        aurora.path(Vec2::new(725.0, 48.0), Vec2::new(9.0, 662.0)),
        1071.0622
    );

    assert_delta!(
        aurora.path(Vec2::new(71.0, 170.0), Vec2::new(930.0, 619.0)),
        1061.8972
    );

    assert_delta!(
        aurora.path(Vec2::new(866.0, 116.0), Vec2::new(155.0, 654.0)),
        1085.53
    );

    assert_delta!(
        aurora.path(Vec2::new(52.0, 639.0), Vec2::new(800.0, 183.0)),
        1076.9483
    );

    assert_delta!(
        aurora.path(Vec2::new(123.0, 162.0), Vec2::new(958.0, 582.0)),
        1074.6262
    );

    assert_delta!(
        aurora.path(Vec2::new(67.0, 690.0), Vec2::new(788.0, 158.0)),
        1089.307
    );

    assert_delta!(
        aurora.path(Vec2::new(246.0, 722.0), Vec2::new(794.0, 88.0)),
        1072.6185
    );

    assert_delta!(
        aurora.path(Vec2::new(29.0, 605.0), Vec2::new(781.0, 83.0)),
        1068.2169
    );

    assert_delta!(
        aurora.path(Vec2::new(758.0, 277.0), Vec2::new(182.0, 721.0)),
        1075.1892
    );

    assert_delta!(
        aurora.path(Vec2::new(136.0, 105.0), Vec2::new(772.0, 680.0)),
        1073.0619
    );

    assert_delta!(
        aurora.path(Vec2::new(3.0, 644.0), Vec2::new(745.0, 241.0)),
        1070.4251
    );

    assert_delta!(
        aurora.path(Vec2::new(57.0, 642.0), Vec2::new(813.0, 191.0)),
        1085.155
    );

    assert_delta!(
        aurora.path(Vec2::new(826.0, 84.0), Vec2::new(151.0, 681.0)),
        1074.3721
    );

    assert_delta!(
        aurora.path(Vec2::new(232.0, 685.0), Vec2::new(852.0, 133.0)),
        1089.0184
    );

    assert_delta!(
        aurora.path(Vec2::new(789.0, 218.0), Vec2::new(155.0, 731.0)),
        1079.1427
    );

    assert_delta!(
        aurora.path(Vec2::new(47.0, 685.0), Vec2::new(993.0, 341.0)),
        1095.0758
    );

    assert_delta!(
        aurora.path(Vec2::new(179.0, 117.0), Vec2::new(1000.0, 598.0)),
        1074.3074
    );

    assert_delta!(
        aurora.path(Vec2::new(116.0, 705.0), Vec2::new(847.0, 195.0)),
        1074.189
    );

    assert_delta!(
        aurora.path(Vec2::new(864.0, 108.0), Vec2::new(238.0, 662.0)),
        1088.5511
    );

    assert_delta!(
        aurora.path(Vec2::new(36.0, 646.0), Vec2::new(802.0, 140.0)),
        1088.3798
    );

    assert_delta!(
        aurora.path(Vec2::new(989.0, 598.0), Vec2::new(152.0, 168.0)),
        1079.792
    );

    assert_delta!(
        aurora.path(Vec2::new(311.0, 699.0), Vec2::new(819.0, 102.0)),
        1083.4738
    );

    assert_delta!(
        aurora.path(Vec2::new(881.0, 230.0), Vec2::new(0.0, 641.0)),
        1074.0278
    );

    assert_delta!(
        aurora.path(Vec2::new(961.0, 447.0), Vec2::new(55.0, 453.0)),
        1085.2016
    );

    assert_delta!(
        aurora.path(Vec2::new(158.0, 125.0), Vec2::new(952.0, 644.0)),
        1086.9142
    );

    assert_delta!(
        aurora.path(Vec2::new(846.0, 105.0), Vec2::new(65.0, 566.0)),
        1080.5805
    );

    assert_delta!(
        aurora.path(Vec2::new(812.0, 91.0), Vec2::new(152.0, 701.0)),
        1077.9138
    );

    assert_delta!(
        aurora.path(Vec2::new(159.0, 705.0), Vec2::new(814.0, 177.0)),
        1085.3328
    );

    assert_delta!(
        aurora.path(Vec2::new(110.0, 121.0), Vec2::new(738.0, 684.0)),
        1075.9822
    );

    assert_delta!(
        aurora.path(Vec2::new(788.0, 159.0), Vec2::new(82.0, 703.0)),
        1098.8684
    );

    assert_delta!(
        aurora.path(Vec2::new(72.0, 664.0), Vec2::new(804.0, 216.0)),
        1086.3925
    );

    assert_delta!(
        aurora.path(Vec2::new(78.0, 609.0), Vec2::new(847.0, 116.0)),
        1088.4016
    );

    assert_delta!(
        aurora.path(Vec2::new(250.0, 684.0), Vec2::new(941.0, 151.0)),
        1087.6429
    );

    assert_delta!(
        aurora.path(Vec2::new(569.0, 24.0), Vec2::new(334.0, 715.0)),
        1092.3505
    );

    assert_delta!(
        aurora.path(Vec2::new(826.0, 90.0), Vec2::new(276.0, 695.0)),
        1084.4033
    );

    assert_delta!(
        aurora.path(Vec2::new(333.0, 755.0), Vec2::new(739.0, 116.0)),
        1079.5976
    );

    assert_delta!(
        aurora.path(Vec2::new(78.0, 689.0), Vec2::new(784.0, 196.0)),
        1091.8364
    );

    assert_delta!(
        aurora.path(Vec2::new(130.0, 717.0), Vec2::new(781.0, 59.0)),
        1082.2023
    );

    assert_delta!(
        aurora.path(Vec2::new(380.0, 78.0), Vec2::new(327.0, 750.0)),
        1090.6357
    );

    assert_delta!(
        aurora.path(Vec2::new(769.0, 69.0), Vec2::new(3.0, 594.0)),
        1081.7998
    );

    assert_delta!(
        aurora.path(Vec2::new(3.0, 659.0), Vec2::new(925.0, 344.0)),
        1085.8277
    );

    assert_delta!(
        aurora.path(Vec2::new(930.0, 307.0), Vec2::new(7.0, 635.0)),
        1080.6315
    );

    assert_delta!(
        aurora.path(Vec2::new(851.0, 157.0), Vec2::new(139.0, 692.0)),
        1101.7203
    );

    assert_delta!(
        aurora.path(Vec2::new(644.0, 148.0), Vec2::new(304.0, 748.0)),
        1093.6667
    );

    assert_delta!(
        aurora.path(Vec2::new(137.0, 701.0), Vec2::new(841.0, 148.0)),
        1100.8814
    );

    assert_delta!(
        aurora.path(Vec2::new(9.0, 518.0), Vec2::new(935.0, 150.0)),
        1093.32
    );

    assert_delta!(
        aurora.path(Vec2::new(17.0, 661.0), Vec2::new(777.0, 210.0)),
        1091.5112
    );

    assert_delta!(
        aurora.path(Vec2::new(1021.0, 350.0), Vec2::new(49.0, 682.0)),
        1105.1879
    );

    assert_delta!(
        aurora.path(Vec2::new(56.0, 673.0), Vec2::new(1007.0, 321.0)),
        1105.7451
    );

    assert_delta!(
        aurora.path(Vec2::new(840.0, 182.0), Vec2::new(113.0, 717.0)),
        1095.2473
    );

    assert_delta!(
        aurora.path(Vec2::new(83.0, 697.0), Vec2::new(957.0, 303.0)),
        1108.7885
    );

    assert_delta!(
        aurora.path(Vec2::new(68.0, 661.0), Vec2::new(821.0, 174.0)),
        1104.4252
    );

    assert_delta!(
        aurora.path(Vec2::new(48.0, 643.0), Vec2::new(798.0, 207.0)),
        1093.8601
    );

    assert_delta!(
        aurora.path(Vec2::new(357.0, 735.0), Vec2::new(752.0, 73.0)),
        1090.8151
    );

    assert_delta!(
        aurora.path(Vec2::new(0.0, 617.0), Vec2::new(754.0, 279.0)),
        1085.8279
    );

    assert_delta!(
        aurora.path(Vec2::new(783.0, 186.0), Vec2::new(59.0, 695.0)),
        1099.823
    );

    assert_delta!(
        aurora.path(Vec2::new(352.0, 735.0), Vec2::new(631.0, 56.0)),
        1099.2037
    );

    assert_delta!(
        aurora.path(Vec2::new(819.0, 79.0), Vec2::new(89.0, 650.0)),
        1091.1437
    );

    assert_delta!(
        aurora.path(Vec2::new(76.0, 620.0), Vec2::new(863.0, 141.0)),
        1101.2165
    );

    assert_delta!(
        aurora.path(Vec2::new(903.0, 155.0), Vec2::new(184.0, 718.0)),
        1106.5873
    );

    assert_delta!(
        aurora.path(Vec2::new(812.0, 95.0), Vec2::new(150.0, 721.0)),
        1097.1694
    );

    assert_delta!(
        aurora.path(Vec2::new(1009.0, 402.0), Vec2::new(0.0, 635.0)),
        1096.4528
    );

    assert_delta!(
        aurora.path(Vec2::new(305.0, 719.0), Vec2::new(777.0, 100.0)),
        1097.7203
    );

    assert_delta!(
        aurora.path(Vec2::new(84.0, 621.0), Vec2::new(901.0, 160.0)),
        1093.2823
    );

    assert_delta!(
        aurora.path(Vec2::new(128.0, 178.0), Vec2::new(960.0, 640.0)),
        1085.7275
    );

    assert_delta!(
        aurora.path(Vec2::new(100.0, 158.0), Vec2::new(943.0, 596.0)),
        1086.1153
    );

    assert_delta!(
        aurora.path(Vec2::new(205.0, 60.0), Vec2::new(599.0, 713.0)),
        1096.3202
    );

    assert_delta!(
        aurora.path(Vec2::new(874.0, 170.0), Vec2::new(40.0, 586.0)),
        1093.7328
    );

    assert_delta!(
        aurora.path(Vec2::new(79.0, 685.0), Vec2::new(946.0, 295.0)),
        1091.2468
    );

    assert_delta!(
        aurora.path(Vec2::new(316.0, 736.0), Vec2::new(815.0, 86.0)),
        1103.6204
    );

    assert_delta!(
        aurora.path(Vec2::new(317.0, 754.0), Vec2::new(657.0, 131.0)),
        1101.3279
    );

    assert_delta!(
        aurora.path(Vec2::new(323.0, 745.0), Vec2::new(653.0, 82.0)),
        1104.373
    );

    assert_delta!(
        aurora.path(Vec2::new(819.0, 253.0), Vec2::new(3.0, 596.0)),
        1094.1186
    );

    assert_delta!(
        aurora.path(Vec2::new(762.0, 73.0), Vec2::new(353.0, 742.0)),
        1096.8566
    );

    assert_delta!(
        aurora.path(Vec2::new(788.0, 70.0), Vec2::new(103.0, 699.0)),
        1097.3119
    );

    assert_delta!(
        aurora.path(Vec2::new(16.0, 627.0), Vec2::new(792.0, 106.0)),
        1096.4103
    );

    assert_delta!(
        aurora.path(Vec2::new(65.0, 642.0), Vec2::new(987.0, 297.0)),
        1102.0939
    );

    assert_delta!(
        aurora.path(Vec2::new(9.0, 667.0), Vec2::new(955.0, 437.0)),
        1104.0966
    );

    assert_delta!(
        aurora.path(Vec2::new(851.0, 172.0), Vec2::new(53.0, 613.0)),
        1108.4147
    );

    assert_delta!(
        aurora.path(Vec2::new(40.0, 670.0), Vec2::new(786.0, 275.0)),
        1110.2192
    );

    assert_delta!(
        aurora.path(Vec2::new(775.0, 717.0), Vec2::new(177.0, 75.0)),
        1100.4302
    );

    assert_delta!(
        aurora.path(Vec2::new(768.0, 260.0), Vec2::new(1.0, 633.0)),
        1096.6247
    );

    assert_delta!(
        aurora.path(Vec2::new(151.0, 733.0), Vec2::new(822.0, 168.0)),
        1114.6805
    );

    assert_delta!(
        aurora.path(Vec2::new(518.0, 35.0), Vec2::new(362.0, 723.0)),
        1111.2067
    );

    assert_delta!(
        aurora.path(Vec2::new(49.0, 665.0), Vec2::new(811.0, 107.0)),
        1107.5912
    );

    assert_delta!(
        aurora.path(Vec2::new(599.0, 717.0), Vec2::new(579.0, 34.0)),
        1106.6263
    );

    assert_delta!(
        aurora.path(Vec2::new(870.0, 160.0), Vec2::new(268.0, 689.0)),
        1115.7576
    );

    assert_delta!(
        aurora.path(Vec2::new(263.0, 728.0), Vec2::new(812.0, 79.0)),
        1104.9207
    );

    assert_delta!(
        aurora.path(Vec2::new(853.0, 187.0), Vec2::new(37.0, 658.0)),
        1100.2835
    );

    assert_delta!(
        aurora.path(Vec2::new(1023.0, 414.0), Vec2::new(3.0, 582.0)),
        1114.0306
    );

    assert_delta!(
        aurora.path(Vec2::new(966.0, 626.0), Vec2::new(122.0, 160.0)),
        1097.3395
    );

    assert_delta!(
        aurora.path(Vec2::new(884.0, 144.0), Vec2::new(79.0, 619.0)),
        1118.86
    );

    assert_delta!(
        aurora.path(Vec2::new(47.0, 617.0), Vec2::new(996.0, 293.0)),
        1109.2477
    );

    assert_delta!(
        aurora.path(Vec2::new(226.0, 720.0), Vec2::new(850.0, 142.0)),
        1118.3891
    );

    assert_delta!(
        aurora.path(Vec2::new(92.0, 709.0), Vec2::new(853.0, 250.0)),
        1103.0609
    );

    assert_delta!(
        aurora.path(Vec2::new(987.0, 618.0), Vec2::new(41.0, 147.0)),
        1096.2784
    );

    assert_delta!(
        aurora.path(Vec2::new(776.0, 255.0), Vec2::new(5.0, 644.0)),
        1102.6386
    );

    assert_delta!(
        aurora.path(Vec2::new(869.0, 132.0), Vec2::new(329.0, 672.0)),
        1116.5478
    );

    assert_delta!(
        aurora.path(Vec2::new(940.0, 150.0), Vec2::new(153.0, 653.0)),
        1107.1249
    );

    assert_delta!(
        aurora.path(Vec2::new(821.0, 194.0), Vec2::new(89.0, 688.0)),
        1112.4352
    );

    assert_delta!(
        aurora.path(Vec2::new(75.0, 144.0), Vec2::new(971.0, 589.0)),
        1103.8122
    );

    assert_delta!(
        aurora.path(Vec2::new(609.0, 730.0), Vec2::new(555.0, 17.0)),
        1117.4709
    );

    assert_delta!(
        aurora.path(Vec2::new(16.0, 611.0), Vec2::new(819.0, 173.0)),
        1116.5963
    );

    assert_delta!(
        aurora.path(Vec2::new(96.0, 176.0), Vec2::new(952.0, 623.0)),
        1104.2441
    );

    assert_delta!(
        aurora.path(Vec2::new(26.0, 649.0), Vec2::new(787.0, 276.0)),
        1110.3193
    );

    assert_delta!(
        aurora.path(Vec2::new(810.0, 138.0), Vec2::new(12.0, 644.0)),
        1116.0613
    );

    assert_delta!(
        aurora.path(Vec2::new(887.0, 119.0), Vec2::new(238.0, 684.0)),
        1122.6222
    );

    assert_delta!(
        aurora.path(Vec2::new(835.0, 179.0), Vec2::new(145.0, 726.0)),
        1123.3291
    );

    assert_delta!(
        aurora.path(Vec2::new(117.0, 670.0), Vec2::new(872.0, 112.0)),
        1129.7237
    );

    assert_delta!(
        aurora.path(Vec2::new(854.0, 679.0), Vec2::new(144.0, 107.0)),
        1108.788
    );

    assert_delta!(
        aurora.path(Vec2::new(779.0, 285.0), Vec2::new(49.0, 690.0)),
        1114.7555
    );

    assert_delta!(
        aurora.path(Vec2::new(50.0, 636.0), Vec2::new(850.0, 123.0)),
        1126.6631
    );

    assert_delta!(
        aurora.path(Vec2::new(34.0, 672.0), Vec2::new(957.0, 276.0)),
        1111.8828
    );

    assert_delta!(
        aurora.path(Vec2::new(62.0, 683.0), Vec2::new(858.0, 178.0)),
        1114.6736
    );

    assert_delta!(
        aurora.path(Vec2::new(895.0, 134.0), Vec2::new(293.0, 677.0)),
        1123.073
    );

    assert_delta!(
        aurora.path(Vec2::new(830.0, 156.0), Vec2::new(77.0, 689.0)),
        1131.2928
    );

    assert_delta!(
        aurora.path(Vec2::new(809.0, 176.0), Vec2::new(41.0, 680.0)),
        1122.342
    );

    assert_delta!(
        aurora.path(Vec2::new(98.0, 191.0), Vec2::new(934.0, 652.0)),
        1104.4646
    );

    assert_delta!(
        aurora.path(Vec2::new(793.0, 78.0), Vec2::new(89.0, 684.0)),
        1114.1801
    );

    assert_delta!(
        aurora.path(Vec2::new(331.0, 671.0), Vec2::new(866.0, 103.0)),
        1122.9406
    );

    assert_delta!(
        aurora.path(Vec2::new(52.0, 614.0), Vec2::new(874.0, 160.0)),
        1129.4693
    );

    assert_delta!(
        aurora.path(Vec2::new(979.0, 590.0), Vec2::new(98.0, 191.0)),
        1120.2311
    );

    assert_delta!(
        aurora.path(Vec2::new(104.0, 166.0), Vec2::new(961.0, 633.0)),
        1110.02
    );

    assert_delta!(
        aurora.path(Vec2::new(875.0, 691.0), Vec2::new(121.0, 107.0)),
        1114.0472
    );

    assert_delta!(
        aurora.path(Vec2::new(67.0, 569.0), Vec2::new(904.0, 144.0)),
        1127.4946
    );

    assert_delta!(
        aurora.path(Vec2::new(843.0, 156.0), Vec2::new(59.0, 675.0)),
        1131.6711
    );

    assert_delta!(
        aurora.path(Vec2::new(284.0, 674.0), Vec2::new(896.0, 119.0)),
        1125.2731
    );

    assert_delta!(
        aurora.path(Vec2::new(991.0, 589.0), Vec2::new(61.0, 140.0)),
        1113.5999
    );

    assert_delta!(
        aurora.path(Vec2::new(808.0, 207.0), Vec2::new(10.0, 627.0)),
        1112.4008
    );

    assert_delta!(
        aurora.path(Vec2::new(855.0, 144.0), Vec2::new(27.0, 600.0)),
        1127.1473
    );

    assert_delta!(
        aurora.path(Vec2::new(596.0, 722.0), Vec2::new(565.0, 34.0)),
        1123.7803
    );

    assert_delta!(
        aurora.path(Vec2::new(182.0, 723.0), Vec2::new(847.0, 148.0)),
        1137.4998
    );

    assert_delta!(
        aurora.path(Vec2::new(145.0, 97.0), Vec2::new(969.0, 616.0)),
        1115.7011
    );

    assert_delta!(
        aurora.path(Vec2::new(718.0, 682.0), Vec2::new(134.0, 115.0)),
        1119.0245
    );

    assert_delta!(
        aurora.path(Vec2::new(856.0, 192.0), Vec2::new(5.0, 643.0)),
        1118.2746
    );

    assert_delta!(
        aurora.path(Vec2::new(1.0, 576.0), Vec2::new(928.0, 151.0)),
        1132.1721
    );

    assert_delta!(
        aurora.path(Vec2::new(950.0, 605.0), Vec2::new(94.0, 119.0)),
        1113.1997
    );

    assert_delta!(
        aurora.path(Vec2::new(891.0, 163.0), Vec2::new(54.0, 645.0)),
        1121.9482
    );

    assert_delta!(
        aurora.path(Vec2::new(993.0, 290.0), Vec2::new(34.0, 622.0)),
        1123.2226
    );
}
