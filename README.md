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
|            44196397 | elonmusk        |     107 |  0.64 |        1896 | 11.32 |              14749 | 88.04 |
|           629698642 | BGatesIsaPyscho |     997 | 22.06 |         105 |  2.32 |               3417 | 75.61 |
| 1151913018936053760 | jacksonhinklle  |     351 |  8.20 |         215 |  5.02 |               3714 | 86.78 |
|          3376321847 | iluminatibot    |     694 | 18.12 |          91 |  2.38 |               3045 | 79.50 |
| 1349149096909668363 | POTUS           |      57 |  1.49 |         242 |  6.33 |               3527 | 92.19 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         103 |  2.85 |               3505 | 97.12 |
|              939091 | JoeBiden        |      16 |  0.46 |         301 |  8.70 |               3143 | 90.84 |
| 1552795969959636992 | EndWokeness     |      67 |  1.98 |         142 |  4.19 |               3180 | 93.83 |
|           191871143 | VigilantFox     |      32 |  0.95 |          47 |  1.40 |               3280 | 97.65 |
|          1291128494 | ShaykhSulaiman  |     257 |  7.83 |         157 |  4.78 |               2868 | 87.39 |
| 1446231057259433988 | DrLoupis        |     213 |  7.33 |         119 |  4.09 |               2574 | 88.58 |
| 1222773302441148416 | visegrad24      |     158 |  5.47 |         269 |  9.30 |               2464 | 85.23 |
| 1319287761048723458 | MarioNawfal     |     185 |  6.55 |          92 |  3.26 |               2547 | 90.19 |
|  953378142198161409 | choquei         |     398 | 14.15 |          69 |  2.45 |               2346 | 83.40 |
| 1326229737551912960 | libsoftiktok    |      33 |  1.30 |          80 |  3.14 |               2435 | 95.57 |
|            16106584 | stillgray       |     288 | 11.76 |          97 |  3.96 |               2065 | 84.29 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         111 |  4.56 |               2316 | 95.11 |
|          3096147528 | dom_lucre       |     279 | 12.50 |          88 |  3.94 |               1865 | 83.56 |
|            76766018 | Dexerto         |     224 | 10.86 |         176 |  8.54 |               1662 | 80.60 |
|          3260357396 | thehealthb0t    |     355 | 18.01 |          30 |  1.52 |               1586 | 80.47 |
| 1366565625401909249 | WallStreetSilv  |     103 |  5.31 |          68 |  3.50 |               1770 | 91.19 |
|  805532293951606785 | MattWallace888  |     222 | 11.79 |          84 |  4.46 |               1577 | 83.75 |
| 1528943647185678336 | LeadingReport   |      55 |  2.94 |          33 |  1.76 |               1782 | 95.29 |
| 1374968033265864706 | TaraBull808     |      95 |  5.11 |          35 |  1.88 |               1729 | 93.01 |
|            80233893 | jakeshieldsajj  |      99 |  5.38 |          99 |  5.38 |               1642 | 89.24 |
|  801203059359678464 | DrEliDavid      |      36 |  1.97 |         152 |  8.30 |               1644 | 89.74 |
| 1429400366819512323 | RadioGenoa      |      55 |  3.02 |          88 |  4.84 |               1676 | 92.14 |
| 1099579684981944320 | WallStreetApes  |      89 |  5.11 |          15 |  0.86 |               1637 | 94.03 |
|  855481986290524160 | historyinmemes  |     141 |  8.51 |         173 | 10.44 |               1343 | 81.05 |
|           537709549 | LauraLoomer     |     201 | 12.15 |          43 |  2.60 |               1411 | 85.26 |
|          2538322138 | PicturesFoIder  |     107 |  6.54 |         178 | 10.87 |               1352 | 82.59 |
|           971820228 | CerfiaFR        |      69 |  4.64 |          87 |  5.85 |               1332 | 89.52 |
| 1344356576786866176 | RepMTG          |      67 |  4.53 |          51 |  3.45 |               1362 | 92.03 |
|           312696314 | Partisangirl    |     110 |  7.51 |          64 |  4.37 |               1291 | 88.12 |
| 1389913567671975937 | cb_doge         |      35 |  2.41 |          81 |  5.58 |               1335 | 92.01 |
|  890061634181373952 | CollinRugg      |      44 |  3.10 |          62 |  4.37 |               1314 | 92.54 |
|            91416107 | OliLondonTV     |      62 |  4.42 |          80 |  5.70 |               1261 | 89.88 |
|           133663801 | BFMTV           |     131 |  9.45 |          84 |  6.06 |               1171 | 84.49 |
|            32867753 | silvano_trotta  |     332 | 24.16 |          13 |  0.95 |               1029 | 74.89 |
| 1486069441259397125 | harris_wins     |      17 |  1.24 |          53 |  3.86 |               1304 | 94.91 |
| 1355721251180961792 | GuntherEagleman |      49 |  3.57 |          31 |  2.26 |               1292 | 94.17 |
|            25073877 | realDonaldTrump |       5 |  0.37 |         116 |  8.65 |               1220 | 90.98 |
|            49849732 | petrogustavo    |      34 |  2.54 |          37 |  2.77 |               1265 | 94.69 |
|            14281853 | Conservatives   |      72 |  5.41 |          42 |  3.16 |               1217 | 91.44 |
|            15212187 | bennyjohnson    |      65 |  4.92 |          45 |  3.40 |               1212 | 91.68 |
|  750683331260321792 | MAstronomers    |      71 |  5.40 |          75 |  5.70 |               1169 | 88.90 |
|  896550698543874049 | UTDTrey         |     123 |  9.43 |          68 |  5.21 |               1114 | 85.36 |
|            17469289 | nypost          |     100 |  7.77 |          53 |  4.12 |               1134 | 88.11 |
|  807355480188141568 | DramaAlert      |     212 | 16.61 |          75 |  5.88 |                989 | 77.51 |
|           592730371 | JackPosobiec    |      76 |  5.97 |          37 |  2.90 |               1161 | 91.13 |
|  855483824351924224 | liz_churchill10 |     141 | 11.08 |          26 |  2.04 |               1106 | 86.88 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1138 | 89.96 |
|            52660746 | Israel          |       6 |  0.48 |         118 |  9.36 |               1137 | 90.17 |
|           133938408 | krassenstein    |      10 |  0.84 |          76 |  6.40 |               1101 | 92.75 |
|           177101260 | Rainmaker1973   |      17 |  1.44 |         127 | 10.76 |               1036 | 87.80 |
|            37501849 | Quakeprediction |      29 |  2.48 |           1 |  0.09 |               1138 | 97.43 |
|           452540168 | DailyLoud       |     295 | 25.26 |          73 |  6.25 |                800 | 68.49 |
| 1455903807389458436 | BladeoftheS     |     159 | 13.61 |          17 |  1.46 |                992 | 84.93 |
|            96684891 | kharaguchi      |     123 | 10.55 |           9 |  0.77 |               1034 | 88.68 |
|           255471924 | mfa_russia      |     134 | 12.16 |          53 |  4.81 |                915 | 83.03 |
| 1335132884278108161 | stats_feed      |     109 |  9.95 |          57 |  5.21 |                929 | 84.84 |
|             7587032 | SkyNews         |      33 |  3.10 |          39 |  3.66 |                993 | 93.24 |
| 1121807798826930177 | MyLordBebo      |     115 | 10.81 |          30 |  2.82 |                919 | 86.37 |
|           428454304 | harryjsisson    |      14 |  1.32 |          34 |  3.21 |               1011 | 95.47 |
|           132339474 | EdKrassen       |      20 |  1.89 |          49 |  4.64 |                987 | 93.47 |
|            16635277 | Acyn            |      33 |  3.17 |          68 |  6.53 |                940 | 90.30 |
| 1486473049146904576 | InternetH0F     |     108 | 10.61 |         120 | 11.79 |                790 | 77.60 |
|            69156861 | TheChiefNerd    |       6 |  0.60 |          10 |  1.00 |                988 | 98.41 |
| 1224185690713460736 | goddeketal      |      51 |  5.09 |          25 |  2.50 |                925 | 92.41 |
|          2670726740 | LulaOficial     |       5 |  0.50 |          24 |  2.40 |                971 | 97.10 |
|            51241574 | AP              |      14 |  1.43 |          45 |  4.58 |                923 | 93.99 |
|            90954365 | earthquakejapan |      87 |  8.86 |           0 |  0.00 |                895 | 91.14 |
|            62957739 | eduardomenoni   |     221 | 22.55 |          13 |  1.33 |                746 | 76.12 |
| 1471414739880189955 | kirawontmiss    |      46 |  4.70 |         151 | 15.42 |                782 | 79.88 |
|           524869533 | QudsNen         |      58 |  5.93 |          31 |  3.17 |                889 | 90.90 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.46 |          23 |  2.37 |                895 | 92.17 |
|           848279118 | Kahlissee       |      37 |  3.83 |          40 |  4.14 |                889 | 92.03 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.42 |          35 |  3.63 |                897 | 92.95 |
|           138203134 | AOC             |      12 |  1.26 |          49 |  5.14 |                893 | 93.61 |
| 1043185714437992449 | catturd2        |      38 |  3.98 |          44 |  4.61 |                872 | 91.40 |
| 1529763962094596097 | wideawake_media |      14 |  1.48 |          13 |  1.37 |                921 | 97.15 |
|           706646642 | EmbassyofRussia |     106 | 11.22 |          33 |  3.49 |                806 | 85.29 |
|            22703645 | TuckerCarlson   |       1 |  0.11 |          88 |  9.35 |                852 | 90.54 |
|           242827267 | PierrePoilievre |       0 |  0.00 |          25 |  2.68 |                908 | 97.32 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|            37491797 | stkirsch        |      62 |  6.74 |          11 |  1.20 |                847 | 92.07 |
|           292929271 | charliekirk11   |      40 |  4.35 |          28 |  3.04 |                852 | 92.61 |
|             7996082 | el_pais         |      39 |  4.28 |          28 |  3.07 |                845 | 92.65 |
| 1538230472299270144 | ayeejuju        |      76 |  8.49 |          98 | 10.95 |                721 | 80.56 |
|           130557513 | mehdirhasan     |      14 |  1.57 |          27 |  3.03 |                850 | 95.40 |
| 1138458175663988738 | PopBase         |      49 |  5.53 |          67 |  7.56 |                770 | 86.91 |
| 1342174584594960384 | iamyesyouareno  |      50 |  5.68 |          52 |  5.90 |                779 | 88.42 |
|           374712154 | TRobinsonNewEra |      38 |  4.33 |          21 |  2.39 |                819 | 93.28 |
| 1430497892314218502 | Resist_05       |      44 |  5.03 |          24 |  2.74 |                807 | 92.23 |
|          2828212668 | AMAZlNGNATURE   |      65 |  7.45 |         102 | 11.70 |                705 | 80.85 |
|          3331429624 | Metropoles      |      71 |  8.15 |          21 |  2.41 |                779 | 89.44 |
|            14128609 | felipeneto      |      18 |  2.10 |          30 |  3.50 |                809 | 94.40 |
| 1187524450809536513 | vitoquiles      |      39 |  4.58 |          24 |  2.82 |                789 | 92.61 |
| 1356434353623093249 | greg16676935420 |      19 |  2.23 |          94 | 11.03 |                739 | 86.74 |
| 1288319695658135552 | realstewpeters  |     180 | 21.15 |          29 |  3.41 |                642 | 75.44 |
|            14594813 | folha           |      33 |  3.88 |          22 |  2.59 |                795 | 93.53 |
|  875856268056969216 | DC_Draino       |      16 |  1.89 |          17 |  2.01 |                814 | 96.10 |
|            11347122 | GavinNewsom     |      10 |  1.18 |          22 |  2.60 |                814 | 96.22 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
|           978932870 | CapitanBitcoin  |      37 |  4.43 |          21 |  2.51 |                778 | 93.06 |
|  855300206086168576 | HumansNoContext |      89 | 10.67 |          70 |  8.39 |                675 | 80.94 |
|           333357345 | Cobratate       |      31 |  3.73 |          77 |  9.25 |                724 | 87.02 |
|          1542228578 | JDVance         |       6 |  0.72 |          57 |  6.85 |                769 | 92.43 |
| 1128337957289697281 | SprinterFamily  |     205 | 24.97 |          18 |  2.19 |                598 | 72.84 |
|          1446465174 | akafacehots     |      85 | 10.46 |          24 |  2.95 |                704 | 86.59 |
|  780460754910732288 | DiscussingFilm  |      32 |  3.94 |          85 | 10.47 |                695 | 85.59 |
| 1225234593789423616 | Megatron_ron    |     107 | 13.23 |          39 |  4.82 |                663 | 81.95 |
| 1562038858988064768 | TheFigen\_      |      96 | 11.87 |          77 |  9.52 |                636 | 78.62 |
|          1640929196 | mmpadellan      |      24 |  2.98 |          15 |  1.86 |                766 | 95.16 |
|            80820758 | JLMelenchon     |      51 |  6.34 |          48 |  5.97 |                705 | 87.69 |
|           109398997 | stopvaccinating |      92 | 11.47 |          10 |  1.25 |                700 | 87.28 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          14 |  1.75 |                784 | 98.25 |
| 1087757588622651397 | porqueTTarg     |     172 | 21.74 |          27 |  3.41 |                592 | 74.84 |
| 1302674142630760450 | EverythingOOC   |      67 |  8.48 |          87 | 11.01 |                636 | 80.51 |
| 1593929531148144645 | stairwayto3dom  |      55 |  6.96 |          27 |  3.42 |                708 | 89.62 |
| 1080188052365029376 | acnewsitics     |       8 |  1.02 |          49 |  6.25 |                727 | 92.73 |
| 1492007194388279333 | LibertyCappy    |      29 |  3.74 |          54 |  6.96 |                693 | 89.30 |
| 1090084079964348419 | MrSinha\_       |      17 |  2.20 |           7 |  0.91 |                749 | 96.90 |
|            46302096 | JoeyMannarinoUS |      49 |  6.44 |          22 |  2.89 |                690 | 90.67 |
|           463142998 | SuppressedNws   |      12 |  1.58 |          35 |  4.61 |                712 | 93.81 |
|          4429003533 | PopCrave        |      41 |  5.40 |          60 |  7.91 |                658 | 86.69 |
| 1600964443122421769 | CNviolations    |       9 |  1.20 |          82 | 10.92 |                660 | 87.88 |
|           319774010 | kirinjisinken   |     409 | 54.68 |          11 |  1.47 |                328 | 43.85 |
|          4020276615 | JMilei          |       4 |  0.53 |          46 |  6.15 |                698 | 93.32 |
|  959531564341317632 | AlertesInfos    |      34 |  4.58 |          47 |  6.33 |                661 | 89.08 |
|  826065164504006657 | mtgreenee       |      31 |  4.19 |          31 |  4.19 |                677 | 91.61 |
|           535707261 | eldiarioes      |       8 |  1.09 |          14 |  1.90 |                713 | 97.01 |
|             4239551 | amuse           |      22 |  3.01 |          21 |  2.87 |                688 | 94.12 |
|  707231479047315456 | PeterSweden7    |      21 |  2.88 |           8 |  1.10 |                701 | 96.03 |
|           294293982 | Rothmus         |      45 |  6.18 |          38 |  5.22 |                645 | 88.60 |
|            27000730 | Timcast         |      15 |  2.07 |          34 |  4.68 |                677 | 93.25 |
|            18576537 | IDF             |       0 |  0.00 |          72 |  9.99 |                649 | 90.01 |
|          1626294277 | spectatorindex  |      10 |  1.40 |          46 |  6.42 |                660 | 92.18 |
|           330262748 | FabrizioRomano  |      59 |  8.26 |          86 | 12.04 |                569 | 79.69 |
|          1500129642 | MattWalshBlog   |      11 |  1.54 |          28 |  3.92 |                675 | 94.54 |
|            39344374 | DonaldJTrumpJr  |      26 |  3.67 |          37 |  5.22 |                646 | 91.11 |
| 1138842105856573445 | NoContextHumans |      62 |  8.74 |          82 | 11.57 |                565 | 79.69 |
| 1062754443798532096 | MakisMD         |      20 |  2.84 |          10 |  1.42 |                675 | 95.74 |
| 1179892477714718721 | gunsnrosesgirl3 |      22 |  3.14 |          82 | 11.71 |                596 | 85.14 |
|            14260960 | JustinTrudeau   |       2 |  0.29 |          50 |  7.16 |                646 | 92.55 |
| 1604139215406727170 | CensoredMen     |      34 |  4.90 |          46 |  6.63 |                614 | 88.47 |
|           288277167 | atrupar         |      16 |  2.31 |          25 |  3.61 |                651 | 94.08 |
| 1625843518643085312 | creepydotorg    |      54 |  7.80 |          51 |  7.37 |                587 | 84.83 |
|             1137701 | DavidSacks      |      21 |  3.05 |          53 |  7.69 |                615 | 89.26 |
|            14436030 | elmundoes       |      60 |  8.72 |          12 |  1.74 |                616 | 89.53 |
|            19069018 | jreichelt       |      18 |  2.62 |          20 |  2.91 |                650 | 94.48 |
|  818893114979061761 | JoJoFromJerz    |      10 |  1.46 |          20 |  2.91 |                657 | 95.63 |
|  795188519115358208 | tweetsoku1      |     213 | 31.09 |           5 |  0.73 |                467 | 68.18 |
|  918197046871502849 | siteptbr        |      77 | 11.34 |          19 |  2.80 |                583 | 85.86 |
| 1661674273122160641 | \_maakun\_\_2   |     135 | 19.91 |           0 |  0.00 |                543 | 80.09 |
|           472412809 | f_philippot     |     106 | 15.66 |          14 |  2.07 |                557 | 82.27 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.48 |          25 |  3.69 |                642 | 94.83 |
|          2161051908 | AvivaKlompas    |      13 |  1.93 |          61 |  9.05 |                600 | 89.02 |
| 1111976778065723393 | nocontextfooty  |      65 |  9.67 |          54 |  8.04 |                553 | 82.29 |
|           337808606 | RobertKennedyJr |      11 |  1.64 |          33 |  4.93 |                626 | 93.43 |
| 1221462414744596483 | RpsAgainstTrump |       8 |  1.19 |          67 | 10.00 |                595 | 88.81 |
| 1434450096557596680 | PolitlcsUK      |      18 |  2.71 |          40 |  6.02 |                607 | 91.28 |
| 1227799690579718144 | VivekGRamaswamy |      14 |  2.11 |          31 |  4.68 |                618 | 93.21 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      52 |  8.00 |           7 |  1.08 |                591 | 90.92 |
|              759251 | CNN             |      16 |  2.47 |          18 |  2.77 |                615 | 94.76 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
|            19017675 | Nigel_Farage    |       5 |  0.77 |          24 |  3.71 |                618 | 95.52 |
| 1392864463204782082 | WarMonitors     |      19 |  2.94 |          28 |  4.33 |                599 | 92.72 |
| 1158115510606815232 | therealbuni     |      73 | 11.35 |          23 |  3.58 |                547 | 85.07 |
| 1298372735383605249 | RonFilipkowski  |      18 |  2.83 |          31 |  4.87 |                587 | 92.30 |
|           109065990 | RealAlexJones   |      40 |  6.31 |          16 |  2.52 |                578 | 91.17 |
|            19923515 | abc_es          |      69 | 10.92 |          19 |  3.01 |                544 | 86.08 |
| 1432287556129812484 | himuro398       |     126 | 20.00 |           9 |  1.43 |                495 | 78.57 |
| 1393726565809278976 | AdameMedia      |      17 |  2.72 |          26 |  4.15 |                583 | 93.13 |
|           195853497 | EuropeInvasions |      42 |  6.72 |          38 |  6.08 |                545 | 87.20 |
|            62513246 | jk_rowling      |       0 |  0.00 |          56 |  8.99 |                567 | 91.01 |
| 1200616796295847936 | unusual_whales  |      79 | 12.68 |          26 |  4.17 |                518 | 83.15 |
| 1450241520859156483 | geoscience16    |     170 | 27.29 |           4 |  0.64 |                449 | 72.07 |
|           351491321 | wallstwolverine |      18 |  2.89 |          27 |  4.34 |                577 | 92.77 |
|          4691437897 | darrengrimes\_  |      32 |  5.16 |          15 |  2.42 |                573 | 92.42 |
| 1155845777039810560 | richimedhurst   |      17 |  2.75 |          22 |  3.55 |                580 | 93.70 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.28 |                600 | 97.72 |
|  930212715422998530 | Travis_4_Trump  |      28 |  4.58 |          16 |  2.62 |                567 | 92.80 |
|            55329156 | RNCResearch     |      31 |  5.09 |          19 |  3.12 |                559 | 91.79 |
|              742143 | BBCWorld        |      22 |  3.62 |          34 |  5.59 |                552 | 90.79 |
|            14173315 | NBCNews         |      11 |  1.82 |          17 |  2.82 |                575 | 95.36 |
|            94324983 | FonsiLoaiza     |      12 |  2.00 |          19 |  3.17 |                569 | 94.83 |
|            39692424 | AlertaNews24    |     109 | 18.26 |          17 |  2.85 |                471 | 78.89 |
|           458008892 | EylonALevy      |       0 |  0.00 |          38 |  6.41 |                555 | 93.59 |
|            14377605 | TheDemocrats    |       2 |  0.34 |          22 |  3.72 |                568 | 95.95 |
| 1483750637275860993 | catsscareme2021 |      22 |  3.72 |          10 |  1.69 |                559 | 94.59 |
|            81371986 | LozzaFox        |      30 |  5.17 |          20 |  3.45 |                530 | 91.38 |
| 1339166129110065152 | GBNEWS          |      24 |  4.15 |           8 |  1.38 |                546 | 94.46 |
| 1157689921349521410 | esjesjesj       |      14 |  2.46 |          44 |  7.72 |                512 | 89.82 |
| 1201670995435646976 | laurenboebert   |      17 |  2.99 |          22 |  3.87 |                530 | 93.15 |
|             5734902 | tagesschau      |      15 |  2.65 |          22 |  3.88 |                530 | 93.47 |
| 1151126931439407107 | RimaHas         |      16 |  2.82 |          43 |  7.58 |                508 | 89.59 |
|            36749572 | Alphafox78      |     106 | 18.89 |          35 |  6.24 |                420 | 74.87 |
|           611986351 | KimDotcom       |      28 |  5.01 |          33 |  5.90 |                498 | 89.09 |
| 1492677599390322689 | weirddalle      |      18 |  3.26 |          68 | 12.32 |                466 | 84.42 |
