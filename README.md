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
|            44196397 | elonmusk        |     120 |  0.63 |        2109 | 11.09 |              16791 | 88.28 |
|           629698642 | BGatesIsaPyscho |    1132 | 22.68 |         125 |  2.50 |               3734 | 74.81 |
| 1151913018936053760 | jacksonhinklle  |     363 |  8.21 |         227 |  5.13 |               3831 | 86.65 |
|          3376321847 | iluminatibot    |     777 | 18.39 |         103 |  2.44 |               3344 | 79.17 |
| 1349149096909668363 | POTUS           |      60 |  1.51 |         253 |  6.36 |               3664 | 92.13 |
| 1552795969959636992 | EndWokeness     |      71 |  1.90 |         156 |  4.18 |               3503 | 93.91 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         103 |  2.85 |               3506 | 97.12 |
|           191871143 | VigilantFox     |      35 |  1.00 |          49 |  1.40 |               3418 | 97.60 |
|              939091 | JoeBiden        |      16 |  0.46 |         303 |  8.68 |               3172 | 90.86 |
|          1291128494 | ShaykhSulaiman  |     262 |  7.69 |         159 |  4.67 |               2985 | 87.64 |
| 1222773302441148416 | visegrad24      |     165 |  5.12 |         282 |  8.76 |               2774 | 86.12 |
| 1319287761048723458 | MarioNawfal     |     193 |  6.23 |          95 |  3.07 |               2810 | 90.70 |
| 1326229737551912960 | libsoftiktok    |      46 |  1.52 |         104 |  3.43 |               2886 | 95.06 |
|  953378142198161409 | choquei         |     446 | 14.85 |          71 |  2.36 |               2487 | 82.79 |
| 1446231057259433988 | DrLoupis        |     216 |  7.22 |         125 |  4.18 |               2650 | 88.60 |
|            16106584 | stillgray       |     300 | 11.46 |         103 |  3.93 |               2215 | 84.61 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         111 |  4.56 |               2317 | 95.11 |
|          3260357396 | thehealthb0t    |     411 | 17.59 |          36 |  1.54 |               1890 | 80.87 |
|          3096147528 | dom_lucre       |     291 | 12.66 |          91 |  3.96 |               1917 | 83.38 |
|            76766018 | Dexerto         |     233 | 10.60 |         181 |  8.23 |               1784 | 81.16 |
|  896550698543874049 | UTDTrey         |     218 | 10.38 |          74 |  3.52 |               1808 | 86.10 |
| 1429400366819512323 | RadioGenoa      |      57 |  2.75 |         107 |  5.17 |               1905 | 92.07 |
| 1366565625401909249 | WallStreetSilv  |     105 |  5.13 |          74 |  3.62 |               1866 | 91.25 |
| 1528943647185678336 | LeadingReport   |      56 |  2.74 |          37 |  1.81 |               1950 | 95.45 |
| 1099579684981944320 | WallStreetApes  |     105 |  5.19 |          21 |  1.04 |               1899 | 93.78 |
| 1374968033265864706 | TaraBull808     |     110 |  5.46 |          42 |  2.09 |               1862 | 92.45 |
|  805532293951606785 | MattWallace888  |     236 | 11.74 |          89 |  4.43 |               1685 | 83.83 |
|  801203059359678464 | DrEliDavid      |      42 |  2.17 |         155 |  8.02 |               1735 | 89.80 |
|            80233893 | jakeshieldsajj  |     101 |  5.37 |         103 |  5.47 |               1678 | 89.16 |
|          2538322138 | PicturesFoIder  |     122 |  6.96 |         183 | 10.45 |               1447 | 82.59 |
|           537709549 | LauraLoomer     |     210 | 12.01 |          46 |  2.63 |               1492 | 85.35 |
|  855481986290524160 | historyinmemes  |     148 |  8.70 |         178 | 10.46 |               1375 | 80.83 |
|           971820228 | CerfiaFR        |      78 |  4.65 |         102 |  6.08 |               1497 | 89.27 |
|           312696314 | Partisangirl    |     131 |  8.05 |          72 |  4.42 |               1425 | 87.53 |
|            32867753 | silvano_trotta  |     368 | 22.73 |          15 |  0.93 |               1236 | 76.34 |
| 1389913567671975937 | cb_doge         |      37 |  2.29 |          95 |  5.87 |               1487 | 91.85 |
|            49849732 | petrogustavo    |      39 |  2.45 |          44 |  2.76 |               1511 | 94.79 |
| 1344356576786866176 | RepMTG          |      71 |  4.58 |          55 |  3.55 |               1424 | 91.87 |
|  890061634181373952 | CollinRugg      |      50 |  3.30 |          67 |  4.42 |               1400 | 92.29 |
|            25073877 | realDonaldTrump |       5 |  0.33 |         135 |  8.91 |               1375 | 90.76 |
| 1355721251180961792 | GuntherEagleman |      49 |  3.28 |          33 |  2.21 |               1412 | 94.51 |
|           133663801 | BFMTV           |     141 |  9.48 |          90 |  6.05 |               1257 | 84.48 |
|            15212187 | bennyjohnson    |      77 |  5.21 |          53 |  3.58 |               1349 | 91.21 |
|  855483824351924224 | liz_churchill10 |     150 | 10.20 |          29 |  1.97 |               1291 | 87.82 |
|            91416107 | OliLondonTV     |      64 |  4.48 |          80 |  5.60 |               1285 | 89.92 |
| 1486069441259397125 | harris_wins     |      18 |  1.27 |          53 |  3.75 |               1341 | 94.97 |
|  807355480188141568 | DramaAlert      |     231 | 16.46 |          81 |  5.77 |               1091 | 77.76 |
|           592730371 | JackPosobiec    |      83 |  5.92 |          38 |  2.71 |               1281 | 91.37 |
|            17469289 | nypost          |     110 |  7.85 |          58 |  4.14 |               1233 | 88.01 |
|  750683331260321792 | MAstronomers    |      71 |  5.09 |          80 |  5.73 |               1244 | 89.18 |
|            14281853 | Conservatives   |      72 |  5.36 |          42 |  3.13 |               1229 | 91.51 |
|           133938408 | krassenstein    |      14 |  1.05 |          88 |  6.59 |               1234 | 92.37 |
| 1455903807389458436 | BladeoftheS     |     166 | 12.55 |          23 |  1.74 |               1134 | 85.71 |
|           177101260 | Rainmaker1973   |      23 |  1.75 |         142 | 10.83 |               1146 | 87.41 |
|            52660746 | Israel          |       6 |  0.47 |         120 |  9.40 |               1150 | 90.13 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1138 | 89.96 |
|           428454304 | harryjsisson    |      16 |  1.29 |          38 |  3.07 |               1184 | 95.64 |
|            96684891 | kharaguchi      |     127 | 10.27 |           9 |  0.73 |               1101 | 89.01 |
|           452540168 | DailyLoud       |     309 | 25.14 |          76 |  6.18 |                844 | 68.67 |
|            37501849 | Quakeprediction |      40 |  3.34 |           2 |  0.17 |               1155 | 96.49 |
|            62957739 | eduardomenoni   |     285 | 23.91 |          19 |  1.59 |                888 | 74.50 |
|             7587032 | SkyNews         |      34 |  2.93 |          46 |  3.96 |               1082 | 93.12 |
|            16635277 | Acyn            |      39 |  3.38 |          79 |  6.85 |               1036 | 89.77 |
| 1121807798826930177 | MyLordBebo      |     129 | 11.19 |          32 |  2.78 |                992 | 86.04 |
| 1335132884278108161 | stats_feed      |     109 |  9.53 |          59 |  5.16 |                976 | 85.31 |
| 1486473049146904576 | InternetH0F     |     120 | 10.55 |         135 | 11.87 |                882 | 77.57 |
| 1224185690713460736 | goddeketal      |      58 |  5.11 |          31 |  2.73 |               1046 | 92.16 |
|           255471924 | mfa_russia      |     137 | 12.22 |          53 |  4.73 |                931 | 83.05 |
|            69156861 | TheChiefNerd    |       8 |  0.72 |          13 |  1.16 |               1096 | 98.12 |
|           132339474 | EdKrassen       |      22 |  1.97 |          53 |  4.74 |               1042 | 93.29 |
|            14594813 | folha           |      41 |  3.68 |          27 |  2.42 |               1047 | 93.90 |
|            51241574 | AP              |      19 |  1.71 |          50 |  4.51 |               1039 | 93.77 |
|          2670726740 | LulaOficial     |       5 |  0.45 |          25 |  2.27 |               1071 | 97.28 |
| 1471414739880189955 | kirawontmiss    |      49 |  4.57 |         161 | 15.00 |                863 | 80.43 |
| 1529763962094596097 | wideawake_media |      18 |  1.68 |          14 |  1.30 |               1041 | 97.02 |
| 1138458175663988738 | PopBase         |      52 |  4.87 |          80 |  7.50 |                935 | 87.63 |
|           292929271 | charliekirk11   |      42 |  3.94 |          35 |  3.28 |                989 | 92.78 |
| 1043185714437992449 | catturd2        |      45 |  4.35 |          47 |  4.54 |                943 | 91.11 |
|           138203134 | AOC             |      12 |  1.18 |          52 |  5.13 |                949 | 93.68 |
|           242827267 | PierrePoilievre |       1 |  0.10 |          30 |  2.96 |                981 | 96.94 |
|           333357345 | Cobratate       |      49 |  4.86 |          89 |  8.83 |                870 | 86.31 |
|          3331429624 | Metropoles      |      82 |  8.21 |          25 |  2.50 |                892 | 89.29 |
|           524869533 | QudsNen         |      58 |  5.81 |          31 |  3.11 |                909 | 91.08 |
|             7996082 | el_pais         |      40 |  4.02 |          30 |  3.02 |                924 | 92.96 |
|           848279118 | Kahlissee       |      37 |  3.72 |          40 |  4.02 |                917 | 92.25 |
|            90954365 | earthquakejapan |      87 |  8.76 |           0 |  0.00 |                906 | 91.24 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.36 |          23 |  2.33 |                912 | 92.31 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.35 |          35 |  3.56 |                916 | 93.09 |
|          2828212668 | AMAZlNGNATURE   |      71 |  7.22 |         123 | 12.51 |                789 | 80.26 |
| 1538230472299270144 | ayeejuju        |      80 |  8.29 |         104 | 10.78 |                781 | 80.93 |
| 1356434353623093249 | greg16676935420 |      20 |  2.08 |         103 | 10.71 |                839 | 87.21 |
|            22703645 | TuckerCarlson   |       1 |  0.10 |          89 |  9.32 |                865 | 90.58 |
| 1342174584594960384 | iamyesyouareno  |      52 |  5.47 |          58 |  6.10 |                841 | 88.43 |
|           130557513 | mehdirhasan     |      14 |  1.48 |          30 |  3.17 |                903 | 95.35 |
|           706646642 | EmbassyofRussia |     106 | 11.19 |          33 |  3.48 |                808 | 85.32 |
|  855300206086168576 | HumansNoContext |      98 | 10.37 |          81 |  8.57 |                766 | 81.06 |
|           374712154 | TRobinsonNewEra |      38 |  4.03 |          22 |  2.33 |                883 | 93.64 |
|            37491797 | stkirsch        |      62 |  6.68 |          12 |  1.29 |                854 | 92.03 |
| 1600964443122421769 | CNviolations    |      12 |  1.29 |          97 | 10.46 |                818 | 88.24 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|            11347122 | GavinNewsom     |      10 |  1.09 |          23 |  2.50 |                886 | 96.41 |
|  875856268056969216 | DC_Draino       |      17 |  1.86 |          21 |  2.30 |                875 | 95.84 |
| 1288319695658135552 | realstewpeters  |     186 | 20.37 |          35 |  3.83 |                692 | 75.79 |
| 1430497892314218502 | Resist_05       |      44 |  4.85 |          24 |  2.65 |                839 | 92.50 |
|  780460754910732288 | DiscussingFilm  |      34 |  3.77 |          96 | 10.65 |                771 | 85.57 |
| 1087757588622651397 | porqueTTarg     |     192 | 21.45 |          29 |  3.24 |                674 | 75.31 |
|          1542228578 | JDVance         |       6 |  0.67 |          61 |  6.83 |                826 | 92.50 |
| 1187524450809536513 | vitoquiles      |      40 |  4.51 |          26 |  2.93 |                820 | 92.55 |
|            14128609 | felipeneto      |      18 |  2.04 |          31 |  3.51 |                834 | 94.45 |
|           423692278 | AkademiksTV     |      41 |  4.67 |          13 |  1.48 |                824 | 93.85 |
|            80820758 | JLMelenchon     |      53 |  6.04 |          58 |  6.61 |                766 | 87.34 |
| 1128337957289697281 | SprinterFamily  |     216 | 24.63 |          21 |  2.39 |                640 | 72.98 |
| 1090084079964348419 | MrSinha\_       |      22 |  2.52 |           8 |  0.92 |                843 | 96.56 |
|           978932870 | CapitanBitcoin  |      37 |  4.27 |          21 |  2.42 |                809 | 93.31 |
| 1562038858988064768 | TheFigen\_      |     105 | 12.17 |          83 |  9.62 |                675 | 78.22 |
| 1225234593789423616 | Megatron_ron    |     112 | 13.11 |          42 |  4.92 |                700 | 81.97 |
|          1446465174 | akafacehots     |      85 |  9.99 |          24 |  2.82 |                742 | 87.19 |
| 1080188052365029376 | acnewsitics     |       9 |  1.06 |          55 |  6.50 |                782 | 92.43 |
|             4239551 | amuse           |      28 |  3.31 |          24 |  2.84 |                793 | 93.85 |
|           294293982 | Rothmus         |      51 |  6.06 |          43 |  5.11 |                747 | 88.82 |
|           463142998 | SuppressedNws   |      14 |  1.66 |          35 |  4.16 |                792 | 94.17 |
|          1640929196 | mmpadellan      |      26 |  3.09 |          17 |  2.02 |                798 | 94.89 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
|  707231479047315456 | PeterSweden7    |      22 |  2.63 |          11 |  1.32 |                803 | 96.05 |
|          4429003533 | PopCrave        |      44 |  5.27 |          68 |  8.14 |                723 | 86.59 |
|           109398997 | stopvaccinating |      92 | 11.04 |          10 |  1.20 |                731 | 87.76 |
|  918197046871502849 | siteptbr        |      94 | 11.31 |          22 |  2.65 |                715 | 86.04 |
|  959531564341317632 | AlertesInfos    |      36 |  4.34 |          54 |  6.51 |                740 | 89.16 |
|  795188519115358208 | tweetsoku1      |     257 | 31.00 |           6 |  0.72 |                566 | 68.28 |
| 1111976778065723393 | nocontextfooty  |      98 | 11.84 |          58 |  7.00 |                672 | 81.16 |
| 1593929531148144645 | stairwayto3dom  |      57 |  6.88 |          31 |  3.74 |                740 | 89.37 |
| 1492007194388279333 | LibertyCappy    |      30 |  3.64 |          58 |  7.03 |                737 | 89.33 |
|           142393421 | GloboNews       |      20 |  2.44 |          16 |  1.95 |                784 | 95.61 |
|           319774010 | kirinjisinken   |     440 | 53.79 |          11 |  1.34 |                367 | 44.87 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          14 |  1.72 |                801 | 98.28 |
| 1302674142630760450 | EverythingOOC   |      68 |  8.38 |          87 | 10.73 |                656 | 80.89 |
| 1393726565809278976 | AdameMedia      |      20 |  2.50 |          34 |  4.26 |                745 | 93.24 |
|            46302096 | JoeyMannarinoUS |      56 |  7.02 |          24 |  3.01 |                718 | 89.97 |
|           535707261 | eldiarioes      |      12 |  1.51 |          15 |  1.88 |                769 | 96.61 |
|           288277167 | atrupar         |      19 |  2.39 |          34 |  4.28 |                741 | 93.32 |
|  818893114979061761 | JoJoFromJerz    |      12 |  1.51 |          27 |  3.40 |                754 | 95.08 |
|           330262748 | FabrizioRomano  |      65 |  8.21 |          96 | 12.12 |                631 | 79.67 |
| 1179892477714718721 | gunsnrosesgirl3 |      24 |  3.05 |          91 | 11.56 |                672 | 85.39 |
|          4020276615 | JMilei          |       4 |  0.51 |          52 |  6.65 |                726 | 92.84 |
|  826065164504006657 | mtgreenee       |      36 |  4.65 |          37 |  4.78 |                701 | 90.57 |
|            19017675 | Nigel_Farage    |       6 |  0.78 |          29 |  3.79 |                730 | 95.42 |
| 1432287556129812484 | himuro398       |     153 | 20.00 |          11 |  1.44 |                601 | 78.56 |
| 1625843518643085312 | creepydotorg    |      56 |  7.33 |          61 |  7.98 |                647 | 84.69 |
|            14260960 | JustinTrudeau   |       1 |  0.13 |          54 |  7.09 |                707 | 92.78 |
| 1221462414744596483 | RpsAgainstTrump |       9 |  1.18 |          76 |  9.97 |                677 | 88.85 |
|           109065990 | RealAlexJones   |      53 |  6.97 |          23 |  3.03 |                684 | 90.00 |
|            27000730 | Timcast         |      15 |  1.98 |          35 |  4.63 |                706 | 93.39 |
| 1434450096557596680 | PolitlcsUK      |      19 |  2.52 |          45 |  5.97 |                690 | 91.51 |
|            39344374 | DonaldJTrumpJr  |      29 |  3.86 |          41 |  5.46 |                681 | 90.68 |
|          1626294277 | spectatorindex  |      11 |  1.47 |          47 |  6.29 |                689 | 92.24 |
| 1138842105856573445 | NoContextHumans |      67 |  8.99 |          83 | 11.14 |                595 | 79.87 |
|          1500129642 | MattWalshBlog   |      11 |  1.48 |          29 |  3.90 |                703 | 94.62 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.35 |          30 |  4.06 |                699 | 94.59 |
|            14436030 | elmundoes       |      61 |  8.27 |          12 |  1.63 |                665 | 90.11 |
|            19069018 | jreichelt       |      20 |  2.72 |          21 |  2.86 |                694 | 94.42 |
| 1661674273122160641 | \_maakun\_\_2   |     139 | 18.91 |           0 |  0.00 |                596 | 81.09 |
| 1062754443798532096 | MakisMD         |      20 |  2.74 |          10 |  1.37 |                699 | 95.88 |
|            18576537 | IDF             |       0 |  0.00 |          72 |  9.90 |                655 | 90.10 |
|            39692424 | AlertaNews24    |     122 | 17.02 |          21 |  2.93 |                574 | 80.06 |
| 1200616796295847936 | unusual_whales  |      91 | 12.80 |          30 |  4.22 |                590 | 82.98 |
| 1227799690579718144 | VivekGRamaswamy |      15 |  2.11 |          33 |  4.64 |                663 | 93.25 |
|           472412809 | f_philippot     |     112 | 15.86 |          16 |  2.27 |                578 | 81.87 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      55 |  7.86 |           7 |  1.00 |                638 | 91.14 |
| 1514045582968598530 | CFC_Janty       |      56 |  8.03 |          27 |  3.87 |                614 | 88.09 |
| 1157689921349521410 | esjesjesj       |      15 |  2.16 |          49 |  7.04 |                632 | 90.80 |
| 1604139215406727170 | CensoredMen     |      34 |  4.89 |          46 |  6.62 |                615 | 88.49 |
|             1137701 | DavidSacks      |      21 |  3.03 |          55 |  7.93 |                618 | 89.05 |
|          2161051908 | AvivaKlompas    |      13 |  1.88 |          61 |  8.82 |                618 | 89.31 |
|          4691437897 | darrengrimes\_  |      36 |  5.20 |          17 |  2.46 |                639 | 92.34 |
|              742143 | BBCWorld        |      28 |  4.08 |          35 |  5.10 |                623 | 90.82 |
|           195853497 | EuropeInvasions |      50 |  7.29 |          39 |  5.69 |                597 | 87.03 |
| 1298372735383605249 | RonFilipkowski  |      18 |  2.63 |          33 |  4.82 |                634 | 92.55 |
| 1392864463204782082 | WarMonitors     |      20 |  2.92 |          32 |  4.67 |                633 | 92.41 |
|           351491321 | wallstwolverine |      19 |  2.78 |          32 |  4.68 |                633 | 92.54 |
| 1158115510606815232 | therealbuni     |      80 | 11.70 |          27 |  3.95 |                577 | 84.36 |
|            94324983 | FonsiLoaiza     |      13 |  1.92 |          21 |  3.10 |                644 | 94.99 |
|            19923515 | abc_es          |      71 | 10.49 |          19 |  2.81 |                587 | 86.71 |
| 1316995857242378240 | realMaalouf     |      41 |  6.08 |          49 |  7.27 |                584 | 86.65 |
|           337808606 | RobertKennedyJr |      11 |  1.63 |          33 |  4.90 |                629 | 93.46 |
| 1450241520859156483 | geoscience16    |     180 | 26.79 |           4 |  0.60 |                488 | 72.62 |
|              759251 | CNN             |      16 |  2.41 |          19 |  2.87 |                628 | 94.72 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.12 |                646 | 97.88 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
|            62513246 | jk_rowling      |       0 |  0.00 |          59 |  9.13 |                587 | 90.87 |
|           112103858 | ALeaument       |      24 |  3.74 |          30 |  4.68 |                587 | 91.58 |
| 1339166129110065152 | GBNEWS          |      27 |  4.23 |          10 |  1.57 |                601 | 94.20 |
|            14377605 | TheDemocrats    |       3 |  0.47 |          23 |  3.62 |                609 | 95.91 |
|            16906660 | Grummz          |       5 |  0.79 |          23 |  3.64 |                604 | 95.57 |
|             5734902 | tagesschau      |      15 |  2.38 |          24 |  3.80 |                592 | 93.82 |
| 1492677599390322689 | weirddalle      |      20 |  3.17 |          79 | 12.54 |                531 | 84.29 |
| 1151126931439407107 | RimaHas         |      16 |  2.55 |          52 |  8.29 |                559 | 89.15 |
| 1155845777039810560 | richimedhurst   |      17 |  2.71 |          23 |  3.67 |                587 | 93.62 |
|            55329156 | RNCResearch     |      32 |  5.19 |          19 |  3.08 |                565 | 91.72 |
|            81371986 | LozzaFox        |      32 |  5.20 |          22 |  3.58 |                561 | 91.22 |
| 1587130893616955393 | cagiago\_       |      52 |  8.46 |          24 |  3.90 |                539 | 87.64 |
