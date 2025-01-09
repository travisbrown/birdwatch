# Community Notes

This repository contains metadata related to X's Community Notes program (formerly "Birdwatch").

## Motivation

X publishes "daily" snapshots of Community Notes data, but these snapshots do not include two useful pieces of information:

1. **User aliases:** The snapshots only show each user's "participant ID", which is a 64-character hexadecimal string, but not the three-word [alias](https://communitynotes.x.com/guide/en/contributing/aliases) that can be used to construct a link to a user's Community Notes profile. As far as I know it is not possible to link directly to a note via its note ID, or to a user via their participant ID, so the absence of aliases makes it difficult to check the current status of notes in the snapshot.
2. **Accounts posting noted tweets:** The snapshots indicate the tweet ID for each note, but not the ID (or screen name, etc.) of the account that posted the tweet. This makes it difficult to evaluate, for example, how many notes targeting Elon Musk's account are considered helpful and shown on the site (a little less than two thirds of one percent).

This repository attempts to provide as much of this missing information as possible.

## Format

All data files in this repository are in the CSV ("comma-separated values") format and can be opened in any spreadsheet application.

### Aliases

The first file schema is used by the `data/aliases.csv` file, and maps participant IDs to aliases.

```bash
$ head -n 5 data/aliases.csv
000045A5FA0CF004F68CBF2913506C37D540CF48522D33BFBF036AAC53FBDA8B,thoughtful-mint-duck
0000AE9A69E1B5D132C053E253DC42A007EDE2F11C39CF89008F447F1ABF936F,effective-juniper-bluejay
00012143F323489C81213B9D34C17FE6159338BC4CB48EB22C3FAD7742EF8144,congenial-banana-spoonbill
00018DBB934257251EBCEE91D0722C71B7DD592A57139831B4CC1BEFB0EE614E,hearty-samba-goose
0002188E5ED3028646C97CBE9ADCD12CB5B8BFAF8819BD02233645DC656EF1AD,mindful-evergreen-owl
```

This file has no column headers.
It is sorted lexicographically by the first column, and is a one-to-one mapping (no participant ID or alias appears twice).

### Missing aliases

We provide aliases for most (currently just over 96%) of the distinct participant IDs associated with notes in the snapshots.
The remaining few percent cannot currently be resolved, presumably because the Community Notes account has been deleted.

There is a second file that maps participant IDs to note IDs that the snapshots attribute to that user.
We provide this mapping because it may be possible to resolve more aliases for these in the future (using historical data or other sources).

```bash
head -n 5 workspace/unknown-aliases.csv
00034FC970AFABC3893397D8000DC1B8AD325277271DA0FC89E10CDC0B8CDCD1,1638800616637222912
000415A1E3D1DA95BD626E1D938E4A9AFFB446D1A7D53212DF1CC5A339AAEBF9,1786014873530282321
000846F219852B16C78762ABC0059ADF32A1222A754A9ABBE6EBFDD740B00B01,1773844885692711107
0016DB32296AA9EFBE2E96406EBBEBDFD57D294DB06CC82B25A3792B5AE7577F,1711670781212500284
0016DB32296AA9EFBE2E96406EBBEBDFD57D294DB06CC82B25A3792B5AE7577F,1712097857669374428
```

This file also has no headers. It is sorted lexicographically by the first column and then by the numerical value of the second column.
Each participant ID may map to many known note IDs, but each note ID will have a unique participant ID.

### Note metadata

The final file format is used by a set of files in the `data/notes` directory.
Each file includes all known notes for a particular month, indicated by the file name.

These files do each have a header that indicates the names of the columns.

```bash
$ head -n 5 data/notes/2024-08.csv
Note ID,Created at,Alias,Tweet ID,User ID,Misleading,Helpful
1818798801303253421,1722470400536,glowing-fir-whistler,1753493581866700876,3096147528,false,
1818799051443220564,1722470460173,glowing-fir-whistler,1815514603633463654,4429003533,true,
1818799135631216731,1722470480245,futuristic-reef-thicketbird,1818460037213212964,1334962540519055363,true,true
1818799164265759179,1722470487072,terrestrial-mangrove-bluebird,1818595370618044819,1202704998888624128,true,true
```

