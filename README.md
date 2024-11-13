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
|            44196397 | elonmusk        |      98 |  0.62 |        1817 | 11.46 |              13947 | 87.93 |
|           629698642 | BGatesIsaPyscho |     925 | 22.12 |          97 |  2.32 |               3159 | 75.56 |
| 1151913018936053760 | jacksonhinklle  |     342 |  8.20 |         209 |  5.01 |               3620 | 86.79 |
| 1349149096909668363 | POTUS           |      57 |  1.50 |         236 |  6.23 |               3496 | 92.27 |
|          3376321847 | iluminatibot    |     639 | 17.70 |          87 |  2.41 |               2884 | 79.89 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         102 |  2.83 |               3505 | 97.15 |
|              939091 | JoeBiden        |      15 |  0.44 |         297 |  8.66 |               3116 | 90.90 |
|           191871143 | VigilantFox     |      32 |  0.99 |          45 |  1.39 |               3163 | 97.62 |
| 1552795969959636992 | EndWokeness     |      66 |  2.04 |         131 |  4.06 |               3033 | 93.90 |
|          1291128494 | ShaykhSulaiman  |     253 |  7.87 |         154 |  4.79 |               2807 | 87.34 |
| 1446231057259433988 | DrLoupis        |     208 |  7.33 |         115 |  4.05 |               2515 | 88.62 |
| 1222773302441148416 | visegrad24      |     155 |  5.66 |         255 |  9.31 |               2328 | 85.03 |
|  953378142198161409 | choquei         |     378 | 13.83 |          67 |  2.45 |               2288 | 83.72 |
| 1319287761048723458 | MarioNawfal     |     185 |  6.81 |          87 |  3.20 |               2444 | 89.99 |
| 1326229737551912960 | libsoftiktok    |      32 |  1.31 |          75 |  3.06 |               2342 | 95.63 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         110 |  4.52 |               2317 | 95.15 |
|            16106584 | stillgray       |     283 | 11.90 |          95 |  3.99 |               2001 | 84.11 |
|          3096147528 | dom_lucre       |     280 | 12.56 |          88 |  3.95 |               1862 | 83.50 |
|            76766018 | Dexerto         |     221 | 11.05 |         166 |  8.30 |               1613 | 80.65 |
| 1366565625401909249 | WallStreetSilv  |     103 |  5.44 |          68 |  3.59 |               1721 | 90.96 |
| 1528943647185678336 | LeadingReport   |      54 |  2.97 |          34 |  1.87 |               1733 | 95.17 |
|  801203059359678464 | DrEliDavid      |      35 |  1.93 |         148 |  8.18 |               1627 | 89.89 |
|            80233893 | jakeshieldsajj  |      97 |  5.38 |          95 |  5.27 |               1611 | 89.35 |
|  805532293951606785 | MattWallace888  |     219 | 12.25 |          83 |  4.64 |               1486 | 83.11 |
| 1374968033265864706 | TaraBull808     |      86 |  4.94 |          33 |  1.89 |               1623 | 93.17 |
| 1429400366819512323 | RadioGenoa      |      50 |  2.89 |          85 |  4.92 |               1594 | 92.19 |
|          3260357396 | thehealthb0t    |     277 | 16.11 |          22 |  1.28 |               1420 | 82.61 |
| 1099579684981944320 | WallStreetApes  |      87 |  5.16 |          14 |  0.83 |               1584 | 94.01 |
|           537709549 | LauraLoomer     |     197 | 12.05 |          42 |  2.57 |               1396 | 85.38 |
|  855481986290524160 | historyinmemes  |     136 |  8.38 |         172 | 10.60 |               1314 | 81.01 |
|          2538322138 | PicturesFoIder  |     104 |  6.52 |         175 | 10.96 |               1317 | 82.52 |
| 1344356576786866176 | RepMTG          |      67 |  4.55 |          50 |  3.40 |               1355 | 92.05 |
|           312696314 | Partisangirl    |     101 |  7.14 |          62 |  4.38 |               1252 | 88.48 |
|            91416107 | OliLondonTV     |      62 |  4.42 |          80 |  5.70 |               1261 | 89.88 |
|  890061634181373952 | CollinRugg      |      44 |  3.15 |          61 |  4.37 |               1292 | 92.48 |
|           971820228 | CerfiaFR        |      62 |  4.47 |          85 |  6.13 |               1239 | 89.39 |
| 1389913567671975937 | cb_doge         |      35 |  2.58 |          79 |  5.81 |               1245 | 91.61 |
| 1486069441259397125 | harris_wins     |      16 |  1.19 |          54 |  4.02 |               1273 | 94.79 |
| 1355721251180961792 | GuntherEagleman |      45 |  3.40 |          31 |  2.34 |               1249 | 94.26 |
|            14281853 | Conservatives   |      72 |  5.45 |          42 |  3.18 |               1206 | 91.36 |
|           133663801 | BFMTV           |     124 |  9.43 |          80 |  6.08 |               1111 | 84.49 |
|            32867753 | silvano_trotta  |     316 | 24.44 |          13 |  1.01 |                964 | 74.56 |
|            15212187 | bennyjohnson    |      66 |  5.14 |          43 |  3.35 |               1174 | 91.50 |
|            49849732 | petrogustavo    |      33 |  2.58 |          35 |  2.74 |               1210 | 94.68 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1138 | 89.96 |
|            25073877 | realDonaldTrump |       5 |  0.40 |         104 |  8.23 |               1155 | 91.38 |
|            52660746 | Israel          |       6 |  0.48 |         116 |  9.21 |               1138 | 90.32 |
|           592730371 | JackPosobiec    |      73 |  5.88 |          35 |  2.82 |               1133 | 91.30 |
|            17469289 | nypost          |      93 |  7.53 |          51 |  4.13 |               1091 | 88.34 |
|  750683331260321792 | MAstronomers    |      71 |  5.97 |          66 |  5.55 |               1053 | 88.49 |
|  855483824351924224 | liz_churchill10 |     132 | 11.10 |          23 |  1.93 |               1034 | 86.96 |
|  807355480188141568 | DramaAlert      |     197 | 16.69 |          69 |  5.85 |                914 | 77.46 |
|            37501849 | Quakeprediction |      29 |  2.49 |           1 |  0.09 |               1136 | 97.43 |
|           133938408 | krassenstein    |      10 |  0.87 |          72 |  6.26 |               1068 | 92.87 |
|           452540168 | DailyLoud       |     294 | 25.57 |          72 |  6.26 |                784 | 68.17 |
| 1455903807389458436 | BladeoftheS     |     157 | 14.01 |          16 |  1.43 |                948 | 84.57 |
|           177101260 | Rainmaker1973   |      17 |  1.54 |         120 | 10.87 |                967 | 87.59 |
|            96684891 | kharaguchi      |     115 | 10.45 |           9 |  0.82 |                977 | 88.74 |
|           255471924 | mfa_russia      |     134 | 12.22 |          53 |  4.83 |                910 | 82.95 |
| 1335132884278108161 | stats_feed      |     109 | 10.15 |          55 |  5.12 |                910 | 84.73 |
|             7587032 | SkyNews         |      30 |  2.90 |          38 |  3.68 |                965 | 93.42 |
|           132339474 | EdKrassen       |      20 |  1.95 |          46 |  4.48 |                961 | 93.57 |
| 1121807798826930177 | MyLordBebo      |     110 | 10.76 |          31 |  3.03 |                881 | 86.20 |
|            16635277 | Acyn            |      33 |  3.23 |          64 |  6.27 |                924 | 90.50 |
|           428454304 | harryjsisson    |      13 |  1.28 |          33 |  3.26 |                966 | 95.45 |
|            69156861 | TheChiefNerd    |       5 |  0.51 |           9 |  0.92 |                968 | 98.57 |
|           524869533 | QudsNen         |      58 |  5.95 |          31 |  3.18 |                885 | 90.86 |
|          2670726740 | LulaOficial     |       5 |  0.51 |          22 |  2.26 |                946 | 97.23 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.46 |          23 |  2.37 |                895 | 92.17 |
| 1224185690713460736 | goddeketal      |      49 |  5.06 |          25 |  2.58 |                895 | 92.36 |
|            90954365 | earthquakejapan |      87 |  9.01 |           0 |  0.00 |                879 | 90.99 |
| 1486473049146904576 | InternetH0F     |     101 | 10.47 |         115 | 11.92 |                749 | 77.62 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.45 |          35 |  3.66 |                889 | 92.89 |
|            51241574 | AP              |      12 |  1.26 |          44 |  4.63 |                895 | 94.11 |
|           706646642 | EmbassyofRussia |     106 | 11.23 |          33 |  3.50 |                805 | 85.28 |
| 1471414739880189955 | kirawontmiss    |      46 |  4.89 |         148 | 15.73 |                747 | 79.38 |
|           848279118 | Kahlissee       |      37 |  3.96 |          39 |  4.17 |                859 | 91.87 |
|           138203134 | AOC             |      12 |  1.30 |          44 |  4.76 |                869 | 93.95 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|            22703645 | TuckerCarlson   |       1 |  0.11 |          87 |  9.46 |                832 | 90.43 |
|            37491797 | stkirsch        |      62 |  6.75 |          11 |  1.20 |                846 | 92.06 |
|           242827267 | PierrePoilievre |       0 |  0.00 |          25 |  2.78 |                875 | 97.22 |
| 1529763962094596097 | wideawake_media |      13 |  1.46 |          13 |  1.46 |                864 | 97.08 |
| 1043185714437992449 | catturd2        |      36 |  4.06 |          41 |  4.62 |                810 | 91.32 |
|           130557513 | mehdirhasan     |      14 |  1.61 |          27 |  3.11 |                827 | 95.28 |
|           292929271 | charliekirk11   |      39 |  4.49 |          25 |  2.88 |                804 | 92.63 |
| 1430497892314218502 | Resist_05       |      44 |  5.07 |          24 |  2.76 |                800 | 92.17 |
|             7996082 | el_pais         |      36 |  4.15 |          28 |  3.23 |                803 | 92.62 |
| 1538230472299270144 | ayeejuju        |      73 |  8.48 |          94 | 10.92 |                694 | 80.60 |
|  896550698543874049 | UTDTrey         |      72 |  8.37 |          64 |  7.44 |                724 | 84.19 |
|           374712154 | TRobinsonNewEra |      38 |  4.47 |          21 |  2.47 |                791 | 93.06 |
|            11347122 | GavinNewsom     |      10 |  1.18 |          22 |  2.60 |                814 | 96.22 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
|            62957739 | eduardomenoni   |     181 | 21.57 |           9 |  1.07 |                649 | 77.35 |
|  875856268056969216 | DC_Draino       |      16 |  1.92 |          17 |  2.04 |                802 | 96.05 |
| 1288319695658135552 | realstewpeters  |     180 | 21.56 |          29 |  3.47 |                626 | 74.97 |
|           978932870 | CapitanBitcoin  |      37 |  4.50 |          21 |  2.55 |                765 | 92.95 |
| 1342174584594960384 | iamyesyouareno  |      51 |  6.21 |          46 |  5.60 |                724 | 88.19 |
|            14128609 | felipeneto      |      18 |  2.21 |          28 |  3.43 |                770 | 94.36 |
| 1356434353623093249 | greg16676935420 |      19 |  2.33 |          93 | 11.40 |                704 | 86.27 |
|          2828212668 | AMAZlNGNATURE   |      55 |  6.75 |          98 | 12.02 |                662 | 81.23 |
|  855300206086168576 | HumansNoContext |      89 | 10.99 |          67 |  8.27 |                654 | 80.74 |
| 1128337957289697281 | SprinterFamily  |     203 | 25.06 |          18 |  2.22 |                589 | 72.72 |
|          3331429624 | Metropoles      |      68 |  8.45 |          21 |  2.61 |                716 | 88.94 |
| 1187524450809536513 | vitoquiles      |      38 |  4.74 |          23 |  2.87 |                741 | 92.39 |
| 1138458175663988738 | PopBase         |      49 |  6.12 |          60 |  7.50 |                691 | 86.38 |
|           333357345 | Cobratate       |      29 |  3.66 |          77 |  9.72 |                686 | 86.62 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          15 |  1.90 |                776 | 98.10 |
| 1225234593789423616 | Megatron_ron    |     105 | 13.27 |          37 |  4.68 |                649 | 82.05 |
|            14594813 | folha           |      29 |  3.67 |          22 |  2.78 |                739 | 93.54 |
|           109398997 | stopvaccinating |      92 | 11.65 |          10 |  1.27 |                688 | 87.09 |
| 1302674142630760450 | EverythingOOC   |      67 |  8.49 |          87 | 11.03 |                635 | 80.48 |
|          1542228578 | JDVance         |       6 |  0.77 |          50 |  6.42 |                723 | 92.81 |
| 1562038858988064768 | TheFigen\_      |      96 | 12.34 |          75 |  9.64 |                607 | 78.02 |
|  780460754910732288 | DiscussingFilm  |      31 |  4.00 |          84 | 10.84 |                660 | 85.16 |
|          1640929196 | mmpadellan      |      22 |  2.86 |          15 |  1.95 |                731 | 95.18 |
|          1446465174 | akafacehots     |      79 | 10.30 |          23 |  3.00 |                665 | 86.70 |
| 1593929531148144645 | stairwayto3dom  |      51 |  6.68 |          27 |  3.54 |                685 | 89.78 |
|            80820758 | JLMelenchon     |      50 |  6.60 |          46 |  6.07 |                662 | 87.34 |
| 1087757588622651397 | porqueTTarg     |     158 | 20.90 |          27 |  3.57 |                571 | 75.53 |
| 1080188052365029376 | acnewsitics     |       8 |  1.07 |          46 |  6.13 |                696 | 92.80 |
| 1492007194388279333 | LibertyCappy    |      27 |  3.60 |          52 |  6.94 |                670 | 89.45 |
|            46302096 | JoeyMannarinoUS |      45 |  6.07 |          20 |  2.70 |                676 | 91.23 |
|  826065164504006657 | mtgreenee       |      31 |  4.22 |          31 |  4.22 |                672 | 91.55 |
|           463142998 | SuppressedNws   |      10 |  1.37 |          32 |  4.37 |                690 | 94.26 |
| 1090084079964348419 | MrSinha\_       |      16 |  2.21 |           7 |  0.97 |                702 | 96.83 |
|            27000730 | Timcast         |      13 |  1.81 |          34 |  4.73 |                672 | 93.46 |
|          4429003533 | PopCrave        |      40 |  5.56 |          53 |  7.37 |                626 | 87.07 |
|           535707261 | eldiarioes      |       8 |  1.12 |          13 |  1.82 |                694 | 97.06 |
|          1626294277 | spectatorindex  |       9 |  1.26 |          44 |  6.18 |                659 | 92.56 |
|            18576537 | IDF             |       0 |  0.00 |          71 | 10.00 |                639 | 90.00 |
|             4239551 | amuse           |      20 |  2.83 |          21 |  2.97 |                665 | 94.19 |
|           319774010 | kirinjisinken   |     383 | 54.33 |          11 |  1.56 |                311 | 44.11 |
|          4020276615 | JMilei          |       4 |  0.57 |          39 |  5.53 |                662 | 93.90 |
|  707231479047315456 | PeterSweden7    |      21 |  2.98 |           7 |  0.99 |                677 | 96.03 |
| 1062754443798532096 | MakisMD         |      20 |  2.86 |          10 |  1.43 |                670 | 95.71 |
|          1500129642 | MattWalshBlog   |      11 |  1.57 |          28 |  4.01 |                660 | 94.42 |
|           330262748 | FabrizioRomano  |      55 |  7.93 |          82 | 11.82 |                557 | 80.26 |
| 1604139215406727170 | CensoredMen     |      34 |  4.90 |          46 |  6.63 |                614 | 88.47 |
| 1138842105856573445 | NoContextHumans |      58 |  8.37 |          81 | 11.69 |                554 | 79.94 |
|  959531564341317632 | AlertesInfos    |      30 |  4.37 |          44 |  6.41 |                612 | 89.21 |
|            39344374 | DonaldJTrumpJr  |      26 |  3.82 |          34 |  4.99 |                621 | 91.19 |
|             1137701 | DavidSacks      |      21 |  3.10 |          51 |  7.53 |                605 | 89.36 |
| 1179892477714718721 | gunsnrosesgirl3 |      21 |  3.11 |          80 | 11.83 |                575 | 85.06 |
|          2161051908 | AvivaKlompas    |      13 |  1.93 |          61 |  9.08 |                598 | 88.99 |
|           288277167 | atrupar         |      16 |  2.39 |          25 |  3.73 |                629 | 93.88 |
|           294293982 | Rothmus         |      40 |  5.97 |          35 |  5.22 |                595 | 88.81 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.49 |          25 |  3.73 |                635 | 94.78 |
|            14436030 | elmundoes       |      58 |  8.67 |          12 |  1.79 |                599 | 89.54 |
|           337808606 | RobertKennedyJr |      11 |  1.65 |          33 |  4.94 |                624 | 93.41 |
| 1625843518643085312 | creepydotorg    |      54 |  8.10 |          48 |  7.20 |                565 | 84.71 |
|           472412809 | f_philippot     |     105 | 15.93 |          13 |  1.97 |                541 | 82.09 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
| 1111976778065723393 | nocontextfooty  |      65 | 10.12 |          53 |  8.26 |                524 | 81.62 |
|            19069018 | jreichelt       |      17 |  2.67 |          18 |  2.83 |                602 | 94.51 |
|            14260960 | JustinTrudeau   |       1 |  0.16 |          45 |  7.08 |                590 | 92.77 |
| 1434450096557596680 | PolitlcsUK      |      17 |  2.68 |          40 |  6.31 |                577 | 91.01 |
| 1392864463204782082 | WarMonitors     |      18 |  2.85 |          28 |  4.43 |                586 | 92.72 |
|              759251 | CNN             |      16 |  2.55 |          15 |  2.39 |                596 | 95.06 |
|  918197046871502849 | siteptbr        |      72 | 11.48 |          18 |  2.87 |                537 | 85.65 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      51 |  8.16 |           6 |  0.96 |                568 | 90.88 |
| 1158115510606815232 | therealbuni     |      69 | 11.06 |          24 |  3.85 |                531 | 85.10 |
| 1661674273122160641 | \_maakun\_\_2   |     121 | 19.52 |           0 |  0.00 |                499 | 80.48 |
|  795188519115358208 | tweetsoku1      |     202 | 32.63 |           4 |  0.65 |                413 | 66.72 |
|            19017675 | Nigel_Farage    |       5 |  0.81 |          24 |  3.88 |                589 | 95.31 |
| 1155845777039810560 | richimedhurst   |      17 |  2.75 |          22 |  3.56 |                579 | 93.69 |
| 1298372735383605249 | RonFilipkowski  |      18 |  2.92 |          31 |  5.03 |                567 | 92.05 |
| 1221462414744596483 | RpsAgainstTrump |       8 |  1.30 |          62 | 10.10 |                544 | 88.60 |
|            62513246 | jk_rowling      |       0 |  0.00 |          53 |  8.67 |                558 | 91.33 |
|  930212715422998530 | Travis_4_Trump  |      28 |  4.58 |          16 |  2.62 |                567 | 92.80 |
| 1227799690579718144 | VivekGRamaswamy |      13 |  2.14 |          30 |  4.93 |                565 | 92.93 |
|            19923515 | abc_es          |      69 | 11.37 |          19 |  3.13 |                519 | 85.50 |
|  818893114979061761 | JoJoFromJerz    |      10 |  1.65 |          17 |  2.80 |                580 | 95.55 |
|            55329156 | RNCResearch     |      31 |  5.12 |          19 |  3.14 |                556 | 91.75 |
|           195853497 | EuropeInvasions |      35 |  5.80 |          38 |  6.30 |                530 | 87.89 |
|          4691437897 | darrengrimes\_  |      31 |  5.17 |          15 |  2.50 |                554 | 92.33 |
|            14173315 | NBCNews         |      11 |  1.85 |          17 |  2.85 |                568 | 95.30 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.37 |                577 | 97.63 |
|              742143 | BBCWorld        |      21 |  3.58 |          34 |  5.79 |                532 | 90.63 |
|           458008892 | EylonALevy      |       0 |  0.00 |          38 |  6.47 |                549 | 93.53 |
| 1200616796295847936 | unusual_whales  |      74 | 12.61 |          23 |  3.92 |                490 | 83.48 |
| 1432287556129812484 | himuro398       |     116 | 19.76 |           7 |  1.19 |                464 | 79.05 |
| 1483750637275860993 | catsscareme2021 |      21 |  3.60 |          10 |  1.72 |                552 | 94.68 |
|            94324983 | FonsiLoaiza     |      12 |  2.07 |          19 |  3.28 |                548 | 94.65 |
|            14377605 | TheDemocrats    |       2 |  0.35 |          20 |  3.47 |                555 | 96.19 |
| 1450241520859156483 | geoscience16    |     161 | 27.90 |           4 |  0.69 |                412 | 71.40 |
| 1201670995435646976 | laurenboebert   |      17 |  2.99 |          22 |  3.87 |                530 | 93.15 |
| 1393726565809278976 | AdameMedia      |      13 |  2.29 |          22 |  3.88 |                532 | 93.83 |
| 1339166129110065152 | GBNEWS          |      24 |  4.27 |           8 |  1.42 |                530 | 94.31 |
|            81371986 | LozzaFox        |      29 |  5.20 |          19 |  3.41 |                510 | 91.40 |
|           611986351 | KimDotcom       |      28 |  5.02 |          32 |  5.73 |                498 | 89.25 |
|             5734902 | tagesschau      |      14 |  2.54 |          21 |  3.80 |                517 | 93.66 |
|            39692424 | AlertaNews24    |     105 | 19.13 |          16 |  2.91 |                428 | 77.96 |
|           109065990 | RealAlexJones   |      39 |  7.10 |          14 |  2.55 |                496 | 90.35 |
|           351491321 | wallstwolverine |      16 |  2.92 |          22 |  4.01 |                510 | 93.07 |
|              807095 | nytimes         |      19 |  3.49 |          19 |  3.49 |                507 | 93.03 |
| 1600964443122421769 | CNviolations    |       5 |  0.93 |          70 | 12.96 |                465 | 86.11 |
|            65045121 | owenjonesjourno |       9 |  1.67 |          21 |  3.90 |                509 | 94.43 |
|           371381075 | sandrousseau    |      29 |  5.40 |          44 |  8.19 |                464 | 86.41 |
|  896466491587080194 | greg_price11    |      17 |  3.18 |          20 |  3.74 |                498 | 93.08 |
