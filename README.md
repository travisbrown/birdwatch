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
|            44196397 | elonmusk        |     108 |  0.67 |        1850 | 11.41 |              14256 | 87.92 |
|           629698642 | BGatesIsaPyscho |     963 | 22.22 |         101 |  2.33 |               3270 | 75.45 |
| 1151913018936053760 | jacksonhinklle  |     350 |  8.32 |         210 |  4.99 |               3649 | 86.70 |
| 1349149096909668363 | POTUS           |      57 |  1.50 |         239 |  6.28 |               3508 | 92.22 |
|          3376321847 | iluminatibot    |     658 | 17.79 |          88 |  2.38 |               2952 | 79.83 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         103 |  2.85 |               3505 | 97.12 |
|              939091 | JoeBiden        |      15 |  0.44 |         301 |  8.77 |               3116 | 90.79 |
| 1552795969959636992 | EndWokeness     |      67 |  2.02 |         139 |  4.19 |               3110 | 93.79 |
|           191871143 | VigilantFox     |      32 |  0.97 |          45 |  1.37 |               3218 | 97.66 |
|          1291128494 | ShaykhSulaiman  |     257 |  7.91 |         155 |  4.77 |               2835 | 87.31 |
| 1446231057259433988 | DrLoupis        |     210 |  7.35 |         117 |  4.10 |               2530 | 88.55 |
| 1222773302441148416 | visegrad24      |     156 |  5.52 |         264 |  9.35 |               2404 | 85.13 |
|  953378142198161409 | choquei         |     388 | 13.96 |          68 |  2.45 |               2324 | 83.60 |
| 1319287761048723458 | MarioNawfal     |     186 |  6.77 |          91 |  3.31 |               2471 | 89.92 |
| 1326229737551912960 | libsoftiktok    |      33 |  1.31 |          78 |  3.10 |               2407 | 95.59 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         111 |  4.56 |               2316 | 95.11 |
|            16106584 | stillgray       |     285 | 11.83 |          96 |  3.98 |               2029 | 84.19 |
|          3096147528 | dom_lucre       |     279 | 12.51 |          88 |  3.95 |               1863 | 83.54 |
|            76766018 | Dexerto         |     222 | 10.95 |         170 |  8.39 |               1635 | 80.66 |
| 1366565625401909249 | WallStreetSilv  |     103 |  5.38 |          68 |  3.55 |               1742 | 91.06 |
|          3260357396 | thehealthb0t    |     313 | 17.05 |          27 |  1.47 |               1496 | 81.48 |
|  805532293951606785 | MattWallace888  |     220 | 12.01 |          84 |  4.59 |               1528 | 83.41 |
| 1528943647185678336 | LeadingReport   |      54 |  2.95 |          33 |  1.80 |               1744 | 95.25 |
|  801203059359678464 | DrEliDavid      |      36 |  1.98 |         148 |  8.12 |               1638 | 89.90 |
|            80233893 | jakeshieldsajj  |      97 |  5.34 |          97 |  5.34 |               1622 | 89.32 |
| 1374968033265864706 | TaraBull808     |      87 |  4.90 |          34 |  1.91 |               1655 | 93.19 |
| 1429400366819512323 | RadioGenoa      |      52 |  2.96 |          86 |  4.90 |               1617 | 92.14 |
| 1099579684981944320 | WallStreetApes  |      88 |  5.15 |          16 |  0.94 |               1605 | 93.91 |
|           537709549 | LauraLoomer     |     197 | 11.98 |          43 |  2.62 |               1404 | 85.40 |
|  855481986290524160 | historyinmemes  |     136 |  8.34 |         173 | 10.61 |               1322 | 81.05 |
|          2538322138 | PicturesFoIder  |     104 |  6.44 |         177 | 10.97 |               1333 | 82.59 |
| 1344356576786866176 | RepMTG          |      67 |  4.55 |          50 |  3.39 |               1357 | 92.06 |
|           971820228 | CerfiaFR        |      66 |  4.60 |          86 |  5.99 |               1284 | 89.42 |
|           312696314 | Partisangirl    |     101 |  7.11 |          62 |  4.36 |               1258 | 88.53 |
| 1389913567671975937 | cb_doge         |      35 |  2.49 |          80 |  5.69 |               1290 | 91.81 |
|  890061634181373952 | CollinRugg      |      44 |  3.13 |          61 |  4.34 |               1299 | 92.52 |
|            91416107 | OliLondonTV     |      62 |  4.42 |          80 |  5.70 |               1261 | 89.88 |
| 1486069441259397125 | harris_wins     |      18 |  1.33 |          54 |  3.98 |               1286 | 94.70 |
|           133663801 | BFMTV           |     130 |  9.59 |          81 |  5.97 |               1145 | 84.44 |
| 1355721251180961792 | GuntherEagleman |      45 |  3.34 |          30 |  2.23 |               1272 | 94.43 |
|            32867753 | silvano_trotta  |     323 | 24.01 |          13 |  0.97 |               1009 | 75.02 |
|            14281853 | Conservatives   |      72 |  5.44 |          42 |  3.17 |               1210 | 91.39 |
|            49849732 | petrogustavo    |      33 |  2.51 |          37 |  2.82 |               1243 | 94.67 |
|            25073877 | realDonaldTrump |       5 |  0.39 |         110 |  8.48 |               1182 | 91.13 |
|            15212187 | bennyjohnson    |      65 |  5.02 |          43 |  3.32 |               1186 | 91.65 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1138 | 89.96 |
|            52660746 | Israel          |       6 |  0.48 |         117 |  9.29 |               1137 | 90.24 |
|            17469289 | nypost          |      95 |  7.60 |          52 |  4.16 |               1103 | 88.24 |
|           592730371 | JackPosobiec    |      74 |  5.93 |          36 |  2.88 |               1138 | 91.19 |
|  750683331260321792 | MAstronomers    |      71 |  5.73 |          66 |  5.33 |               1102 | 88.94 |
|  807355480188141568 | DramaAlert      |     199 | 16.31 |          71 |  5.82 |                950 | 77.87 |
|  855483824351924224 | liz_churchill10 |     136 | 11.23 |          23 |  1.90 |               1052 | 86.87 |
|           133938408 | krassenstein    |      10 |  0.85 |          72 |  6.15 |               1088 | 92.99 |
|            37501849 | Quakeprediction |      29 |  2.49 |           1 |  0.09 |               1136 | 97.43 |
|           452540168 | DailyLoud       |     294 | 25.28 |          72 |  6.19 |                797 | 68.53 |
| 1455903807389458436 | BladeoftheS     |     159 | 13.95 |          16 |  1.40 |                965 | 84.65 |
|            96684891 | kharaguchi      |     120 | 10.56 |           9 |  0.79 |               1007 | 88.64 |
|           177101260 | Rainmaker1973   |      17 |  1.51 |         121 | 10.72 |                991 | 87.78 |
|           255471924 | mfa_russia      |     134 | 12.18 |          53 |  4.82 |                913 | 83.00 |
| 1335132884278108161 | stats_feed      |     109 | 10.04 |          56 |  5.16 |                921 | 84.81 |
|             7587032 | SkyNews         |      33 |  3.11 |          39 |  3.68 |                989 | 93.21 |
|           132339474 | EdKrassen       |      20 |  1.91 |          49 |  4.68 |                977 | 93.40 |
| 1121807798826930177 | MyLordBebo      |     113 | 10.90 |          30 |  2.89 |                894 | 86.21 |
|            16635277 | Acyn            |      33 |  3.19 |          66 |  6.38 |                935 | 90.43 |
|           428454304 | harryjsisson    |      14 |  1.36 |          34 |  3.30 |                983 | 95.34 |
|            69156861 | TheChiefNerd    |       6 |  0.60 |          11 |  1.10 |                979 | 98.29 |
| 1486473049146904576 | InternetH0F     |     104 | 10.58 |         119 | 12.11 |                760 | 77.31 |
|            90954365 | earthquakejapan |      87 |  8.86 |           0 |  0.00 |                895 | 91.14 |
|          2670726740 | LulaOficial     |       5 |  0.51 |          22 |  2.24 |                955 | 97.25 |
|           524869533 | QudsNen         |      58 |  5.94 |          31 |  3.17 |                888 | 90.89 |
| 1224185690713460736 | goddeketal      |      49 |  5.05 |          25 |  2.57 |                897 | 92.38 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.46 |          23 |  2.37 |                895 | 92.17 |
|            51241574 | AP              |      13 |  1.35 |          44 |  4.57 |                906 | 94.08 |
| 1471414739880189955 | kirawontmiss    |      46 |  4.79 |         149 | 15.52 |                765 | 79.69 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.44 |          35 |  3.65 |                890 | 92.90 |
|           848279118 | Kahlissee       |      37 |  3.87 |          39 |  4.08 |                879 | 92.04 |
|           706646642 | EmbassyofRussia |     106 | 11.22 |          33 |  3.49 |                806 | 85.29 |
|           138203134 | AOC             |      12 |  1.27 |          46 |  4.87 |                886 | 93.86 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|            22703645 | TuckerCarlson   |       1 |  0.11 |          87 |  9.43 |                835 | 90.47 |
|            37491797 | stkirsch        |      62 |  6.75 |          11 |  1.20 |                846 | 92.06 |
| 1043185714437992449 | catturd2        |      38 |  4.15 |          42 |  4.59 |                836 | 91.27 |
| 1529763962094596097 | wideawake_media |      13 |  1.42 |          13 |  1.42 |                887 | 97.15 |
|           242827267 | PierrePoilievre |       0 |  0.00 |          25 |  2.75 |                885 | 97.25 |
|           292929271 | charliekirk11   |      39 |  4.32 |          27 |  2.99 |                837 | 92.69 |
|  896550698543874049 | UTDTrey         |      76 |  8.43 |          65 |  7.21 |                761 | 84.37 |
|             7996082 | el_pais         |      37 |  4.13 |          28 |  3.12 |                831 | 92.75 |
|           130557513 | mehdirhasan     |      14 |  1.59 |          27 |  3.07 |                838 | 95.34 |
|            62957739 | eduardomenoni   |     200 | 22.78 |           8 |  0.91 |                670 | 76.31 |
| 1430497892314218502 | Resist_05       |      44 |  5.03 |          24 |  2.75 |                806 | 92.22 |
| 1538230472299270144 | ayeejuju        |      76 |  8.74 |          94 | 10.80 |                700 | 80.46 |
|           374712154 | TRobinsonNewEra |      38 |  4.38 |          21 |  2.42 |                808 | 93.19 |
|            14128609 | felipeneto      |      18 |  2.12 |          30 |  3.54 |                800 | 94.34 |
|            11347122 | GavinNewsom     |      10 |  1.18 |          22 |  2.60 |                814 | 96.22 |
| 1138458175663988738 | PopBase         |      49 |  5.80 |          64 |  7.57 |                732 | 86.63 |
| 1288319695658135552 | realstewpeters  |     180 | 21.35 |          29 |  3.44 |                634 | 75.21 |
| 1342174584594960384 | iamyesyouareno  |      50 |  5.95 |          49 |  5.83 |                741 | 88.21 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
|  875856268056969216 | DC_Draino       |      16 |  1.91 |          17 |  2.03 |                806 | 96.07 |
| 1356434353623093249 | greg16676935420 |      19 |  2.27 |          94 | 11.24 |                723 | 86.48 |
|           978932870 | CapitanBitcoin  |      37 |  4.46 |          21 |  2.53 |                772 | 93.01 |
|          3331429624 | Metropoles      |      70 |  8.44 |          21 |  2.53 |                738 | 89.02 |
|          2828212668 | AMAZlNGNATURE   |      57 |  6.91 |          98 | 11.88 |                670 | 81.21 |
|            14594813 | folha           |      30 |  3.65 |          22 |  2.68 |                770 | 93.67 |
|  855300206086168576 | HumansNoContext |      88 | 10.81 |          67 |  8.23 |                659 | 80.96 |
|           333357345 | Cobratate       |      30 |  3.69 |          77 |  9.47 |                706 | 86.84 |
| 1128337957289697281 | SprinterFamily  |     203 | 25.00 |          18 |  2.22 |                591 | 72.78 |
| 1187524450809536513 | vitoquiles      |      38 |  4.68 |          22 |  2.71 |                752 | 92.61 |
|  780460754910732288 | DiscussingFilm  |      32 |  3.96 |          85 | 10.52 |                691 | 85.52 |
| 1225234593789423616 | Megatron_ron    |     105 | 13.12 |          38 |  4.75 |                657 | 82.12 |
|          1542228578 | JDVance         |       6 |  0.75 |          52 |  6.52 |                740 | 92.73 |
|           109398997 | stopvaccinating |      92 | 11.57 |          10 |  1.26 |                693 | 87.17 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          14 |  1.77 |                779 | 98.23 |
| 1562038858988064768 | TheFigen\_      |      96 | 12.15 |          75 |  9.49 |                619 | 78.35 |
| 1302674142630760450 | EverythingOOC   |      67 |  8.49 |          87 | 11.03 |                635 | 80.48 |
|          1446465174 | akafacehots     |      83 | 10.53 |          24 |  3.05 |                681 | 86.42 |
|          1640929196 | mmpadellan      |      23 |  2.92 |          15 |  1.91 |                749 | 95.17 |
|            80820758 | JLMelenchon     |      52 |  6.63 |          48 |  6.12 |                684 | 87.24 |
| 1593929531148144645 | stairwayto3dom  |      55 |  7.06 |          27 |  3.47 |                697 | 89.47 |
| 1087757588622651397 | porqueTTarg     |     161 | 20.85 |          28 |  3.63 |                583 | 75.52 |
| 1080188052365029376 | acnewsitics     |       8 |  1.05 |          46 |  6.04 |                708 | 92.91 |
| 1492007194388279333 | LibertyCappy    |      27 |  3.57 |          53 |  7.01 |                676 | 89.42 |
| 1090084079964348419 | MrSinha\_       |      16 |  2.14 |           8 |  1.07 |                724 | 96.79 |
|           463142998 | SuppressedNws   |      12 |  1.61 |          33 |  4.42 |                701 | 93.97 |
|            46302096 | JoeyMannarinoUS |      45 |  6.05 |          20 |  2.69 |                679 | 91.26 |
|  826065164504006657 | mtgreenee       |      31 |  4.21 |          31 |  4.21 |                675 | 91.59 |
|          4429003533 | PopCrave        |      40 |  5.48 |          55 |  7.53 |                635 | 86.99 |
|          4020276615 | JMilei          |       4 |  0.55 |          44 |  6.04 |                681 | 93.42 |
|           319774010 | kirinjisinken   |     395 | 54.41 |          11 |  1.52 |                320 | 44.08 |
|           535707261 | eldiarioes      |       8 |  1.10 |          14 |  1.93 |                702 | 96.96 |
|             4239551 | amuse           |      21 |  2.91 |          21 |  2.91 |                679 | 94.17 |
|            27000730 | Timcast         |      14 |  1.94 |          34 |  4.72 |                673 | 93.34 |
|          1626294277 | spectatorindex  |      10 |  1.40 |          45 |  6.28 |                661 | 92.32 |
|  959531564341317632 | AlertesInfos    |      33 |  4.61 |          47 |  6.56 |                636 | 88.83 |
|            18576537 | IDF             |       0 |  0.00 |          71 |  9.94 |                643 | 90.06 |
|  707231479047315456 | PeterSweden7    |      21 |  2.95 |           8 |  1.12 |                683 | 95.93 |
|          1500129642 | MattWalshBlog   |      11 |  1.56 |          28 |  3.97 |                667 | 94.48 |
| 1138842105856573445 | NoContextHumans |      62 |  8.79 |          82 | 11.63 |                561 | 79.57 |
| 1062754443798532096 | MakisMD         |      20 |  2.84 |          10 |  1.42 |                674 | 95.74 |
|           330262748 | FabrizioRomano  |      57 |  8.12 |          83 | 11.82 |                562 | 80.06 |
|           294293982 | Rothmus         |      41 |  5.91 |          37 |  5.33 |                616 | 88.76 |
| 1604139215406727170 | CensoredMen     |      34 |  4.90 |          46 |  6.63 |                614 | 88.47 |
|            39344374 | DonaldJTrumpJr  |      26 |  3.76 |          36 |  5.20 |                630 | 91.04 |
|             1137701 | DavidSacks      |      21 |  3.05 |          53 |  7.69 |                615 | 89.26 |
|           288277167 | atrupar         |      16 |  2.34 |          25 |  3.65 |                644 | 94.01 |
| 1625843518643085312 | creepydotorg    |      54 |  7.93 |          49 |  7.20 |                578 | 84.88 |
| 1179892477714718721 | gunsnrosesgirl3 |      21 |  3.09 |          80 | 11.76 |                579 | 85.15 |
|            14436030 | elmundoes       |      59 |  8.74 |          12 |  1.78 |                604 | 89.48 |
|          2161051908 | AvivaKlompas    |      13 |  1.93 |          61 |  9.05 |                600 | 89.02 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.49 |          25 |  3.72 |                637 | 94.79 |
|           472412809 | f_philippot     |     105 | 15.65 |          14 |  2.09 |                552 | 82.27 |
|           337808606 | RobertKennedyJr |      11 |  1.64 |          33 |  4.93 |                625 | 93.42 |
|            19069018 | jreichelt       |      17 |  2.54 |          19 |  2.84 |                632 | 94.61 |
|  818893114979061761 | JoJoFromJerz    |      10 |  1.50 |          18 |  2.70 |                638 | 95.80 |
| 1600964443122421769 | CNviolations    |       7 |  1.07 |          73 | 11.13 |                576 | 87.80 |
| 1661674273122160641 | \_maakun\_\_2   |     133 | 20.27 |           0 |  0.00 |                523 | 79.73 |
|  795188519115358208 | tweetsoku1      |     209 | 31.96 |           5 |  0.76 |                440 | 67.28 |
|  918197046871502849 | siteptbr        |      77 | 11.79 |          19 |  2.91 |                557 | 85.30 |
| 1111976778065723393 | nocontextfooty  |      65 | 10.03 |          53 |  8.18 |                530 | 81.79 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
|            14260960 | JustinTrudeau   |       1 |  0.15 |          46 |  7.11 |                600 | 92.74 |
| 1221462414744596483 | RpsAgainstTrump |       8 |  1.24 |          66 | 10.26 |                569 | 88.49 |
|              759251 | CNN             |      16 |  2.50 |          18 |  2.81 |                606 | 94.69 |
| 1434450096557596680 | PolitlcsUK      |      17 |  2.66 |          40 |  6.26 |                582 | 91.08 |
| 1392864463204782082 | WarMonitors     |      19 |  2.98 |          28 |  4.39 |                591 | 92.63 |
| 1158115510606815232 | therealbuni     |      71 | 11.18 |          23 |  3.62 |                541 | 85.20 |
| 1298372735383605249 | RonFilipkowski  |      18 |  2.83 |          31 |  4.88 |                586 | 92.28 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      50 |  7.92 |           6 |  0.95 |                575 | 91.13 |
|            19017675 | Nigel_Farage    |       5 |  0.80 |          24 |  3.85 |                594 | 95.35 |
|            62513246 | jk_rowling      |       0 |  0.00 |          55 |  8.87 |                565 | 91.13 |
| 1155845777039810560 | richimedhurst   |      17 |  2.75 |          22 |  3.55 |                580 | 93.70 |
| 1227799690579718144 | VivekGRamaswamy |      13 |  2.10 |          30 |  4.85 |                576 | 93.05 |
|           195853497 | EuropeInvasions |      39 |  6.34 |          38 |  6.18 |                538 | 87.48 |
|            19923515 | abc_es          |      69 | 11.24 |          19 |  3.09 |                526 | 85.67 |
|          4691437897 | darrengrimes\_  |      31 |  5.06 |          15 |  2.45 |                567 | 92.50 |
| 1432287556129812484 | himuro398       |     125 | 20.42 |           9 |  1.47 |                478 | 78.10 |
|  930212715422998530 | Travis_4_Trump  |      28 |  4.58 |          16 |  2.62 |                567 | 92.80 |
|            55329156 | RNCResearch     |      31 |  5.10 |          19 |  3.12 |                558 | 91.78 |
| 1200616796295847936 | unusual_whales  |      76 | 12.50 |          25 |  4.11 |                507 | 83.39 |
|           109065990 | RealAlexJones   |      40 |  6.60 |          16 |  2.64 |                550 | 90.76 |
|            14173315 | NBCNews         |      11 |  1.84 |          17 |  2.84 |                570 | 95.32 |
| 1393726565809278976 | AdameMedia      |      15 |  2.51 |          25 |  4.19 |                557 | 93.30 |
|              742143 | BBCWorld        |      22 |  3.69 |          34 |  5.70 |                540 | 90.60 |
|           351491321 | wallstwolverine |      18 |  3.02 |          25 |  4.19 |                553 | 92.79 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.36 |                579 | 97.64 |
|            94324983 | FonsiLoaiza     |      12 |  2.02 |          19 |  3.20 |                562 | 94.77 |
| 1450241520859156483 | geoscience16    |     163 | 27.49 |           4 |  0.67 |                426 | 71.84 |
|           458008892 | EylonALevy      |       0 |  0.00 |          38 |  6.42 |                554 | 93.58 |
| 1483750637275860993 | catsscareme2021 |      21 |  3.58 |          10 |  1.71 |                555 | 94.71 |
|            14377605 | TheDemocrats    |       2 |  0.35 |          20 |  3.45 |                557 | 96.20 |
|            81371986 | LozzaFox        |      30 |  5.24 |          19 |  3.32 |                524 | 91.45 |
| 1339166129110065152 | GBNEWS          |      24 |  4.20 |           8 |  1.40 |                540 | 94.41 |
|            39692424 | AlertaNews24    |     107 | 18.80 |          16 |  2.81 |                446 | 78.38 |
| 1201670995435646976 | laurenboebert   |      17 |  2.99 |          22 |  3.87 |                530 | 93.15 |
|             5734902 | tagesschau      |      15 |  2.68 |          21 |  3.76 |                523 | 93.56 |
|           611986351 | KimDotcom       |      28 |  5.01 |          33 |  5.90 |                498 | 89.09 |
|              807095 | nytimes         |      19 |  3.46 |          19 |  3.46 |                511 | 93.08 |
| 1157689921349521410 | esjesjesj       |      14 |  2.57 |          42 |  7.72 |                488 | 89.71 |
|            36749572 | Alphafox78      |     101 | 18.63 |          34 |  6.27 |                407 | 75.09 |
|            65045121 | owenjonesjourno |       9 |  1.67 |          21 |  3.89 |                510 | 94.44 |