Several of the columns may have empty values. The alias column may be empty for the reasons discussed in the previous subsection.
The tweet ID column will be empty if the note only appears in the `noteStatusHistory` snapshot file, which does not provide tweet IDs.
The user ID column (which indicates the account that posted the tweet) will be empty if we have not been able to resolve the tweet (because it has been deleted, or because the account that posted it has been made private, etc.).

The final column ("Helpful") will never be empty because of missing data, but may be empty to indicate that the note has status `NEEDS_MORE_RATINGS` in the snapshot data.
A value of `true` in that column corresponds to `CURRENTLY_RATED_HELPFUL` in the snapshot data, and `false` to `CURRENTLY_RATED_NOT_HELPFUL`.

## Limitations

This repository intentionally only contains metadata. It does not include either tweet or note content.

The metadata that is here is not complete.
Approximately 12% of tweets that have received Community Notes have not been available to us during the compilation of these files.
Additionally, as noted above, some Community Notes user accounts no longer exist.
Some of these gaps may be partially filled in the future, but providing a complete record would have required beginning this project in 2021, when Birdwatch was launched.

## Most noted accounts

|          Twitter ID | Screen name     | Helpful |     % | Not helpful |     % | Needs more ratings |     % |
| ------------------: | --------------- | ------: | ----: | ----------: | ----: | -----------------: | ----: |
|            44196397 | elonmusk        |     116 |  0.64 |        2026 | 11.18 |              15972 | 88.17 |
|           629698642 | BGatesIsaPyscho |    1091 | 22.55 |         118 |  2.44 |               3629 | 75.01 |
| 1151913018936053760 | jacksonhinklle  |     361 |  8.27 |         224 |  5.13 |               3782 | 86.60 |
|          3376321847 | iluminatibot    |     755 | 18.42 |          99 |  2.42 |               3245 | 79.17 |
| 1349149096909668363 | POTUS           |      57 |  1.47 |         244 |  6.31 |               3565 | 92.21 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         103 |  2.85 |               3505 | 97.12 |
| 1552795969959636992 | EndWokeness     |      70 |  1.94 |         151 |  4.19 |               3386 | 93.87 |
|              939091 | JoeBiden        |      16 |  0.46 |         303 |  8.68 |               3170 | 90.86 |
|           191871143 | VigilantFox     |      35 |  1.02 |          49 |  1.43 |               3349 | 97.55 |
|          1291128494 | ShaykhSulaiman  |     262 |  7.76 |         159 |  4.71 |               2957 | 87.54 |
| 1222773302441148416 | visegrad24      |     164 |  5.31 |         275 |  8.91 |               2647 | 85.77 |
| 1319287761048723458 | MarioNawfal     |     192 |  6.38 |          94 |  3.12 |               2723 | 90.50 |
| 1446231057259433988 | DrLoupis        |     216 |  7.24 |         125 |  4.19 |               2643 | 88.57 |
|  953378142198161409 | choquei         |     431 | 14.79 |          70 |  2.40 |               2413 | 82.81 |
| 1326229737551912960 | libsoftiktok    |      44 |  1.56 |          89 |  3.15 |               2692 | 95.29 |
|            16106584 | stillgray       |     296 | 11.52 |         101 |  3.93 |               2172 | 84.55 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         111 |  4.56 |               2316 | 95.11 |
|          3096147528 | dom_lucre       |     288 | 12.61 |          90 |  3.94 |               1905 | 83.44 |
|          3260357396 | thehealthb0t    |     376 | 17.25 |          35 |  1.61 |               1769 | 81.15 |
|            76766018 | Dexerto         |     228 | 10.65 |         178 |  8.32 |               1734 | 81.03 |
| 1366565625401909249 | WallStreetSilv  |     103 |  5.13 |          71 |  3.54 |               1834 | 91.33 |
| 1429400366819512323 | RadioGenoa      |      58 |  2.91 |         102 |  5.12 |               1833 | 91.97 |
| 1528943647185678336 | LeadingReport   |      56 |  2.82 |          37 |  1.86 |               1894 | 95.32 |
|  805532293951606785 | MattWallace888  |     230 | 11.71 |          87 |  4.43 |               1647 | 83.86 |
| 1374968033265864706 | TaraBull808     |     106 |  5.41 |          38 |  1.94 |               1814 | 92.65 |
| 1099579684981944320 | WallStreetApes  |      98 |  5.15 |          18 |  0.95 |               1787 | 93.90 |
|  801203059359678464 | DrEliDavid      |      41 |  2.17 |         153 |  8.11 |               1693 | 89.72 |
|            80233893 | jakeshieldsajj  |     100 |  5.32 |         103 |  5.48 |               1675 | 89.19 |
|  896550698543874049 | UTDTrey         |     192 | 10.37 |          72 |  3.89 |               1587 | 85.74 |
|           537709549 | LauraLoomer     |     210 | 12.04 |          46 |  2.64 |               1488 | 85.32 |
|          2538322138 | PicturesFoIder  |     119 |  6.95 |         182 | 10.63 |               1411 | 82.42 |
|  855481986290524160 | historyinmemes  |     147 |  8.73 |         176 | 10.46 |               1360 | 80.81 |
|           971820228 | CerfiaFR        |      75 |  4.63 |          96 |  5.92 |               1450 | 89.45 |
|           312696314 | Partisangirl    |     131 |  8.12 |          71 |  4.40 |               1411 | 87.48 |
| 1389913567671975937 | cb_doge         |      37 |  2.33 |          92 |  5.80 |               1458 | 91.87 |
| 1344356576786866176 | RepMTG          |      69 |  4.53 |          53 |  3.48 |               1401 | 91.99 |
|            32867753 | silvano_trotta  |     350 | 23.58 |          14 |  0.94 |               1120 | 75.47 |
|  890061634181373952 | CollinRugg      |      50 |  3.40 |          66 |  4.49 |               1354 | 92.11 |
|            25073877 | realDonaldTrump |       5 |  0.34 |         131 |  8.93 |               1331 | 90.73 |
| 1355721251180961792 | GuntherEagleman |      48 |  3.30 |          32 |  2.20 |               1374 | 94.50 |
|            49849732 | petrogustavo    |      33 |  2.31 |          38 |  2.66 |               1360 | 95.04 |
|           133663801 | BFMTV           |     136 |  9.54 |          86 |  6.03 |               1204 | 84.43 |
|            91416107 | OliLondonTV     |      64 |  4.51 |          80 |  5.64 |               1275 | 89.85 |
|  855483824351924224 | liz_churchill10 |     145 | 10.29 |          29 |  2.06 |               1235 | 87.65 |
|            15212187 | bennyjohnson    |      71 |  5.10 |          48 |  3.45 |               1273 | 91.45 |
| 1486069441259397125 | harris_wins     |      17 |  1.23 |          53 |  3.83 |               1313 | 94.94 |
|  750683331260321792 | MAstronomers    |      71 |  5.16 |          80 |  5.81 |               1225 | 89.03 |
|           592730371 | JackPosobiec    |      82 |  6.02 |          38 |  2.79 |               1241 | 91.18 |
|  807355480188141568 | DramaAlert      |     223 | 16.52 |          78 |  5.78 |               1049 | 77.70 |
|            17469289 | nypost          |     109 |  8.10 |          55 |  4.09 |               1182 | 87.82 |
|            14281853 | Conservatives   |      72 |  5.38 |          42 |  3.14 |               1224 | 91.48 |
|            52660746 | Israel          |       6 |  0.47 |         119 |  9.37 |               1145 | 90.16 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1138 | 89.96 |
|           177101260 | Rainmaker1973   |      22 |  1.75 |         135 | 10.75 |               1099 | 87.50 |
| 1455903807389458436 | BladeoftheS     |     166 | 13.25 |          20 |  1.60 |               1067 | 85.16 |
|           133938408 | krassenstein    |      14 |  1.12 |          80 |  6.41 |               1155 | 92.47 |
|            96684891 | kharaguchi      |     127 | 10.50 |           9 |  0.74 |               1074 | 88.76 |
|           452540168 | DailyLoud       |     304 | 25.40 |          75 |  6.27 |                818 | 68.34 |
|            37501849 | Quakeprediction |      32 |  2.72 |           1 |  0.09 |               1142 | 97.19 |
|             7587032 | SkyNews         |      33 |  2.89 |          46 |  4.03 |               1063 | 93.08 |
|            62957739 | eduardomenoni   |     262 | 23.29 |          17 |  1.51 |                846 | 75.20 |
| 1121807798826930177 | MyLordBebo      |     126 | 11.24 |          32 |  2.85 |                963 | 85.91 |
| 1335132884278108161 | stats_feed      |     110 |  9.84 |          58 |  5.19 |                950 | 84.97 |
|           428454304 | harryjsisson    |      14 |  1.26 |          35 |  3.14 |               1066 | 95.61 |
| 1224185690713460736 | goddeketal      |      56 |  5.02 |          30 |  2.69 |               1029 | 92.29 |
|           255471924 | mfa_russia      |     134 | 12.07 |          53 |  4.77 |                923 | 83.15 |
| 1486473049146904576 | InternetH0F     |     116 | 10.71 |         129 | 11.91 |                838 | 77.38 |
|           132339474 | EdKrassen       |      20 |  1.85 |          51 |  4.72 |               1009 | 93.43 |
|            51241574 | AP              |      17 |  1.59 |          48 |  4.50 |               1002 | 93.91 |
|            69156861 | TheChiefNerd    |       7 |  0.66 |          11 |  1.04 |               1043 | 98.30 |
|            16635277 | Acyn            |      34 |  3.22 |          69 |  6.53 |                954 | 90.26 |
| 1471414739880189955 | kirawontmiss    |      48 |  4.63 |         159 | 15.35 |                829 | 80.02 |
| 1529763962094596097 | wideawake_media |      17 |  1.64 |          14 |  1.35 |               1005 | 97.01 |
|          2670726740 | LulaOficial     |       5 |  0.49 |          25 |  2.45 |                992 | 97.06 |
| 1043185714437992449 | catturd2        |      43 |  4.26 |          46 |  4.56 |                920 | 91.18 |
|           524869533 | QudsNen         |      58 |  5.84 |          31 |  3.12 |                905 | 91.05 |
|            90954365 | earthquakejapan |      87 |  8.78 |           0 |  0.00 |                904 | 91.22 |
| 1138458175663988738 | PopBase         |      52 |  5.25 |          74 |  7.47 |                864 | 87.27 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.39 |          23 |  2.34 |                907 | 92.27 |
|           848279118 | Kahlissee       |      37 |  3.78 |          40 |  4.09 |                901 | 92.13 |
|           242827267 | PierrePoilievre |       0 |  0.00 |          28 |  2.87 |                946 | 97.13 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.39 |          35 |  3.59 |                906 | 93.02 |
|           138203134 | AOC             |      12 |  1.24 |          50 |  5.15 |                909 | 93.61 |
|           292929271 | charliekirk11   |      42 |  4.37 |          28 |  2.91 |                891 | 92.72 |
|             7996082 | el_pais         |      40 |  4.19 |          29 |  3.04 |                886 | 92.77 |
|            22703645 | TuckerCarlson   |       1 |  0.10 |          89 |  9.34 |                863 | 90.56 |
|           706646642 | EmbassyofRussia |     106 | 11.19 |          33 |  3.48 |                808 | 85.32 |
|          2828212668 | AMAZlNGNATURE   |      67 |  7.07 |         117 | 12.35 |                763 | 80.57 |
| 1538230472299270144 | ayeejuju        |      78 |  8.26 |         101 | 10.70 |                765 | 81.04 |
| 1342174584594960384 | iamyesyouareno  |      52 |  5.54 |          57 |  6.07 |                830 | 88.39 |
|            37491797 | stkirsch        |      62 |  6.68 |          12 |  1.29 |                854 | 92.03 |
| 1356434353623093249 | greg16676935420 |      19 |  2.05 |         101 | 10.88 |                808 | 87.07 |
|           374712154 | TRobinsonNewEra |      38 |  4.11 |          22 |  2.38 |                865 | 93.51 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|           130557513 | mehdirhasan     |      14 |  1.52 |          29 |  3.15 |                878 | 95.33 |
|            14594813 | folha           |      39 |  4.25 |          25 |  2.72 |                854 | 93.03 |
|          3331429624 | Metropoles      |      75 |  8.21 |          22 |  2.41 |                817 | 89.39 |
|           333357345 | Cobratate       |      39 |  4.28 |          85 |  9.32 |                788 | 86.40 |
| 1288319695658135552 | realstewpeters  |     184 | 20.38 |          35 |  3.88 |                684 | 75.75 |
| 1430497892314218502 | Resist_05       |      44 |  4.90 |          24 |  2.67 |                830 | 92.43 |
|  855300206086168576 | HumansNoContext |      92 | 10.37 |          76 |  8.57 |                719 | 81.06 |
|  875856268056969216 | DC_Draino       |      16 |  1.80 |          19 |  2.14 |                852 | 96.05 |
|            14128609 | felipeneto      |      18 |  2.05 |          31 |  3.53 |                830 | 94.43 |
|          1542228578 | JDVance         |       6 |  0.69 |          60 |  6.88 |                806 | 92.43 |
| 1187524450809536513 | vitoquiles      |      40 |  4.59 |          25 |  2.87 |                807 | 92.55 |
|           978932870 | CapitanBitcoin  |      37 |  4.33 |          21 |  2.46 |                797 | 93.22 |
|  780460754910732288 | DiscussingFilm  |      33 |  3.86 |          91 | 10.64 |                731 | 85.50 |
|            80820758 | JLMelenchon     |      51 |  5.98 |          57 |  6.68 |                745 | 87.34 |
| 1087757588622651397 | porqueTTarg     |     182 | 21.34 |          28 |  3.28 |                643 | 75.38 |
| 1128337957289697281 | SprinterFamily  |     210 | 24.62 |          21 |  2.46 |                622 | 72.92 |
|            11347122 | GavinNewsom     |      10 |  1.18 |          22 |  2.59 |                818 | 96.24 |
| 1600964443122421769 | CNviolations    |      12 |  1.41 |          90 | 10.60 |                747 | 87.99 |
|          1446465174 | akafacehots     |      85 | 10.08 |          25 |  2.97 |                733 | 86.95 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
| 1562038858988064768 | TheFigen\_      |     100 | 11.90 |          78 |  9.29 |                662 | 78.81 |
| 1225234593789423616 | Megatron_ron    |     111 | 13.25 |          41 |  4.89 |                686 | 81.86 |
| 1090084079964348419 | MrSinha\_       |      21 |  2.52 |           8 |  0.96 |                803 | 96.51 |
|           109398997 | stopvaccinating |      92 | 11.10 |          10 |  1.21 |                727 | 87.70 |
| 1080188052365029376 | acnewsitics     |       9 |  1.09 |          53 |  6.42 |                764 | 92.49 |
| 1593929531148144645 | stairwayto3dom  |      57 |  6.94 |          30 |  3.65 |                734 | 89.40 |
|           294293982 | Rothmus         |      49 |  5.98 |          43 |  5.24 |                728 | 88.78 |
|          1640929196 | mmpadellan      |      25 |  3.05 |          15 |  1.83 |                780 | 95.12 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          14 |  1.72 |                800 | 98.28 |
| 1492007194388279333 | LibertyCappy    |      30 |  3.70 |          57 |  7.04 |                723 | 89.26 |
|           463142998 | SuppressedNws   |      14 |  1.75 |          35 |  4.39 |                749 | 93.86 |
| 1302674142630760450 | EverythingOOC   |      67 |  8.41 |          87 | 10.92 |                643 | 80.68 |
|          4429003533 | PopCrave        |      43 |  5.43 |          64 |  8.08 |                685 | 86.49 |
|             4239551 | amuse           |      24 |  3.04 |          22 |  2.78 |                744 | 94.18 |
|  707231479047315456 | PeterSweden7    |      22 |  2.78 |          12 |  1.52 |                756 | 95.70 |
|            46302096 | JoeyMannarinoUS |      53 |  6.72 |          24 |  3.04 |                712 | 90.24 |
|  959531564341317632 | AlertesInfos    |      34 |  4.33 |          52 |  6.62 |                699 | 89.04 |
|           319774010 | kirinjisinken   |     425 | 54.21 |          11 |  1.40 |                348 | 44.39 |
| 1111976778065723393 | nocontextfooty  |      91 | 11.76 |          55 |  7.11 |                628 | 81.14 |
|           535707261 | eldiarioes      |      12 |  1.57 |          14 |  1.83 |                740 | 96.61 |
|          4020276615 | JMilei          |       4 |  0.52 |          47 |  6.16 |                712 | 93.32 |
|           330262748 | FabrizioRomano  |      60 |  7.89 |          93 | 12.24 |                607 | 79.87 |
|  826065164504006657 | mtgreenee       |      33 |  4.34 |          35 |  4.61 |                692 | 91.05 |
|  795188519115358208 | tweetsoku1      |     236 | 31.13 |           6 |  0.79 |                516 | 68.07 |
| 1625843518643085312 | creepydotorg    |      55 |  7.38 |          58 |  7.79 |                632 | 84.83 |
|            27000730 | Timcast         |      15 |  2.02 |          36 |  4.84 |                693 | 93.15 |
| 1179892477714718721 | gunsnrosesgirl3 |      24 |  3.23 |          84 | 11.29 |                636 | 85.48 |
|  918197046871502849 | siteptbr        |      87 | 11.71 |          20 |  2.69 |                636 | 85.60 |
| 1138842105856573445 | NoContextHumans |      67 |  9.10 |          84 | 11.41 |                585 | 79.48 |
| 1393726565809278976 | AdameMedia      |      18 |  2.46 |          33 |  4.51 |                680 | 93.02 |
|  818893114979061761 | JoJoFromJerz    |      11 |  1.51 |          22 |  3.02 |                695 | 95.47 |
|           288277167 | atrupar         |      16 |  2.20 |          26 |  3.58 |                685 | 94.22 |
|            14260960 | JustinTrudeau   |       1 |  0.14 |          52 |  7.16 |                673 | 92.70 |
|            18576537 | IDF             |       0 |  0.00 |          72 |  9.92 |                654 | 90.08 |
|          1626294277 | spectatorindex  |      10 |  1.38 |          47 |  6.49 |                667 | 92.13 |
|            19017675 | Nigel_Farage    |       5 |  0.69 |          28 |  3.89 |                687 | 95.42 |
|            39344374 | DonaldJTrumpJr  |      29 |  4.04 |          38 |  5.29 |                651 | 90.67 |
|            14436030 | elmundoes       |      60 |  8.37 |          12 |  1.67 |                645 | 89.96 |
|          1500129642 | MattWalshBlog   |      11 |  1.53 |          28 |  3.91 |                678 | 94.56 |
| 1062754443798532096 | MakisMD         |      20 |  2.80 |          10 |  1.40 |                685 | 95.80 |
|            19069018 | jreichelt       |      20 |  2.80 |          21 |  2.94 |                673 | 94.26 |
| 1221462414744596483 | RpsAgainstTrump |       8 |  1.12 |          69 |  9.68 |                636 | 89.20 |
| 1432287556129812484 | himuro398       |     140 | 19.69 |          12 |  1.69 |                559 | 78.62 |
| 1661674273122160641 | \_maakun\_\_2   |     139 | 19.58 |           0 |  0.00 |                571 | 80.42 |
| 1227799690579718144 | VivekGRamaswamy |      15 |  2.12 |          33 |  4.67 |                659 | 93.21 |
| 1434450096557596680 | PolitlcsUK      |      18 |  2.55 |          42 |  5.94 |                647 | 91.51 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.42 |          27 |  3.83 |                668 | 94.75 |
|           423692278 | AkademiksTV     |      36 |  5.15 |          10 |  1.43 |                653 | 93.42 |
|           472412809 | f_philippot     |     112 | 16.07 |          14 |  2.01 |                571 | 81.92 |
| 1604139215406727170 | CensoredMen     |      34 |  4.89 |          46 |  6.62 |                615 | 88.49 |
|             1137701 | DavidSacks      |      21 |  3.03 |          55 |  7.93 |                618 | 89.05 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      55 |  7.98 |           7 |  1.02 |                627 | 91.00 |
|          2161051908 | AvivaKlompas    |      13 |  1.90 |          61 |  8.89 |                612 | 89.21 |
|           109065990 | RealAlexJones   |      46 |  6.76 |          18 |  2.65 |                616 | 90.59 |
| 1200616796295847936 | unusual_whales  |      88 | 13.00 |          27 |  3.99 |                562 | 83.01 |
| 1392864463204782082 | WarMonitors     |      20 |  2.95 |          31 |  4.58 |                626 | 92.47 |
|           337808606 | RobertKennedyJr |      11 |  1.64 |          33 |  4.91 |                628 | 93.45 |
|           351491321 | wallstwolverine |      18 |  2.69 |          31 |  4.64 |                619 | 92.66 |
| 1298372735383605249 | RonFilipkowski  |      18 |  2.70 |          33 |  4.95 |                616 | 92.35 |
|           195853497 | EuropeInvasions |      50 |  7.53 |          39 |  5.87 |                575 | 86.60 |
| 1158115510606815232 | therealbuni     |      76 | 11.50 |          25 |  3.78 |                560 | 84.72 |
|              759251 | CNN             |      16 |  2.43 |          19 |  2.88 |                624 | 94.69 |
|            19923515 | abc_es          |      70 | 10.64 |          19 |  2.89 |                569 | 86.47 |
|          4691437897 | darrengrimes\_  |      34 |  5.18 |          15 |  2.28 |                608 | 92.54 |
|            94324983 | FonsiLoaiza     |      13 |  2.00 |          19 |  2.92 |                618 | 95.08 |
|              742143 | BBCWorld        |      23 |  3.54 |          34 |  5.24 |                592 | 91.22 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.16 |                635 | 97.84 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
| 1450241520859156483 | geoscience16    |     175 | 27.13 |           4 |  0.62 |                466 | 72.25 |
| 1514045582968598530 | CFC_Janty       |      50 |  7.75 |          27 |  4.19 |                568 | 88.06 |
| 1157689921349521410 | esjesjesj       |      15 |  2.33 |          47 |  7.30 |                582 | 90.37 |
|            62513246 | jk_rowling      |       0 |  0.00 |          59 |  9.19 |                583 | 90.81 |
|            39692424 | AlertaNews24    |     115 | 18.03 |          20 |  3.13 |                503 | 78.84 |
| 1316995857242378240 | realMaalouf     |      42 |  6.64 |          47 |  7.42 |                544 | 85.94 |
| 1155845777039810560 | richimedhurst   |      17 |  2.72 |          23 |  3.67 |                586 | 93.61 |
|            55329156 | RNCResearch     |      31 |  5.06 |          19 |  3.10 |                563 | 91.84 |
|  930212715422998530 | Travis_4_Trump  |      28 |  4.58 |          16 |  2.61 |                568 | 92.81 |
|           142393421 | GloboNews       |      11 |  1.80 |           9 |  1.47 |                591 | 96.73 |
|            14173315 | NBCNews         |      11 |  1.81 |          17 |  2.80 |                580 | 95.39 |
|            81371986 | LozzaFox        |      31 |  5.10 |          22 |  3.62 |                555 | 91.28 |
|            16906660 | Grummz          |       5 |  0.82 |          23 |  3.79 |                579 | 95.39 |
|             5734902 | tagesschau      |      15 |  2.48 |          23 |  3.80 |                567 | 93.72 |
|            14377605 | TheDemocrats    |       2 |  0.33 |          23 |  3.81 |                578 | 95.85 |
| 1492677599390322689 | weirddalle      |      18 |  2.99 |          76 | 12.60 |                509 | 84.41 |
| 1339166129110065152 | GBNEWS          |      25 |  4.16 |          10 |  1.66 |                566 | 94.18 |
| 1151126931439407107 | RimaHas         |      16 |  2.68 |          48 |  8.03 |                534 | 89.30 |
