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
|            44196397 | elonmusk        |      97 |  0.62 |        1792 | 11.51 |              13678 | 87.87 |
| 1151913018936053760 | jacksonhinklle  |     336 |  8.15 |         209 |  5.07 |               3578 | 86.78 |
|           629698642 | BGatesIsaPyscho |     907 | 22.16 |          96 |  2.35 |               3090 | 75.49 |
| 1349149096909668363 | POTUS           |      57 |  1.51 |         235 |  6.22 |               3488 | 92.28 |
|          3376321847 | iluminatibot    |     631 | 17.60 |          87 |  2.43 |               2867 | 79.97 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         105 |  2.93 |               3474 | 97.04 |
|              939091 | JoeBiden        |      15 |  0.44 |         296 |  8.64 |               3113 | 90.92 |
|           191871143 | VigilantFox     |      32 |  1.00 |          45 |  1.40 |               3129 | 97.60 |
|          1291128494 | ShaykhSulaiman  |     253 |  7.91 |         154 |  4.81 |               2792 | 87.28 |
| 1552795969959636992 | EndWokeness     |      66 |  2.07 |         130 |  4.07 |               2996 | 93.86 |
| 1446231057259433988 | DrLoupis        |     206 |  7.31 |         113 |  4.01 |               2500 | 88.68 |
|  953378142198161409 | choquei         |     368 | 13.58 |          67 |  2.47 |               2274 | 83.94 |
| 1222773302441148416 | visegrad24      |     153 |  5.68 |         253 |  9.39 |               2289 | 84.94 |
| 1319287761048723458 | MarioNawfal     |     185 |  6.91 |          88 |  3.29 |               2403 | 89.80 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         108 |  4.49 |               2289 | 95.18 |
| 1326229737551912960 | libsoftiktok    |      32 |  1.34 |          73 |  3.06 |               2279 | 95.60 |
|            16106584 | stillgray       |     281 | 11.90 |          95 |  4.02 |               1986 | 84.08 |
|          3096147528 | dom_lucre       |     277 | 12.53 |          88 |  3.98 |               1846 | 83.49 |
|            76766018 | Dexerto         |     218 | 11.04 |         161 |  8.15 |               1596 | 80.81 |
| 1366565625401909249 | WallStreetSilv  |     101 |  5.35 |          67 |  3.55 |               1719 | 91.10 |
|            80233893 | jakeshieldsajj  |      97 |  5.39 |          96 |  5.34 |               1605 | 89.27 |
| 1528943647185678336 | LeadingReport   |      53 |  2.96 |          31 |  1.73 |               1707 | 95.31 |
|  801203059359678464 | DrEliDavid      |      34 |  1.91 |         146 |  8.21 |               1599 | 89.88 |
|  805532293951606785 | MattWallace888  |     214 | 12.26 |          81 |  4.64 |               1450 | 83.09 |
| 1429400366819512323 | RadioGenoa      |      50 |  2.91 |          85 |  4.95 |               1583 | 92.14 |
| 1374968033265864706 | TaraBull808     |      80 |  4.67 |          33 |  1.93 |               1601 | 93.41 |
|          3260357396 | thehealthb0t    |     266 | 15.71 |          23 |  1.36 |               1404 | 82.93 |
| 1099579684981944320 | WallStreetApes  |      85 |  5.11 |          15 |  0.90 |               1564 | 93.99 |
|           537709549 | LauraLoomer     |     197 | 12.14 |          42 |  2.59 |               1384 | 85.27 |
|  855481986290524160 | historyinmemes  |     134 |  8.32 |         170 | 10.56 |               1306 | 81.12 |
|          2538322138 | PicturesFoIder  |     101 |  6.42 |         172 | 10.94 |               1299 | 82.63 |
| 1344356576786866176 | RepMTG          |      66 |  4.48 |          50 |  3.40 |               1356 | 92.12 |
|           312696314 | Partisangirl    |     101 |  7.21 |          61 |  4.36 |               1238 | 88.43 |
|            91416107 | OliLondonTV     |      62 |  4.44 |          80 |  5.73 |               1253 | 89.82 |
|  890061634181373952 | CollinRugg      |      44 |  3.19 |          58 |  4.21 |               1276 | 92.60 |
|           971820228 | CerfiaFR        |      62 |  4.58 |          83 |  6.13 |               1210 | 89.30 |
|            14281853 | Conservatives   |      72 |  5.45 |          42 |  3.18 |               1206 | 91.36 |
| 1486069441259397125 | harris_wins     |      16 |  1.21 |          53 |  4.02 |               1248 | 94.76 |
| 1355721251180961792 | GuntherEagleman |      44 |  3.37 |          30 |  2.30 |               1231 | 94.33 |
| 1389913567671975937 | cb_doge         |      33 |  2.54 |          79 |  6.08 |               1187 | 91.38 |
|            32867753 | silvano_trotta  |     314 | 24.78 |          13 |  1.03 |                940 | 74.19 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1138 | 89.96 |
|           133663801 | BFMTV           |     122 |  9.65 |          74 |  5.85 |               1068 | 84.49 |
|            49849732 | petrogustavo    |      33 |  2.61 |          35 |  2.77 |               1195 | 94.62 |
|            15212187 | bennyjohnson    |      66 |  5.23 |          42 |  3.33 |               1153 | 91.44 |
|            52660746 | Israel          |       6 |  0.48 |         118 |  9.40 |               1131 | 90.12 |
|            25073877 | realDonaldTrump |       5 |  0.40 |         101 |  8.16 |               1131 | 91.43 |
|           592730371 | JackPosobiec    |      73 |  5.94 |          35 |  2.85 |               1120 | 91.21 |
|            17469289 | nypost          |      93 |  7.60 |          51 |  4.17 |               1080 | 88.24 |
|  855483824351924224 | liz_churchill10 |     132 | 11.12 |          23 |  1.94 |               1032 | 86.94 |
|  750683331260321792 | MAstronomers    |      68 |  5.87 |          61 |  5.26 |               1030 | 88.87 |
|            37501849 | Quakeprediction |      29 |  2.51 |           1 |  0.09 |               1124 | 97.40 |
|           452540168 | DailyLoud       |     294 | 25.81 |          71 |  6.23 |                774 | 67.95 |
|           133938408 | krassenstein    |      10 |  0.88 |          72 |  6.35 |               1052 | 92.77 |
|  807355480188141568 | DramaAlert      |     189 | 16.71 |          67 |  5.92 |                875 | 77.37 |
| 1455903807389458436 | BladeoftheS     |     156 | 14.08 |          17 |  1.53 |                935 | 84.39 |
|           255471924 | mfa_russia      |     133 | 12.14 |          53 |  4.84 |                910 | 83.03 |
|           177101260 | Rainmaker1973   |      17 |  1.55 |         118 | 10.79 |                959 | 87.66 |
|            96684891 | kharaguchi      |     114 | 10.53 |           9 |  0.83 |                960 | 88.64 |
| 1335132884278108161 | stats_feed      |     109 | 10.16 |          55 |  5.13 |                909 | 84.72 |
|           132339474 | EdKrassen       |      20 |  1.95 |          47 |  4.59 |                957 | 93.46 |
|             7587032 | SkyNews         |      30 |  2.99 |          35 |  3.49 |                938 | 93.52 |
|            16635277 | Acyn            |      33 |  3.29 |          65 |  6.48 |                905 | 90.23 |
| 1121807798826930177 | MyLordBebo      |     107 | 10.69 |          30 |  3.00 |                864 | 86.31 |
|           428454304 | harryjsisson    |      13 |  1.32 |          32 |  3.25 |                939 | 95.43 |
|            69156861 | TheChiefNerd    |       5 |  0.51 |           9 |  0.92 |                964 | 98.57 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.47 |          23 |  2.37 |                893 | 92.16 |
| 1224185690713460736 | goddeketal      |      49 |  5.07 |          24 |  2.48 |                893 | 92.44 |
|           524869533 | QudsNen         |      58 |  6.01 |          31 |  3.21 |                876 | 90.78 |
| 1486473049146904576 | InternetH0F     |     101 | 10.51 |         115 | 11.97 |                745 | 77.52 |
|            90954365 | earthquakejapan |      87 |  9.09 |           0 |  0.00 |                870 | 90.91 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.45 |          35 |  3.66 |                889 | 92.89 |
|          2670726740 | LulaOficial     |       5 |  0.53 |          22 |  2.31 |                924 | 97.16 |
|           706646642 | EmbassyofRussia |     106 | 11.23 |          33 |  3.50 |                805 | 85.28 |
|            51241574 | AP              |      12 |  1.28 |          44 |  4.69 |                882 | 94.03 |
|           848279118 | Kahlissee       |      38 |  4.09 |          39 |  4.20 |                852 | 91.71 |
| 1471414739880189955 | kirawontmiss    |      45 |  4.85 |         145 | 15.62 |                738 | 79.53 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|            22703645 | TuckerCarlson   |       1 |  0.11 |          87 |  9.46 |                832 | 90.43 |
|            37491797 | stkirsch        |      62 |  6.75 |          11 |  1.20 |                846 | 92.06 |
|           138203134 | AOC             |      11 |  1.20 |          44 |  4.81 |                860 | 93.99 |
| 1529763962094596097 | wideawake_media |      13 |  1.48 |          13 |  1.48 |                854 | 97.05 |
|           242827267 | PierrePoilievre |       0 |  0.00 |          24 |  2.73 |                854 | 97.27 |
| 1043185714437992449 | catturd2        |      35 |  4.00 |          40 |  4.57 |                801 | 91.44 |
| 1430497892314218502 | Resist_05       |      44 |  5.09 |          24 |  2.77 |                797 | 92.14 |
|             7996082 | el_pais         |      36 |  4.17 |          28 |  3.24 |                799 | 92.58 |
|           130557513 | mehdirhasan     |      14 |  1.63 |          26 |  3.03 |                817 | 95.33 |
|  896550698543874049 | UTDTrey         |      73 |  8.56 |          63 |  7.39 |                717 | 84.06 |
| 1538230472299270144 | ayeejuju        |      73 |  8.58 |          94 | 11.05 |                684 | 80.38 |
|           374712154 | TRobinsonNewEra |      38 |  4.50 |          21 |  2.49 |                785 | 93.01 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
|            11347122 | GavinNewsom     |      10 |  1.19 |          22 |  2.62 |                807 | 96.19 |
|           292929271 | charliekirk11   |      38 |  4.54 |          24 |  2.87 |                775 | 92.59 |
| 1288319695658135552 | realstewpeters  |     180 | 21.66 |          29 |  3.49 |                622 | 74.85 |
|  875856268056969216 | DC_Draino       |      11 |  1.34 |          17 |  2.07 |                793 | 96.59 |
| 1342174584594960384 | iamyesyouareno  |      49 |  5.99 |          47 |  5.75 |                722 | 88.26 |
|            14128609 | felipeneto      |      18 |  2.21 |          28 |  3.44 |                768 | 94.35 |
|          2828212668 | AMAZlNGNATURE   |      55 |  6.77 |          98 | 12.07 |                659 | 81.16 |
|           978932870 | CapitanBitcoin  |      37 |  4.56 |          21 |  2.59 |                753 | 92.85 |
| 1128337957289697281 | SprinterFamily  |     203 | 25.06 |          19 |  2.35 |                588 | 72.59 |
|  855300206086168576 | HumansNoContext |      90 | 11.19 |          67 |  8.33 |                647 | 80.47 |
| 1356434353623093249 | greg16676935420 |      19 |  2.37 |          92 | 11.47 |                691 | 86.16 |
|            62957739 | eduardomenoni   |     167 | 21.01 |           8 |  1.01 |                620 | 77.99 |
|          3331429624 | Metropoles      |      64 |  8.05 |          21 |  2.64 |                710 | 89.31 |
|           109398997 | stopvaccinating |      91 | 11.52 |          10 |  1.27 |                689 | 87.22 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          15 |  1.90 |                774 | 98.10 |
| 1302674142630760450 | EverythingOOC   |      66 |  8.37 |          87 | 11.03 |                636 | 80.61 |
|            14594813 | folha           |      29 |  3.69 |          22 |  2.80 |                735 | 93.51 |
| 1138458175663988738 | PopBase         |      46 |  5.85 |          58 |  7.38 |                682 | 86.77 |
|           333357345 | Cobratate       |      29 |  3.76 |          77 |  9.97 |                666 | 86.27 |
| 1225234593789423616 | Megatron_ron    |     105 | 13.60 |          35 |  4.53 |                632 | 81.87 |
| 1187524450809536513 | vitoquiles      |      38 |  4.93 |          22 |  2.85 |                711 | 92.22 |
| 1562038858988064768 | TheFigen\_      |      95 | 12.37 |          75 |  9.77 |                598 | 77.86 |
|  780460754910732288 | DiscussingFilm  |      30 |  3.94 |          80 | 10.50 |                652 | 85.56 |
|          1640929196 | mmpadellan      |      22 |  2.90 |          15 |  1.98 |                722 | 95.13 |
|          1446465174 | akafacehots     |      78 | 10.30 |          24 |  3.17 |                655 | 86.53 |
| 1593929531148144645 | stairwayto3dom  |      49 |  6.47 |          27 |  3.57 |                681 | 89.96 |
|          1542228578 | JDVance         |       6 |  0.80 |          45 |  5.97 |                703 | 93.24 |
|            80820758 | JLMelenchon     |      50 |  6.70 |          45 |  6.03 |                651 | 87.27 |
| 1492007194388279333 | LibertyCappy    |      26 |  3.49 |          52 |  6.98 |                667 | 89.53 |
| 1087757588622651397 | porqueTTarg     |     155 | 20.86 |          27 |  3.63 |                561 | 75.50 |
| 1080188052365029376 | acnewsitics     |       8 |  1.09 |          44 |  6.01 |                680 | 92.90 |
|            46302096 | JoeyMannarinoUS |      45 |  6.16 |          20 |  2.74 |                666 | 91.11 |
|  826065164504006657 | mtgreenee       |      31 |  4.26 |          30 |  4.13 |                666 | 91.61 |
|            18576537 | IDF             |       0 |  0.00 |          71 | 10.00 |                639 | 90.00 |
|           463142998 | SuppressedNws   |      10 |  1.41 |          31 |  4.38 |                667 | 94.21 |
|            27000730 | Timcast         |      13 |  1.84 |          34 |  4.81 |                660 | 93.35 |
| 1090084079964348419 | MrSinha\_       |      16 |  2.26 |           7 |  0.99 |                684 | 96.75 |
|          4020276615 | JMilei          |       4 |  0.57 |          39 |  5.56 |                658 | 93.87 |
|          4429003533 | PopCrave        |      39 |  5.56 |          53 |  7.56 |                609 | 86.88 |
| 1062754443798532096 | MakisMD         |      20 |  2.86 |          10 |  1.43 |                670 | 95.71 |
|           319774010 | kirinjisinken   |     383 | 54.95 |          11 |  1.58 |                303 | 43.47 |
|             4239551 | amuse           |      18 |  2.59 |          21 |  3.02 |                656 | 94.39 |
| 1604139215406727170 | CensoredMen     |      34 |  4.90 |          46 |  6.63 |                614 | 88.47 |
|          1626294277 | spectatorindex  |       8 |  1.15 |          42 |  6.06 |                643 | 92.78 |
| 1138842105856573445 | NoContextHumans |      58 |  8.39 |          81 | 11.72 |                552 | 79.88 |
|          1500129642 | MattWalshBlog   |      11 |  1.60 |          28 |  4.08 |                648 | 94.32 |
|  707231479047315456 | PeterSweden7    |      21 |  3.06 |           7 |  1.02 |                658 | 95.92 |
|           330262748 | FabrizioRomano  |      54 |  7.94 |          82 | 12.06 |                544 | 80.00 |
|           535707261 | eldiarioes      |       8 |  1.18 |          14 |  2.06 |                658 | 96.76 |
| 1179892477714718721 | gunsnrosesgirl3 |      21 |  3.11 |          80 | 11.83 |                575 | 85.06 |
|            39344374 | DonaldJTrumpJr  |      26 |  3.89 |          34 |  5.08 |                609 | 91.03 |
|  959531564341317632 | AlertesInfos    |      29 |  4.34 |          40 |  5.99 |                599 | 89.67 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.50 |          25 |  3.75 |                631 | 94.74 |
|           288277167 | atrupar         |      15 |  2.27 |          24 |  3.63 |                622 | 94.10 |
|             1137701 | DavidSacks      |      20 |  3.03 |          48 |  7.28 |                591 | 89.68 |
|          2161051908 | AvivaKlompas    |      13 |  1.97 |          60 |  9.10 |                586 | 88.92 |
|           337808606 | RobertKennedyJr |      11 |  1.67 |          31 |  4.71 |                616 | 93.62 |
|           472412809 | f_philippot     |     105 | 16.03 |          14 |  2.14 |                536 | 81.83 |
| 1625843518643085312 | creepydotorg    |      55 |  8.40 |          46 |  7.02 |                554 | 84.58 |
|           294293982 | Rothmus         |      40 |  6.13 |          34 |  5.21 |                578 | 88.65 |
|            14436030 | elmundoes       |      58 |  8.91 |          12 |  1.84 |                581 | 89.25 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
| 1111976778065723393 | nocontextfooty  |      63 |  9.92 |          53 |  8.35 |                519 | 81.73 |
|            19069018 | jreichelt       |      17 |  2.69 |          18 |  2.84 |                598 | 94.47 |
|              759251 | CNN             |      16 |  2.56 |          15 |  2.40 |                595 | 95.05 |
| 1392864463204782082 | WarMonitors     |      17 |  2.72 |          28 |  4.49 |                579 | 92.79 |
| 1434450096557596680 | PolitlcsUK      |      16 |  2.57 |          40 |  6.42 |                567 | 91.01 |
|            14260960 | JustinTrudeau   |       1 |  0.16 |          45 |  7.27 |                573 | 92.57 |
| 1155845777039810560 | richimedhurst   |      17 |  2.75 |          22 |  3.56 |                579 | 93.69 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      49 |  7.94 |           6 |  0.97 |                562 | 91.09 |
| 1158115510606815232 | therealbuni     |      65 | 10.57 |          23 |  3.74 |                527 | 85.69 |
| 1661674273122160641 | \_maakun\_\_2   |     120 | 19.54 |           0 |  0.00 |                494 | 80.46 |
|  795188519115358208 | tweetsoku1      |     198 | 32.57 |           4 |  0.66 |                406 | 66.78 |
|  930212715422998530 | Travis_4_Trump  |      28 |  4.61 |          16 |  2.63 |                564 | 92.76 |
| 1227799690579718144 | VivekGRamaswamy |      13 |  2.14 |          30 |  4.94 |                564 | 92.92 |
|            55329156 | RNCResearch     |      31 |  5.12 |          19 |  3.14 |                556 | 91.75 |
|            19017675 | Nigel_Farage    |       5 |  0.83 |          23 |  3.81 |                575 | 95.36 |
|            19923515 | abc_es          |      68 | 11.28 |          18 |  2.99 |                517 | 85.74 |
|            62513246 | jk_rowling      |       0 |  0.00 |          50 |  8.31 |                552 | 91.69 |
| 1221462414744596483 | RpsAgainstTrump |       8 |  1.34 |          60 | 10.02 |                531 | 88.65 |
|  918197046871502849 | siteptbr        |      70 | 11.71 |          17 |  2.84 |                511 | 85.45 |
| 1298372735383605249 | RonFilipkowski  |      18 |  3.01 |          30 |  5.02 |                550 | 91.97 |
|            14173315 | NBCNews         |      11 |  1.85 |          17 |  2.86 |                567 | 95.29 |
|           195853497 | EuropeInvasions |      33 |  5.56 |          37 |  6.23 |                524 | 88.22 |
|          4691437897 | darrengrimes\_  |      31 |  5.22 |          15 |  2.53 |                548 | 92.26 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.38 |                575 | 97.62 |
|           458008892 | EylonALevy      |       0 |  0.00 |          39 |  6.66 |                547 | 93.34 |
|  818893114979061761 | JoJoFromJerz    |      10 |  1.71 |          16 |  2.74 |                559 | 95.56 |
| 1483750637275860993 | catsscareme2021 |      21 |  3.61 |          10 |  1.72 |                551 | 94.67 |
| 1200616796295847936 | unusual_whales  |      73 | 12.72 |          22 |  3.83 |                479 | 83.45 |
| 1432287556129812484 | himuro398       |     113 | 19.72 |           7 |  1.22 |                453 | 79.06 |
|              742143 | BBCWorld        |      22 |  3.85 |          33 |  5.77 |                517 | 90.38 |
|            14377605 | TheDemocrats    |       2 |  0.35 |          20 |  3.50 |                550 | 96.15 |
| 1450241520859156483 | geoscience16    |     159 | 27.80 |           4 |  0.70 |                409 | 71.50 |
| 1201670995435646976 | laurenboebert   |      17 |  3.00 |          22 |  3.88 |                528 | 93.12 |
|            94324983 | FonsiLoaiza     |      11 |  1.96 |          19 |  3.38 |                532 | 94.66 |
|           611986351 | KimDotcom       |      27 |  4.85 |          33 |  5.92 |                497 | 89.23 |
| 1339166129110065152 | GBNEWS          |      24 |  4.31 |           7 |  1.26 |                526 | 94.43 |
|            81371986 | LozzaFox        |      29 |  5.23 |          18 |  3.24 |                508 | 91.53 |
|             5734902 | tagesschau      |      14 |  2.55 |          21 |  3.83 |                513 | 93.61 |
|            39692424 | AlertaNews24    |     105 | 19.20 |          16 |  2.93 |                426 | 77.88 |
| 1393726565809278976 | AdameMedia      |      12 |  2.22 |          22 |  4.07 |                507 | 93.72 |
|              807095 | nytimes         |      19 |  3.53 |          19 |  3.53 |                501 | 92.95 |
|           109065990 | RealAlexJones   |      39 |  7.25 |          13 |  2.42 |                486 | 90.33 |
|            65045121 | owenjonesjourno |       9 |  1.69 |          21 |  3.93 |                504 | 94.38 |
|           371381075 | sandrousseau    |      29 |  5.43 |          44 |  8.24 |                461 | 86.33 |
|            36749572 | Alphafox78      |      97 | 18.34 |          33 |  6.24 |                399 | 75.43 |
|  896466491587080194 | greg_price11    |      17 |  3.21 |          20 |  3.78 |                492 | 93.01 |
|           351491321 | wallstwolverine |      16 |  3.06 |          22 |  4.21 |                485 | 92.73 |
