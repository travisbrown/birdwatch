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
|            44196397 | elonmusk        |      98 |  0.62 |        1823 | 11.44 |              14010 | 87.94 |
|           629698642 | BGatesIsaPyscho |     932 | 22.17 |          96 |  2.28 |               3176 | 75.55 |
| 1151913018936053760 | jacksonhinklle  |     342 |  8.19 |         210 |  5.03 |               3625 | 86.78 |
| 1349149096909668363 | POTUS           |      57 |  1.50 |         236 |  6.23 |               3496 | 92.27 |
|          3376321847 | iluminatibot    |     648 | 17.83 |          87 |  2.39 |               2900 | 79.78 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         102 |  2.83 |               3505 | 97.15 |
|              939091 | JoeBiden        |      15 |  0.44 |         301 |  8.77 |               3116 | 90.79 |
|           191871143 | VigilantFox     |      32 |  0.98 |          45 |  1.38 |               3177 | 97.63 |
| 1552795969959636992 | EndWokeness     |      66 |  2.03 |         132 |  4.06 |               3052 | 93.91 |
|          1291128494 | ShaykhSulaiman  |     254 |  7.88 |         154 |  4.78 |               2816 | 87.34 |
| 1446231057259433988 | DrLoupis        |     208 |  7.31 |         117 |  4.11 |               2519 | 88.57 |
| 1222773302441148416 | visegrad24      |     155 |  5.62 |         256 |  9.28 |               2349 | 85.11 |
|  953378142198161409 | choquei         |     379 | 13.85 |          67 |  2.45 |               2291 | 83.70 |
| 1319287761048723458 | MarioNawfal     |     185 |  6.81 |          88 |  3.24 |               2444 | 89.95 |
| 1326229737551912960 | libsoftiktok    |      32 |  1.30 |          74 |  3.00 |               2362 | 95.71 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         111 |  4.56 |               2316 | 95.11 |
|            16106584 | stillgray       |     283 | 11.86 |          95 |  3.98 |               2009 | 84.16 |
|          3096147528 | dom_lucre       |     280 | 12.56 |          88 |  3.95 |               1862 | 83.50 |
|            76766018 | Dexerto         |     222 | 11.09 |         167 |  8.34 |               1613 | 80.57 |
| 1366565625401909249 | WallStreetSilv  |     103 |  5.44 |          68 |  3.59 |               1724 | 90.98 |
| 1528943647185678336 | LeadingReport   |      54 |  2.96 |          34 |  1.87 |               1735 | 95.17 |
|  801203059359678464 | DrEliDavid      |      36 |  1.99 |         148 |  8.17 |               1627 | 89.84 |
|  805532293951606785 | MattWallace888  |     219 | 12.09 |          83 |  4.58 |               1509 | 83.32 |
|            80233893 | jakeshieldsajj  |      97 |  5.38 |          95 |  5.27 |               1612 | 89.36 |
| 1374968033265864706 | TaraBull808     |      87 |  4.97 |          34 |  1.94 |               1631 | 93.09 |
|          3260357396 | thehealthb0t    |     284 | 16.32 |          22 |  1.26 |               1434 | 82.41 |
| 1429400366819512323 | RadioGenoa      |      51 |  2.95 |          85 |  4.91 |               1594 | 92.14 |
| 1099579684981944320 | WallStreetApes  |      88 |  5.20 |          15 |  0.89 |               1590 | 93.92 |
|           537709549 | LauraLoomer     |     197 | 12.00 |          43 |  2.62 |               1401 | 85.37 |
|  855481986290524160 | historyinmemes  |     136 |  8.37 |         172 | 10.59 |               1316 | 81.03 |
|          2538322138 | PicturesFoIder  |     103 |  6.45 |         174 | 10.90 |               1320 | 82.65 |
| 1344356576786866176 | RepMTG          |      67 |  4.55 |          50 |  3.39 |               1356 | 92.06 |
|           312696314 | Partisangirl    |     101 |  7.13 |          62 |  4.38 |               1253 | 88.49 |
|            91416107 | OliLondonTV     |      62 |  4.42 |          80 |  5.70 |               1261 | 89.88 |
|  890061634181373952 | CollinRugg      |      44 |  3.14 |          61 |  4.35 |               1296 | 92.51 |
|           971820228 | CerfiaFR        |      64 |  4.58 |          85 |  6.08 |               1248 | 89.33 |
| 1389913567671975937 | cb_doge         |      35 |  2.55 |          79 |  5.75 |               1261 | 91.71 |
| 1486069441259397125 | harris_wins     |      17 |  1.26 |          54 |  4.00 |               1279 | 94.74 |
| 1355721251180961792 | GuntherEagleman |      45 |  3.39 |          31 |  2.33 |               1253 | 94.28 |
|           133663801 | BFMTV           |     124 |  9.37 |          81 |  6.12 |               1118 | 84.50 |
|            14281853 | Conservatives   |      72 |  5.45 |          42 |  3.18 |               1206 | 91.36 |
|            32867753 | silvano_trotta  |     318 | 24.35 |          13 |  1.00 |                975 | 74.66 |
|            15212187 | bennyjohnson    |      65 |  5.04 |          44 |  3.41 |               1181 | 91.55 |
|            49849732 | petrogustavo    |      33 |  2.57 |          36 |  2.80 |               1215 | 94.63 |
|            25073877 | realDonaldTrump |       5 |  0.39 |         105 |  8.27 |               1159 | 91.33 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1138 | 89.96 |
|            52660746 | Israel          |       6 |  0.48 |         116 |  9.21 |               1138 | 90.32 |
|           592730371 | JackPosobiec    |      73 |  5.88 |          35 |  2.82 |               1134 | 91.30 |
|            17469289 | nypost          |      93 |  7.52 |          51 |  4.12 |               1093 | 88.36 |
|  750683331260321792 | MAstronomers    |      71 |  5.92 |          66 |  5.50 |               1062 | 88.57 |
|  855483824351924224 | liz_churchill10 |     134 | 11.21 |          23 |  1.92 |               1038 | 86.86 |
|  807355480188141568 | DramaAlert      |     198 | 16.64 |          69 |  5.80 |                923 | 77.56 |
|            37501849 | Quakeprediction |      29 |  2.49 |           1 |  0.09 |               1136 | 97.43 |
|           133938408 | krassenstein    |      10 |  0.86 |          73 |  6.29 |               1077 | 92.84 |
|           452540168 | DailyLoud       |     294 | 25.37 |          72 |  6.21 |                793 | 68.42 |
| 1455903807389458436 | BladeoftheS     |     157 | 14.01 |          16 |  1.43 |                948 | 84.57 |
|           177101260 | Rainmaker1973   |      17 |  1.53 |         120 | 10.78 |                976 | 87.69 |
|            96684891 | kharaguchi      |     117 | 10.53 |           9 |  0.81 |                985 | 88.66 |
|           255471924 | mfa_russia      |     134 | 12.22 |          53 |  4.83 |                910 | 82.95 |
| 1335132884278108161 | stats_feed      |     109 | 10.11 |          55 |  5.10 |                914 | 84.79 |
|           132339474 | EdKrassen       |      20 |  1.93 |          48 |  4.62 |                970 | 93.45 |
|             7587032 | SkyNews         |      30 |  2.90 |          38 |  3.68 |                966 | 93.42 |
| 1121807798826930177 | MyLordBebo      |     111 | 10.84 |          30 |  2.93 |                883 | 86.23 |
|            16635277 | Acyn            |      33 |  3.23 |          66 |  6.45 |                924 | 90.32 |
|           428454304 | harryjsisson    |      13 |  1.28 |          33 |  3.24 |                971 | 95.48 |
|            69156861 | TheChiefNerd    |       6 |  0.61 |           9 |  0.91 |                973 | 98.48 |
|          2670726740 | LulaOficial     |       5 |  0.51 |          22 |  2.24 |                954 | 97.25 |
|           524869533 | QudsNen         |      58 |  5.94 |          31 |  3.18 |                887 | 90.88 |
|            90954365 | earthquakejapan |      88 |  9.05 |           0 |  0.00 |                884 | 90.95 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.46 |          23 |  2.37 |                895 | 92.17 |
| 1224185690713460736 | goddeketal      |      49 |  5.06 |          25 |  2.58 |                895 | 92.36 |
| 1486473049146904576 | InternetH0F     |     103 | 10.65 |         116 | 12.00 |                748 | 77.35 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.44 |          35 |  3.65 |                890 | 92.90 |
|            51241574 | AP              |      12 |  1.26 |          44 |  4.62 |                897 | 94.12 |
| 1471414739880189955 | kirawontmiss    |      46 |  4.85 |         148 | 15.61 |                754 | 79.54 |
|           706646642 | EmbassyofRussia |     106 | 11.23 |          33 |  3.50 |                805 | 85.28 |
|           848279118 | Kahlissee       |      37 |  3.94 |          39 |  4.15 |                864 | 91.91 |
|           138203134 | AOC             |      12 |  1.30 |          43 |  4.65 |                870 | 94.05 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|            22703645 | TuckerCarlson   |       1 |  0.11 |          87 |  9.46 |                832 | 90.43 |
|            37491797 | stkirsch        |      62 |  6.75 |          11 |  1.20 |                846 | 92.06 |
|           242827267 | PierrePoilievre |       0 |  0.00 |          25 |  2.77 |                878 | 97.23 |
| 1529763962094596097 | wideawake_media |      13 |  1.45 |          13 |  1.45 |                873 | 97.11 |
| 1043185714437992449 | catturd2        |      35 |  3.91 |          41 |  4.59 |                818 | 91.50 |
|           292929271 | charliekirk11   |      39 |  4.43 |          25 |  2.84 |                817 | 92.74 |
|  896550698543874049 | UTDTrey         |      76 |  8.67 |          63 |  7.18 |                738 | 84.15 |
|           130557513 | mehdirhasan     |      14 |  1.60 |          27 |  3.09 |                832 | 95.30 |
| 1430497892314218502 | Resist_05       |      44 |  5.05 |          24 |  2.75 |                804 | 92.20 |
|             7996082 | el_pais         |      36 |  4.14 |          28 |  3.22 |                806 | 92.64 |
| 1538230472299270144 | ayeejuju        |      73 |  8.48 |          94 | 10.92 |                694 | 80.60 |
|           374712154 | TRobinsonNewEra |      38 |  4.46 |          21 |  2.46 |                793 | 93.08 |
|            11347122 | GavinNewsom     |      10 |  1.18 |          22 |  2.60 |                814 | 96.22 |
|            62957739 | eduardomenoni   |     185 | 21.89 |           9 |  1.07 |                651 | 77.04 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
| 1288319695658135552 | realstewpeters  |     180 | 21.45 |          30 |  3.58 |                629 | 74.97 |
|  875856268056969216 | DC_Draino       |      16 |  1.91 |          17 |  2.03 |                804 | 96.06 |
| 1342174584594960384 | iamyesyouareno  |      51 |  6.19 |          47 |  5.70 |                726 | 88.11 |
|           978932870 | CapitanBitcoin  |      37 |  4.50 |          21 |  2.55 |                765 | 92.95 |
|            14128609 | felipeneto      |      18 |  2.20 |          28 |  3.42 |                773 | 94.38 |
|          2828212668 | AMAZlNGNATURE   |      56 |  6.85 |          97 | 11.86 |                665 | 81.30 |
| 1356434353623093249 | greg16676935420 |      19 |  2.32 |          94 | 11.49 |                705 | 86.19 |
| 1138458175663988738 | PopBase         |      48 |  5.89 |          60 |  7.36 |                707 | 86.75 |
| 1128337957289697281 | SprinterFamily  |     203 | 25.03 |          18 |  2.22 |                590 | 72.75 |
|  855300206086168576 | HumansNoContext |      86 | 10.62 |          67 |  8.27 |                657 | 81.11 |
| 1187524450809536513 | vitoquiles      |      38 |  4.71 |          22 |  2.73 |                746 | 92.56 |
|          3331429624 | Metropoles      |      68 |  8.45 |          21 |  2.61 |                716 | 88.94 |
| 1225234593789423616 | Megatron_ron    |     105 | 13.21 |          37 |  4.65 |                653 | 82.14 |
|           333357345 | Cobratate       |      29 |  3.65 |          77 |  9.70 |                688 | 86.65 |
|            14594813 | folha           |      29 |  3.66 |          22 |  2.77 |                742 | 93.57 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          15 |  1.89 |                778 | 98.11 |
|           109398997 | stopvaccinating |      92 | 11.65 |          10 |  1.27 |                688 | 87.09 |
| 1302674142630760450 | EverythingOOC   |      67 |  8.49 |          87 | 11.03 |                635 | 80.48 |
| 1562038858988064768 | TheFigen\_      |      96 | 12.28 |          75 |  9.59 |                611 | 78.13 |
|          1542228578 | JDVance         |       6 |  0.77 |          51 |  6.55 |                722 | 92.68 |
|  780460754910732288 | DiscussingFilm  |      31 |  3.98 |          83 | 10.65 |                665 | 85.37 |
|          1446465174 | akafacehots     |      82 | 10.54 |          23 |  2.96 |                673 | 86.50 |
|          1640929196 | mmpadellan      |      22 |  2.85 |          15 |  1.95 |                734 | 95.20 |
| 1593929531148144645 | stairwayto3dom  |      51 |  6.67 |          27 |  3.53 |                687 | 89.80 |
|            80820758 | JLMelenchon     |      50 |  6.59 |          46 |  6.06 |                663 | 87.35 |
| 1087757588622651397 | porqueTTarg     |     160 | 21.11 |          27 |  3.56 |                571 | 75.33 |
| 1080188052365029376 | acnewsitics     |       8 |  1.06 |          46 |  6.10 |                700 | 92.84 |
| 1492007194388279333 | LibertyCappy    |      27 |  3.60 |          52 |  6.94 |                670 | 89.45 |
|            46302096 | JoeyMannarinoUS |      46 |  6.20 |          20 |  2.70 |                676 | 91.11 |
|           463142998 | SuppressedNws   |      10 |  1.36 |          32 |  4.35 |                693 | 94.29 |
|  826065164504006657 | mtgreenee       |      31 |  4.22 |          31 |  4.22 |                672 | 91.55 |
| 1090084079964348419 | MrSinha\_       |      16 |  2.20 |           7 |  0.96 |                704 | 96.84 |
|          4429003533 | PopCrave        |      40 |  5.52 |          54 |  7.45 |                631 | 87.03 |
|            27000730 | Timcast         |      13 |  1.80 |          34 |  4.72 |                674 | 93.48 |
|           535707261 | eldiarioes      |       8 |  1.12 |          13 |  1.82 |                695 | 97.07 |
|            18576537 | IDF             |       0 |  0.00 |          71 |  9.94 |                643 | 90.06 |
|          1626294277 | spectatorindex  |       9 |  1.26 |          44 |  6.16 |                661 | 92.58 |
|  707231479047315456 | PeterSweden7    |      21 |  2.95 |           8 |  1.13 |                682 | 95.92 |
|             4239551 | amuse           |      21 |  2.96 |          21 |  2.96 |                668 | 94.08 |
|          4020276615 | JMilei          |       4 |  0.56 |          39 |  5.49 |                667 | 93.94 |
|           319774010 | kirinjisinken   |     384 | 54.39 |          11 |  1.56 |                311 | 44.05 |
|          1500129642 | MattWalshBlog   |      11 |  1.57 |          28 |  4.00 |                661 | 94.43 |
| 1062754443798532096 | MakisMD         |      20 |  2.86 |          10 |  1.43 |                670 | 95.71 |
|           330262748 | FabrizioRomano  |      55 |  7.90 |          82 | 11.78 |                559 | 80.32 |
| 1138842105856573445 | NoContextHumans |      59 |  8.49 |          81 | 11.65 |                555 | 79.86 |
| 1604139215406727170 | CensoredMen     |      34 |  4.90 |          46 |  6.63 |                614 | 88.47 |
|  959531564341317632 | AlertesInfos    |      31 |  4.47 |          46 |  6.64 |                616 | 88.89 |
|             1137701 | DavidSacks      |      21 |  3.08 |          52 |  7.62 |                609 | 89.30 |
|            39344374 | DonaldJTrumpJr  |      26 |  3.82 |          34 |  4.99 |                621 | 91.19 |
| 1179892477714718721 | gunsnrosesgirl3 |      21 |  3.10 |          80 | 11.80 |                577 | 85.10 |
|           288277167 | atrupar         |      16 |  2.37 |          25 |  3.71 |                633 | 93.92 |
|            14436030 | elmundoes       |      58 |  8.62 |          12 |  1.78 |                603 | 89.60 |
|           294293982 | Rothmus         |      41 |  6.10 |          35 |  5.21 |                596 | 88.69 |
|          2161051908 | AvivaKlompas    |      13 |  1.93 |          61 |  9.08 |                598 | 88.99 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.49 |          25 |  3.73 |                636 | 94.78 |
|           337808606 | RobertKennedyJr |      11 |  1.65 |          33 |  4.94 |                624 | 93.41 |
| 1625843518643085312 | creepydotorg    |      54 |  8.10 |          48 |  7.20 |                565 | 84.71 |
|           472412809 | f_philippot     |     105 | 15.84 |          13 |  1.96 |                545 | 82.20 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
| 1111976778065723393 | nocontextfooty  |      65 | 10.08 |          53 |  8.22 |                527 | 81.71 |
|            14260960 | JustinTrudeau   |       1 |  0.16 |          45 |  7.06 |                591 | 92.78 |
|            19069018 | jreichelt       |      17 |  2.67 |          18 |  2.83 |                602 | 94.51 |
|  918197046871502849 | siteptbr        |      76 | 11.99 |          18 |  2.84 |                540 | 85.17 |
| 1434450096557596680 | PolitlcsUK      |      17 |  2.68 |          40 |  6.31 |                577 | 91.01 |
|              759251 | CNN             |      16 |  2.53 |          16 |  2.53 |                600 | 94.94 |
| 1392864463204782082 | WarMonitors     |      18 |  2.85 |          28 |  4.43 |                586 | 92.72 |
| 1298372735383605249 | RonFilipkowski  |      18 |  2.86 |          31 |  4.93 |                580 | 92.21 |
|  818893114979061761 | JoJoFromJerz    |      10 |  1.59 |          17 |  2.71 |                600 | 95.69 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      50 |  7.99 |           6 |  0.96 |                570 | 91.05 |
| 1158115510606815232 | therealbuni     |      69 | 11.04 |          23 |  3.68 |                533 | 85.28 |
|  795188519115358208 | tweetsoku1      |     202 | 32.37 |           4 |  0.64 |                418 | 66.99 |
| 1221462414744596483 | RpsAgainstTrump |       8 |  1.28 |          64 | 10.27 |                551 | 88.44 |
| 1661674273122160641 | \_maakun\_\_2   |     121 | 19.45 |           0 |  0.00 |                501 | 80.55 |
|            19017675 | Nigel_Farage    |       5 |  0.81 |          24 |  3.87 |                591 | 95.32 |
| 1155845777039810560 | richimedhurst   |      17 |  2.75 |          22 |  3.56 |                579 | 93.69 |
|            62513246 | jk_rowling      |       0 |  0.00 |          53 |  8.67 |                558 | 91.33 |
|  930212715422998530 | Travis_4_Trump  |      28 |  4.58 |          16 |  2.62 |                567 | 92.80 |
| 1227799690579718144 | VivekGRamaswamy |      13 |  2.13 |          30 |  4.93 |                566 | 92.94 |
|            19923515 | abc_es          |      69 | 11.35 |          19 |  3.12 |                520 | 85.53 |
|           195853497 | EuropeInvasions |      37 |  6.09 |          38 |  6.25 |                533 | 87.66 |
|          4691437897 | darrengrimes\_  |      31 |  5.11 |          15 |  2.47 |                561 | 92.42 |
|            55329156 | RNCResearch     |      31 |  5.12 |          19 |  3.14 |                556 | 91.75 |
|            14173315 | NBCNews         |      11 |  1.85 |          17 |  2.85 |                568 | 95.30 |
| 1432287556129812484 | himuro398       |     118 | 19.93 |           7 |  1.18 |                467 | 78.89 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.37 |                577 | 97.63 |
| 1200616796295847936 | unusual_whales  |      74 | 12.54 |          23 |  3.90 |                493 | 83.56 |
|              742143 | BBCWorld        |      21 |  3.57 |          33 |  5.61 |                534 | 90.82 |
|           458008892 | EylonALevy      |       0 |  0.00 |          38 |  6.46 |                550 | 93.54 |
|            94324983 | FonsiLoaiza     |      12 |  2.04 |          19 |  3.24 |                556 | 94.72 |
| 1483750637275860993 | catsscareme2021 |      21 |  3.60 |          10 |  1.72 |                552 | 94.68 |
|            14377605 | TheDemocrats    |       2 |  0.35 |          20 |  3.46 |                556 | 96.19 |
| 1450241520859156483 | geoscience16    |     161 | 27.90 |           4 |  0.69 |                412 | 71.40 |
| 1393726565809278976 | AdameMedia      |      13 |  2.26 |          22 |  3.83 |                540 | 93.91 |
|           351491321 | wallstwolverine |      17 |  2.98 |          24 |  4.21 |                529 | 92.81 |
| 1201670995435646976 | laurenboebert   |      17 |  2.99 |          22 |  3.87 |                530 | 93.15 |
| 1339166129110065152 | GBNEWS          |      24 |  4.26 |           8 |  1.42 |                531 | 94.32 |
|            81371986 | LozzaFox        |      29 |  5.20 |          19 |  3.41 |                510 | 91.40 |
|           611986351 | KimDotcom       |      28 |  5.02 |          33 |  5.91 |                497 | 89.07 |
|            39692424 | AlertaNews24    |     105 | 18.92 |          16 |  2.88 |                434 | 78.20 |
|           109065990 | RealAlexJones   |      39 |  7.04 |          14 |  2.53 |                501 | 90.43 |
|             5734902 | tagesschau      |      14 |  2.54 |          21 |  3.80 |                517 | 93.66 |
| 1600964443122421769 | CNviolations    |       5 |  0.91 |          70 | 12.70 |                476 | 86.39 |
|              807095 | nytimes         |      19 |  3.49 |          19 |  3.49 |                507 | 93.03 |
|            65045121 | owenjonesjourno |       9 |  1.67 |          21 |  3.89 |                510 | 94.44 |
|           371381075 | sandrousseau    |      30 |  5.58 |          44 |  8.18 |                464 | 86.25 |
|  896466491587080194 | greg_price11    |      17 |  3.18 |          20 |  3.74 |                498 | 93.08 |
