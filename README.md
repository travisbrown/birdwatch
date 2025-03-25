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

|          Twitter ID | Screen name     | Helpful |     % | Not helpful |     % | Needs more ratings |      % |
| ------------------: | --------------- | ------: | ----: | ----------: | ----: | -----------------: | -----: |
|            44196397 | elonmusk        |     133 |  0.62 |        2371 | 11.05 |              18957 |  88.33 |
|           629698642 | BGatesIsaPyscho |    1241 | 22.16 |         144 |  2.57 |               4215 |  75.27 |
|          3376321847 | iluminatibot    |     881 | 19.37 |         114 |  2.51 |               3554 |  78.13 |
| 1151913018936053760 | jacksonhinklle  |     365 |  8.05 |         235 |  5.18 |               3933 |  86.76 |
| 1552795969959636992 | EndWokeness     |      73 |  1.80 |         174 |  4.30 |               3800 |  93.90 |
| 1349149096909668363 | POTUS46Archive  |      60 |  1.51 |         253 |  6.36 |               3664 |  92.13 |
|           191871143 | VigilantFox     |      35 |  0.96 |          53 |  1.45 |               3576 |  97.60 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         103 |  2.85 |               3507 |  97.12 |
| 1326229737551912960 | libsoftiktok    |      55 |  1.53 |         124 |  3.44 |               3421 |  95.03 |
| 1319287761048723458 | MarioNawfal     |     202 |  5.62 |         112 |  3.12 |               3278 |  91.26 |
|              939091 | JoeBiden        |      16 |  0.46 |         304 |  8.70 |               3173 |  90.84 |
|          1291128494 | ShaykhSulaiman  |     265 |  7.67 |         159 |  4.60 |               3032 |  87.73 |
| 1222773302441148416 | visegrad24      |     170 |  5.02 |         304 |  8.98 |               2911 |  86.00 |
|  953378142198161409 | choquei         |     485 | 15.38 |          75 |  2.38 |               2594 |  82.24 |
| 1446231057259433988 | DrLoupis        |     216 |  7.22 |         125 |  4.18 |               2652 |  88.61 |
|            16106584 | stillgray       |     320 | 11.19 |         115 |  4.02 |               2424 |  84.78 |
|          3260357396 | thehealthb0t    |     517 | 18.65 |          41 |  1.48 |               2214 |  79.87 |
|  896550698543874049 | UTDTrey         |     298 | 10.90 |          78 |  2.85 |               2357 |  86.24 |
| 1099579684981944320 | WallStreetApes  |     123 |  4.99 |          32 |  1.30 |               2309 |  93.71 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         111 |  4.56 |               2317 |  95.11 |
|          3096147528 | dom_lucre       |     298 | 12.43 |          95 |  3.96 |               2005 |  83.61 |
|            76766018 | Dexerto         |     245 | 10.49 |         190 |  8.13 |               1901 |  81.38 |
| 1429400366819512323 | RadioGenoa      |      63 |  2.76 |         126 |  5.52 |               2092 |  91.71 |
| 1374968033265864706 | TaraBull808     |     132 |  5.99 |          46 |  2.09 |               2024 |  91.92 |
| 1528943647185678336 | LeadingReport   |      58 |  2.65 |          42 |  1.92 |               2085 |  95.42 |
| 1366565625401909249 | WallStreetMav   |     106 |  4.92 |          80 |  3.71 |               1969 |  91.37 |
|  805532293951606785 | MattWallace888  |     244 | 11.65 |          92 |  4.39 |               1758 |  83.95 |
|  801203059359678464 | DrEliDavid      |      44 |  2.22 |         157 |  7.94 |               1777 |  89.84 |
|            80233893 | jakeshieldsajj  |     107 |  5.54 |         109 |  5.64 |               1715 |  88.81 |
|           971820228 | CerfiaFR        |      88 |  4.75 |         116 |  6.27 |               1647 |  88.98 |
|           537709549 | LauraLoomer     |     219 | 11.85 |          52 |  2.81 |               1577 |  85.34 |
|          2538322138 | PicturesFoIder  |     126 |  6.92 |         194 | 10.66 |               1500 |  82.42 |
|            49849732 | petrogustavo    |      45 |  2.48 |          48 |  2.64 |               1723 |  94.88 |
|            32867753 | silvano_trotta  |     391 | 22.15 |          15 |  0.85 |               1359 |  77.00 |
|            25073877 | realDonaldTrump |       5 |  0.29 |         182 | 10.39 |               1565 |  89.33 |
|  855481986290524160 | historyinmemes  |     149 |  8.69 |         178 | 10.38 |               1388 |  80.93 |
| 1389913567671975937 | cb_doge         |      38 |  2.22 |          97 |  5.67 |               1577 |  92.11 |
| 1355721251180961792 | GuntherEagleman |      53 |  3.12 |          42 |  2.47 |               1604 |  94.41 |
|            15212187 | bennyjohnson    |      86 |  5.15 |          61 |  3.65 |               1523 |  91.20 |
|           312696314 | Partisangirl    |     133 |  7.97 |          75 |  4.49 |               1461 |  87.54 |
|  890061634181373952 | CollinRugg      |      51 |  3.16 |          72 |  4.46 |               1490 |  92.37 |
|           133663801 | BFMTV           |     151 |  9.41 |          94 |  5.86 |               1359 |  84.73 |
| 1344356576786866176 | RepMTG          |      71 |  4.48 |          57 |  3.59 |               1458 |  91.93 |
|  855483824351924224 | liz_churchill10 |     158 | 10.08 |          30 |  1.91 |               1380 |  88.01 |
|           592730371 | JackPosobiec    |      90 |  5.86 |          45 |  2.93 |               1401 |  91.21 |
|  807355480188141568 | DramaAlert      |     245 | 15.95 |          93 |  6.05 |               1198 |  77.99 |
|            17469289 | nypost          |     112 |  7.37 |          66 |  4.34 |               1341 |  88.28 |
|           133938408 | krassenstein    |      15 |  0.99 |         100 |  6.59 |               1403 |  92.42 |
| 1486069441259397125 | DemocraticWins  |      18 |  1.21 |          56 |  3.76 |               1414 |  95.03 |
|            91416107 | OliLondonTV     |      65 |  4.44 |          87 |  5.95 |               1311 |  89.61 |
|           177101260 | Rainmaker1973   |      25 |  1.76 |         160 | 11.25 |               1237 |  86.99 |
|  750683331260321792 | MAstronomers    |      71 |  4.99 |          85 |  5.98 |               1266 |  89.03 |
|            16635277 | Acyn            |      50 |  3.52 |         109 |  7.68 |               1260 |  88.79 |
| 1455903807389458436 | BladeoftheS     |     168 | 12.13 |          25 |  1.81 |               1192 |  86.06 |
|            14281853 | Conservatives   |      72 |  5.31 |          42 |  3.10 |               1242 |  91.59 |
|            62957739 | eduardomenoni   |     325 | 23.97 |          17 |  1.25 |               1014 |  74.78 |
|           428454304 | harryjsisson    |      16 |  1.18 |          42 |  3.10 |               1295 |  95.71 |
|            14594813 | folha           |      46 |  3.43 |          35 |  2.61 |               1262 |  93.97 |
|            96684891 | kharaguchi      |     133 | 10.15 |           9 |  0.69 |               1168 |  89.16 |
|            52660746 | Israel          |       6 |  0.46 |         123 |  9.42 |               1177 |  90.12 |
|           452540168 | DailyLoud       |     320 | 25.12 |          78 |  6.12 |                876 |  68.76 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1139 |  89.97 |
|           292929271 | charliekirk11   |      46 |  3.72 |          41 |  3.31 |               1150 |  92.97 |
|           132339474 | EdKrassen       |      23 |  1.87 |          62 |  5.03 |               1148 |  93.11 |
| 1121807798826930177 | MyLordBebo      |     136 | 11.09 |          34 |  2.77 |               1056 |  86.13 |
| 1486473049146904576 | InternetH0F     |     128 | 10.44 |         138 | 11.26 |                960 |  78.30 |
|             7587032 | SkyNews         |      38 |  3.11 |          48 |  3.93 |               1136 |  92.96 |
|            37501849 | Quakeprediction |      41 |  3.37 |           2 |  0.16 |               1174 |  96.47 |
| 1335132884278108161 | stats_feed      |     117 |  9.62 |          65 |  5.35 |               1034 |  85.03 |
|            69156861 | TheChiefNerd    |       9 |  0.76 |          14 |  1.18 |               1165 |  98.06 |
| 1138458175663988738 | PopBase         |      57 |  4.80 |          97 |  8.17 |               1033 |  87.03 |
|            51241574 | AP              |      19 |  1.61 |          53 |  4.48 |               1110 |  93.91 |
| 1224185690713460736 | goddeketal      |      60 |  5.09 |          35 |  2.97 |               1084 |  91.94 |
|          2670726740 | LulaOficial     |       5 |  0.43 |          26 |  2.24 |               1132 |  97.33 |
| 1529763962094596097 | wideawake_media |      22 |  1.89 |          14 |  1.21 |               1125 |  96.90 |
|          1542228578 | JDVance         |       6 |  0.52 |          95 |  8.20 |               1058 |  91.29 |
| 1043185714437992449 | catturd2        |      51 |  4.43 |          52 |  4.52 |               1047 |  91.04 |
| 1471414739880189955 | kirawontmiss    |      55 |  4.83 |         169 | 14.85 |                914 |  80.32 |
|           255471924 | mfa_russia      |     137 | 12.06 |          54 |  4.75 |                945 |  83.19 |
|           138203134 | AOC             |      12 |  1.06 |          57 |  5.06 |               1058 |  93.88 |
|          3331429624 | Metropoles      |      94 |  8.36 |          27 |  2.40 |               1004 |  89.24 |
|             4239551 | amuse           |      33 |  2.99 |          38 |  3.45 |               1032 |  93.56 |
|           423692278 | AkademiksTV     |      49 |  4.54 |          14 |  1.30 |               1016 |  94.16 |
|           242827267 | PierrePoilievre |       1 |  0.09 |          35 |  3.26 |               1037 |  96.64 |
|           333357345 | Cobratate       |      57 |  5.33 |          90 |  8.41 |                923 |  86.26 |
| 1600964443122421769 | CNviolations    |      17 |  1.62 |         105 |  9.99 |                929 |  88.39 |
|           288277167 | atrupar         |      27 |  2.59 |          55 |  5.28 |                960 |  92.13 |
|          2828212668 | AMAZlNGNATURE   |      75 |  7.24 |         129 | 12.45 |                832 |  80.31 |
| 1538230472299270144 | ayeejuju        |      82 |  7.93 |         114 | 11.03 |                838 |  81.04 |
| 1342174584594960384 | iamyesyouareno  |      54 |  5.23 |          60 |  5.81 |                919 |  88.96 |
|             7996082 | el_pais         |      41 |  3.97 |          30 |  2.91 |                961 |  93.12 |
| 1577761560394665984 | DiedSuddenly\_  |      56 |  5.44 |          23 |  2.24 |                950 |  92.32 |
| 1080188052365029376 | acnewsitics     |       9 |  0.88 |          63 |  6.14 |                954 |  92.98 |
| 1356434353623093249 | greg16676935420 |      20 |  1.95 |         114 | 11.11 |                892 |  86.94 |
|           848279118 | Kahlissee       |      37 |  3.63 |          41 |  4.02 |                942 |  92.35 |
|           524869533 | QudsNen         |      58 |  5.70 |          31 |  3.05 |                928 |  91.25 |
| 1087757588622651397 | porqueTTarg     |     217 | 21.36 |          36 |  3.54 |                763 |  75.10 |
|            90954365 | earthquakejapan |      90 |  8.89 |           0 |  0.00 |                922 |  91.11 |
|  780460754910732288 | DiscussingFilm  |      37 |  3.67 |         109 | 10.82 |                861 |  85.50 |
|  875856268056969216 | DC_Draino       |      25 |  2.49 |          23 |  2.29 |                956 |  95.22 |
|  855300206086168576 | HumansNoContext |     102 | 10.18 |          82 |  8.18 |                818 |  81.64 |
|            22703645 | TuckerCarlson   |       1 |  0.10 |          94 |  9.40 |                905 |  90.50 |
|  795188519115358208 | tweetsoku1      |     295 | 29.56 |           7 |  0.70 |                696 |  69.74 |
|           142393421 | GloboNews       |      25 |  2.54 |          18 |  1.83 |                943 |  95.64 |
| 1111976778065723393 | nocontextfooty  |     111 | 11.26 |          62 |  6.29 |                813 |  82.45 |
| 1323730225067339784 | WhiteHouse46    |      33 |  3.35 |          35 |  3.56 |                916 |  93.09 |
|           130557513 | mehdirhasan     |      14 |  1.43 |          31 |  3.18 |                931 |  95.39 |
|           374712154 | TRobinsonNewEra |      40 |  4.10 |          23 |  2.36 |                913 |  93.55 |
| 1288319695658135552 | realstewpeters  |     191 | 19.71 |          39 |  4.02 |                739 |  76.26 |
|            80820758 | JLMelenchon     |      64 |  6.63 |          70 |  7.25 |                831 |  86.11 |
| 1430497892314218502 | Resist_05       |      47 |  4.91 |          30 |  3.13 |                881 |  91.96 |
|  959531564341317632 | AlertesInfos    |      40 |  4.21 |          66 |  6.95 |                843 |  88.83 |
|           706646642 | EmbassyofRussia |     106 | 11.19 |          33 |  3.48 |                808 |  85.32 |
|  918197046871502849 | siteptbr        |     106 | 11.31 |          30 |  3.20 |                801 |  85.49 |
| 1090084079964348419 | MrSinha\_       |      23 |  2.45 |          10 |  1.07 |                904 |  96.48 |
|  707231479047315456 | PeterSweden7    |      25 |  2.68 |          17 |  1.82 |                892 |  95.50 |
|            37491797 | stkirsch        |      62 |  6.65 |          12 |  1.29 |                858 |  92.06 |
|           978932870 | CapitanBitcoin  |      39 |  4.18 |          26 |  2.79 |                867 |  93.03 |
|           463142998 | SuppressedNws   |      15 |  1.61 |          37 |  3.97 |                879 |  94.41 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.89 |                890 |  96.11 |
| 1562038858988064768 | TheFigen\_      |     112 | 12.11 |          93 | 10.05 |                720 |  77.84 |
| 1432287556129812484 | himuro398       |     182 | 19.72 |          13 |  1.41 |                728 |  78.87 |
|            11347122 | GavinNewsom     |      10 |  1.08 |          23 |  2.49 |                889 |  96.42 |
|           294293982 | Rothmus         |      55 |  5.97 |          45 |  4.89 |                821 |  89.14 |
|          4429003533 | PopCrave        |      50 |  5.48 |          75 |  8.22 |                787 |  86.29 |
| 1393726565809278976 | AdameMedia      |      22 |  2.41 |          42 |  4.61 |                848 |  92.98 |
| 1128337957289697281 | SprinterObserve |     221 | 24.31 |          21 |  2.31 |                667 |  73.38 |
|  885786515640655873 | CR7Brasil       |     462 | 50.94 |          14 |  1.54 |                431 |  47.52 |
|          1640929196 | mmpadellan      |      27 |  3.00 |          20 |  2.22 |                854 |  94.78 |
| 1187524450809536513 | vitoquiles      |      40 |  4.44 |          27 |  3.00 |                834 |  92.56 |
|            14128609 | felipeneto      |      18 |  2.00 |          31 |  3.44 |                851 |  94.56 |
|          1446465174 | akafaceUS       |      89 |  9.92 |          26 |  2.90 |                782 |  87.18 |
| 1200616796295847936 | unusual_whales  |     102 | 11.37 |          42 |  4.68 |                753 |  83.95 |
| 1225234593789423616 | Megatron_ron    |     114 | 12.74 |          43 |  4.80 |                738 |  82.46 |
|           109065990 | RealAlexJones   |      57 |  6.42 |          28 |  3.15 |                803 |  90.43 |
|           109398997 | stopvaccinating |     100 | 11.26 |          11 |  1.24 |                777 |  87.50 |
|           319774010 | kirinjisinken   |     483 | 54.45 |          11 |  1.24 |                393 |  44.31 |
| 1221462414744596483 | RpsAgainstTrump |      10 |  1.14 |          91 | 10.39 |                775 |  88.47 |
| 1492007194388279333 | LibertyCappy    |      30 |  3.43 |          62 |  7.09 |                783 |  89.49 |
|  818893114979061761 | JoJoFromJerz    |      12 |  1.37 |          33 |  3.78 |                829 |  94.85 |
| 1179892477714718721 | gunsnrosesgirl3 |      28 |  3.22 |         105 | 12.08 |                736 |  84.70 |
| 1593929531148144645 | stairwayto3dom  |      58 |  6.74 |          32 |  3.72 |                771 |  89.55 |
|          4020276615 | JMilei          |       4 |  0.47 |          63 |  7.33 |                793 |  92.21 |
|           330262748 | FabrizioRomano  |      69 |  8.14 |         104 | 12.26 |                675 |  79.60 |
| 1431774993419956224 | jhmdrei         |     228 | 27.11 |           3 |  0.36 |                610 |  72.53 |
|  935142655213703168 | zoo_bear        |       1 |  0.12 |          14 |  1.68 |                820 |  98.20 |
|           535707261 | eldiarioes      |      18 |  2.16 |          15 |  1.80 |                801 |  96.04 |
| 1686901686185721857 | TrumpDailyPosts |      11 |  1.33 |          35 |  4.22 |                783 |  94.45 |
|            19017675 | Nigel_Farage    |       7 |  0.85 |          35 |  4.23 |                786 |  94.93 |
|            14260960 | JustinTrudeau   |       1 |  0.12 |          65 |  7.87 |                760 |  92.01 |
|            27000730 | Timcast         |      15 |  1.82 |          41 |  4.98 |                768 |  93.20 |
|            46302096 | JoeyMannarinoUS |      60 |  7.28 |          24 |  2.91 |                740 |  89.81 |
| 1514045582968598530 | CFC_Janty       |      66 |  8.03 |          31 |  3.77 |                725 |  88.20 |
|            39692424 | AlertaNews24    |     139 | 16.93 |          26 |  3.17 |                656 |  79.90 |
| 1302674142630760450 | EverythingOOC   |      69 |  8.49 |          87 | 10.70 |                657 |  80.81 |
| 1849500209488752640 | Inevitablewest  |      26 |  3.21 |          20 |  2.47 |                765 |  94.33 |
| 1157689921349521410 | esjesjesj       |      16 |  1.98 |          54 |  6.69 |                737 |  91.33 |
|          1176335252 | basherkella     |       0 |  0.00 |           0 |  0.00 |                806 | 100.00 |
|            39344374 | DonaldJTrumpJr  |      31 |  3.88 |          42 |  5.26 |                726 |  90.86 |
|          1500129642 | MattWalshBlog   |      11 |  1.38 |          29 |  3.65 |                755 |  94.97 |
|  826065164504006657 | mtgreenee       |      37 |  4.68 |          39 |  4.94 |                714 |  90.38 |
| 1625843518643085312 | creepydotorg    |      58 |  7.38 |          62 |  7.89 |                666 |  84.73 |
|          1626294277 | spectatorindex  |      12 |  1.53 |          50 |  6.39 |                720 |  92.07 |
| 1138842105856573445 | NoContextHumans |      74 |  9.48 |          85 | 10.88 |                622 |  79.64 |
| 1434450096557596680 | PolitlcsUK      |      20 |  2.56 |          45 |  5.77 |                715 |  91.67 |
|            14436030 | elmundoes       |      62 |  7.97 |          13 |  1.67 |                703 |  90.36 |
|           472412809 | f_philippot     |     117 | 15.08 |          17 |  2.19 |                642 |  82.73 |
|            19069018 | jreichelt       |      20 |  2.59 |          21 |  2.72 |                731 |  94.69 |
| 1316995857242378240 | realMaalouf     |      46 |  6.00 |          53 |  6.91 |                668 |  87.09 |
|          2162812627 | nicksortor      |      37 |  4.89 |          23 |  3.04 |                696 |  92.06 |
| 1661674273122160641 | \_maakun\_\_2   |     139 | 18.39 |           0 |  0.00 |                617 |  81.61 |
|            16906660 | Grummz          |       5 |  0.66 |          26 |  3.45 |                722 |  95.88 |
|             1137701 | DavidSacks      |      21 |  2.80 |          63 |  8.41 |                665 |  88.79 |
|          4691437897 | darrengrimes\_  |      36 |  4.83 |          18 |  2.42 |                691 |  92.75 |
| 1485068728412913666 | interesting_aIl |      91 | 12.25 |          49 |  6.59 |                603 |  81.16 |
|            16397147 | liberal_party   |       0 |  0.00 |          16 |  2.16 |                724 |  97.84 |
|            18576537 | IDF             |       0 |  0.00 |          75 | 10.15 |                664 |  89.85 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      55 |  7.44 |           7 |  0.95 |                677 |  91.61 |
| 1158115510606815232 | therealbuni     |      83 | 11.25 |          28 |  3.79 |                627 |  84.96 |
|              742143 | BBCWorld        |      30 |  4.08 |          37 |  5.03 |                669 |  90.90 |
| 1151126931439407107 | RimaHas         |      16 |  2.17 |          56 |  7.61 |                664 |  90.22 |
| 1227799690579718144 | VivekGRamaswamy |      16 |  2.17 |          35 |  4.76 |                685 |  93.07 |
|           195853497 | UpdateNews724   |      59 |  8.03 |          39 |  5.31 |                637 |  86.67 |
|           351491321 | wallstwolverine |      20 |  2.72 |          35 |  4.76 |                680 |  92.52 |
| 1062754443798532096 | MakisMD         |      20 |  2.72 |          10 |  1.36 |                705 |  95.92 |
| 1298372735383605249 | RonFilipkowski  |      19 |  2.59 |          37 |  5.03 |                679 |  92.38 |
|            94324983 | FonsiLoaiza     |      13 |  1.80 |          20 |  2.77 |                690 |  95.44 |
|          2470489110 | mattariver1     |     124 | 17.42 |          18 |  2.53 |                570 |  80.06 |
| 1392864463204782082 | WarMonitors     |      20 |  2.82 |          33 |  4.65 |                657 |  92.54 |
|          2161051908 | AvivaKlompas    |      13 |  1.84 |          63 |  8.92 |                630 |  89.24 |
|           112103858 | ALeaument       |      28 |  3.99 |          33 |  4.70 |                641 |  91.31 |
|           221593437 | droidindo       |       0 |  0.00 |           0 |  0.00 |                699 | 100.00 |
| 1711732984338878465 | NewsLiberdade   |      36 |  5.15 |          15 |  2.15 |                648 |  92.70 |
|            19923515 | abc_es          |      74 | 10.62 |          21 |  3.01 |                602 |  86.37 |
| 1594871021978652673 | FearedBuck      |      86 | 12.36 |          56 |  8.05 |                554 |  79.60 |
| 1449140157903482882 | BRICSinfo       |      13 |  1.87 |          41 |  5.90 |                641 |  92.23 |
| 1604139215406727170 | CensoredMen     |      34 |  4.89 |          46 |  6.62 |                615 |  88.49 |
| 1450241520859156483 | geoscience16    |     183 | 26.45 |           5 |  0.72 |                504 |  72.83 |
|            15801906 | TristanSnell    |      23 |  3.34 |          23 |  3.34 |                643 |  93.32 |
|              759251 | CNN             |      16 |  2.36 |          20 |  2.95 |                643 |  94.70 |
