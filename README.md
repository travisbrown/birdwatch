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
|            44196397 | elonmusk        |      95 |  0.63 |        1756 | 11.65 |              13228 | 87.72 |
| 1151913018936053760 | jacksonhinklle  |     335 |  8.18 |         205 |  5.00 |               3557 | 86.82 |
|           629698642 | BGatesIsaPyscho |     900 | 22.51 |          95 |  2.38 |               3004 | 75.12 |
| 1349149096909668363 | POTUS           |      57 |  1.51 |         234 |  6.22 |               3473 | 92.27 |
|          3376321847 | iluminatibot    |     620 | 17.59 |          87 |  2.47 |               2818 | 79.94 |
|              939091 | JoeBiden        |      15 |  0.44 |         290 |  8.57 |               3080 | 90.99 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |          93 |  2.79 |               3240 | 97.18 |
|          1291128494 | ShaykhSulaiman  |     253 |  7.96 |         152 |  4.78 |               2773 | 87.26 |
| 1552795969959636992 | EndWokeness     |      66 |  2.09 |         125 |  3.97 |               2961 | 93.94 |
|           191871143 | VigilantFox     |      32 |  1.02 |          45 |  1.43 |               3070 | 97.55 |
| 1446231057259433988 | DrLoupis        |     203 |  7.27 |         111 |  3.97 |               2480 | 88.76 |
|  953378142198161409 | choquei         |     366 | 13.56 |          67 |  2.48 |               2266 | 83.96 |
| 1222773302441148416 | visegrad24      |     152 |  5.70 |         253 |  9.48 |               2264 | 84.83 |
| 1319287761048723458 | MarioNawfal     |     184 |  7.03 |          82 |  3.13 |               2351 | 89.84 |
| 1326229737551912960 | libsoftiktok    |      32 |  1.37 |          71 |  3.05 |               2228 | 95.58 |
|            16106584 | stillgray       |     278 | 11.97 |          95 |  4.09 |               1950 | 83.94 |
|            30354991 | KamalaHarris    |       8 |  0.36 |          95 |  4.30 |               2104 | 95.33 |
|          3096147528 | dom_lucre       |     277 | 12.61 |          86 |  3.92 |               1833 | 83.47 |
|            76766018 | Dexerto         |     215 | 11.01 |         159 |  8.15 |               1578 | 80.84 |
| 1366565625401909249 | WallStreetSilv  |     101 |  5.40 |          65 |  3.47 |               1706 | 91.13 |
|            80233893 | jakeshieldsajj  |      97 |  5.43 |          94 |  5.26 |               1595 | 89.31 |
|  801203059359678464 | DrEliDavid      |      33 |  1.88 |         142 |  8.07 |               1584 | 90.05 |
| 1528943647185678336 | LeadingReport   |      53 |  3.03 |          29 |  1.66 |               1670 | 95.32 |
|  805532293951606785 | MattWallace888  |     209 | 12.21 |          80 |  4.67 |               1423 | 83.12 |
| 1429400366819512323 | RadioGenoa      |      50 |  2.93 |          84 |  4.92 |               1573 | 92.15 |
|          3260357396 | thehealthb0t    |     257 | 15.56 |          23 |  1.39 |               1372 | 83.05 |
| 1374968033265864706 | TaraBull808     |      74 |  4.49 |          32 |  1.94 |               1542 | 93.57 |
|           537709549 | LauraLoomer     |     196 | 12.14 |          42 |  2.60 |               1376 | 85.25 |
| 1099579684981944320 | WallStreetApes  |      84 |  5.21 |          14 |  0.87 |               1514 | 93.92 |
|  855481986290524160 | historyinmemes  |     133 |  8.41 |         165 | 10.44 |               1283 | 81.15 |
|          2538322138 | PicturesFoIder  |      99 |  6.43 |         165 | 10.71 |               1276 | 82.86 |
| 1344356576786866176 | RepMTG          |      66 |  4.51 |          50 |  3.42 |               1348 | 92.08 |
|           312696314 | Partisangirl    |     101 |  7.29 |          60 |  4.33 |               1224 | 88.38 |
|            91416107 | OliLondonTV     |      62 |  4.49 |          80 |  5.79 |               1239 | 89.72 |
|           971820228 | CerfiaFR        |      60 |  4.54 |          80 |  6.05 |               1182 | 89.41 |
|  890061634181373952 | CollinRugg      |      44 |  3.34 |          55 |  4.18 |               1217 | 92.48 |
|            14281853 | Conservatives   |      72 |  5.48 |          40 |  3.04 |               1203 | 91.48 |
| 1486069441259397125 | harris_wins     |      16 |  1.25 |          53 |  4.13 |               1214 | 94.62 |
| 1355721251180961792 | GuntherEagleman |      39 |  3.07 |          30 |  2.36 |               1203 | 94.58 |
| 1168968080690749441 | RishiSunak      |      58 |  4.61 |          66 |  5.25 |               1133 | 90.14 |
|           133663801 | BFMTV           |     120 |  9.62 |          72 |  5.77 |               1055 | 84.60 |
|            52660746 | Israel          |       6 |  0.48 |         114 |  9.19 |               1121 | 90.33 |
|            49849732 | petrogustavo    |      33 |  2.67 |          35 |  2.83 |               1167 | 94.49 |
|            32867753 | silvano_trotta  |     308 | 25.08 |          12 |  0.98 |                908 | 73.94 |
| 1389913567671975937 | cb_doge         |      33 |  2.70 |          75 |  6.13 |               1116 | 91.18 |
|            15212187 | bennyjohnson    |      64 |  5.28 |          40 |  3.30 |               1107 | 91.41 |
|            17469289 | nypost          |      91 |  7.58 |          50 |  4.16 |               1060 | 88.26 |
|           592730371 | JackPosobiec    |      73 |  6.09 |          35 |  2.92 |               1091 | 90.99 |
|  855483824351924224 | liz_churchill10 |     132 | 11.20 |          23 |  1.95 |               1024 | 86.85 |
|            37501849 | Quakeprediction |      29 |  2.51 |           1 |  0.09 |               1124 | 97.40 |
|           452540168 | DailyLoud       |     290 | 25.69 |          71 |  6.29 |                768 | 68.02 |
|  750683331260321792 | MAstronomers    |      68 |  6.08 |          60 |  5.37 |                990 | 88.55 |
|  807355480188141568 | DramaAlert      |     187 | 16.91 |          65 |  5.88 |                854 | 77.22 |
|           133938408 | krassenstein    |      10 |  0.91 |          69 |  6.26 |               1024 | 92.84 |
|           255471924 | mfa_russia      |     133 | 12.17 |          53 |  4.85 |                907 | 82.98 |
| 1455903807389458436 | BladeoftheS     |     154 | 14.14 |          16 |  1.47 |                919 | 84.39 |
|           177101260 | Rainmaker1973   |      17 |  1.58 |         117 | 10.86 |                943 | 87.56 |
|            25073877 | realDonaldTrump |       5 |  0.46 |          87 |  8.09 |                984 | 91.45 |
|            96684891 | kharaguchi      |     114 | 10.69 |           9 |  0.84 |                943 | 88.46 |
| 1335132884278108161 | stats_feed      |     106 | 10.14 |          54 |  5.17 |                885 | 84.69 |
|           132339474 | EdKrassen       |      20 |  2.01 |          47 |  4.72 |                929 | 93.27 |
|             7587032 | SkyNews         |      30 |  3.02 |          34 |  3.42 |                929 | 93.55 |
| 1121807798826930177 | MyLordBebo      |     106 | 10.76 |          30 |  3.05 |                849 | 86.19 |
|            69156861 | TheChiefNerd    |       5 |  0.51 |           9 |  0.92 |                962 | 98.57 |
|           428454304 | harryjsisson    |      13 |  1.34 |          31 |  3.20 |                926 | 95.46 |
|           524869533 | QudsNen         |      58 |  6.02 |          31 |  3.22 |                874 | 90.76 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.50 |          23 |  2.39 |                887 | 92.11 |
| 1224185690713460736 | goddeketal      |      48 |  4.99 |          24 |  2.50 |                889 | 92.51 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.48 |          33 |  3.48 |                883 | 93.05 |
|            90954365 | earthquakejapan |      87 |  9.22 |           0 |  0.00 |                857 | 90.78 |
|           706646642 | EmbassyofRussia |     106 | 11.23 |          33 |  3.50 |                805 | 85.28 |
|          2670726740 | LulaOficial     |       5 |  0.53 |          22 |  2.35 |                911 | 97.12 |
| 1486473049146904576 | InternetH0F     |      98 | 10.48 |         109 | 11.66 |                728 | 77.86 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|            16635277 | Acyn            |      33 |  3.58 |          59 |  6.41 |                829 | 90.01 |
|            51241574 | AP              |      12 |  1.30 |          44 |  4.78 |                865 | 93.92 |
|            22703645 | TuckerCarlson   |       1 |  0.11 |          86 |  9.36 |                832 | 90.53 |
|            37491797 | stkirsch        |      62 |  6.75 |          11 |  1.20 |                846 | 92.06 |
| 1471414739880189955 | kirawontmiss    |      45 |  4.92 |         142 | 15.54 |                727 | 79.54 |
|           848279118 | Kahlissee       |      37 |  4.07 |          39 |  4.29 |                833 | 91.64 |
|           138203134 | AOC             |      11 |  1.23 |          40 |  4.49 |                840 | 94.28 |
|           242827267 | PierrePoilievre |       0 |  0.00 |          23 |  2.66 |                842 | 97.34 |
| 1529763962094596097 | wideawake_media |      13 |  1.51 |          12 |  1.39 |                838 | 97.10 |
| 1430497892314218502 | Resist_05       |      44 |  5.12 |          24 |  2.79 |                792 | 92.09 |
| 1043185714437992449 | catturd2        |      33 |  3.85 |          40 |  4.67 |                784 | 91.48 |
| 1538230472299270144 | ayeejuju        |      73 |  8.61 |          94 | 11.08 |                681 | 80.31 |
|           130557513 | mehdirhasan     |      14 |  1.65 |          26 |  3.07 |                806 | 95.27 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
|            11347122 | GavinNewsom     |      10 |  1.20 |          22 |  2.63 |                804 | 96.17 |
|             7996082 | el_pais         |      36 |  4.33 |          27 |  3.25 |                769 | 92.43 |
|  896550698543874049 | UTDTrey         |      72 |  8.66 |          60 |  7.22 |                699 | 84.12 |
|           374712154 | TRobinsonNewEra |      38 |  4.61 |          20 |  2.42 |                767 | 92.97 |
| 1288319695658135552 | realstewpeters  |     179 | 21.78 |          29 |  3.53 |                614 | 74.70 |
|            14128609 | felipeneto      |      18 |  2.21 |          28 |  3.44 |                767 | 94.34 |
|  875856268056969216 | DC_Draino       |      11 |  1.37 |          17 |  2.12 |                774 | 96.51 |
| 1356434353623093249 | greg16676935420 |      19 |  2.37 |          92 | 11.47 |                691 | 86.16 |
| 1128337957289697281 | SprinterFamily  |     201 | 25.09 |          19 |  2.37 |                581 | 72.53 |
|           292929271 | charliekirk11   |      38 |  4.78 |          23 |  2.89 |                734 | 92.33 |
| 1342174584594960384 | iamyesyouareno  |      49 |  6.16 |          46 |  5.79 |                700 | 88.05 |
|          2828212668 | AMAZlNGNATURE   |      54 |  6.80 |          93 | 11.71 |                647 | 81.49 |
|           978932870 | CapitanBitcoin  |      36 |  4.58 |          21 |  2.67 |                729 | 92.75 |
|          3331429624 | Metropoles      |      64 |  8.14 |          21 |  2.67 |                701 | 89.19 |
| 1302674142630760450 | EverythingOOC   |      66 |  8.43 |          87 | 11.11 |                630 | 80.46 |
|           109398997 | stopvaccinating |      90 | 11.51 |          10 |  1.28 |                682 | 87.21 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          15 |  1.92 |                765 | 98.08 |
| 1138458175663988738 | PopBase         |      46 |  5.91 |          58 |  7.46 |                674 | 86.63 |
|            14594813 | folha           |      29 |  3.75 |          21 |  2.72 |                723 | 93.53 |
|            62957739 | eduardomenoni   |     162 | 20.98 |           8 |  1.04 |                602 | 77.98 |
| 1225234593789423616 | Megatron_ron    |     105 | 13.74 |          35 |  4.58 |                624 | 81.68 |
| 1187524450809536513 | vitoquiles      |      38 |  4.99 |          22 |  2.89 |                701 | 92.12 |
|           333357345 | Cobratate       |      28 |  3.70 |          75 |  9.92 |                653 | 86.38 |
|  780460754910732288 | DiscussingFilm  |      29 |  3.85 |          80 | 10.61 |                645 | 85.54 |
| 1593929531148144645 | stairwayto3dom  |      49 |  6.50 |          27 |  3.58 |                678 | 89.92 |
| 1562038858988064768 | TheFigen\_      |      93 | 12.35 |          74 |  9.83 |                586 | 77.82 |
|          1640929196 | mmpadellan      |      22 |  2.94 |          15 |  2.01 |                711 | 95.05 |
|          1446465174 | akafacehots     |      76 | 10.22 |          23 |  3.09 |                645 | 86.69 |
| 1492007194388279333 | LibertyCappy    |      26 |  3.50 |          52 |  7.01 |                664 | 89.49 |
|            80820758 | JLMelenchon     |      49 |  6.65 |          43 |  5.83 |                645 | 87.52 |
| 1087757588622651397 | porqueTTarg     |     148 | 20.44 |          26 |  3.59 |                550 | 75.97 |
|            46302096 | JoeyMannarinoUS |      42 |  5.87 |          19 |  2.65 |                655 | 91.48 |
|  826065164504006657 | mtgreenee       |      30 |  4.21 |          30 |  4.21 |                653 | 91.58 |
|            18576537 | IDF             |       0 |  0.00 |          71 | 10.03 |                637 | 89.97 |
|          1542228578 | JDVance         |       6 |  0.85 |          43 |  6.10 |                656 | 93.05 |
|            27000730 | Timcast         |      13 |  1.85 |          34 |  4.83 |                657 | 93.32 |
| 1090084079964348419 | MrSinha\_       |      16 |  2.29 |           7 |  1.00 |                677 | 96.71 |
| 1062754443798532096 | MakisMD         |      20 |  2.86 |          10 |  1.43 |                669 | 95.71 |
| 1080188052365029376 | acnewsitics     |       7 |  1.00 |          41 |  5.87 |                651 | 93.13 |
|           463142998 | SuppressedNws   |      10 |  1.45 |          31 |  4.48 |                651 | 94.08 |
| 1604139215406727170 | CensoredMen     |      34 |  4.93 |          46 |  6.67 |                610 | 88.41 |
|          1626294277 | spectatorindex  |       8 |  1.16 |          42 |  6.10 |                639 | 92.74 |
|           319774010 | kirinjisinken   |     378 | 55.43 |          11 |  1.61 |                293 | 42.96 |
|             4239551 | amuse           |      18 |  2.64 |          21 |  3.08 |                642 | 94.27 |
| 1138842105856573445 | NoContextHumans |      57 |  8.42 |          77 | 11.37 |                543 | 80.21 |
|          4020276615 | JMilei          |       4 |  0.59 |          38 |  5.64 |                632 | 93.77 |
|  707231479047315456 | PeterSweden7    |      21 |  3.12 |           7 |  1.04 |                645 | 95.84 |
| 1179892477714718721 | gunsnrosesgirl3 |      21 |  3.12 |          79 | 11.76 |                572 | 85.12 |
|          4429003533 | PopCrave        |      38 |  5.67 |          49 |  7.31 |                583 | 87.01 |
|  959531564341317632 | AlertesInfos    |      29 |  4.35 |          40 |  6.01 |                597 | 89.64 |
|            39344374 | DonaldJTrumpJr  |      26 |  3.94 |          34 |  5.15 |                600 | 90.91 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.52 |          25 |  3.79 |                625 | 94.70 |
|          2161051908 | AvivaKlompas    |      13 |  1.98 |          59 |  8.98 |                585 | 89.04 |
|             1137701 | DavidSacks      |      20 |  3.05 |          48 |  7.32 |                588 | 89.63 |
|           535707261 | eldiarioes      |       8 |  1.22 |          13 |  1.98 |                635 | 96.80 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
|          1500129642 | MattWalshBlog   |      10 |  1.55 |          27 |  4.17 |                610 | 94.28 |
|           472412809 | f_philippot     |     103 | 15.99 |          12 |  1.86 |                529 | 82.14 |
|           294293982 | Rothmus         |      38 |  5.95 |          35 |  5.48 |                566 | 88.58 |
|           330262748 | FabrizioRomano  |      52 |  8.15 |          73 | 11.44 |                513 | 80.41 |
|           337808606 | RobertKennedyJr |      11 |  1.72 |          30 |  4.70 |                597 | 93.57 |
| 1625843518643085312 | creepydotorg    |      53 |  8.31 |          45 |  7.05 |                540 | 84.64 |
|            19069018 | jreichelt       |      17 |  2.71 |          18 |  2.87 |                593 | 94.43 |
| 1111976778065723393 | nocontextfooty  |      61 |  9.74 |          52 |  8.31 |                513 | 81.95 |
|              759251 | CNN             |      16 |  2.56 |          15 |  2.40 |                594 | 95.04 |
|            14436030 | elmundoes       |      57 |  9.13 |          11 |  1.76 |                556 | 89.10 |
|  855300206086168576 | HumansNoContext |      40 |  6.45 |          66 | 10.65 |                514 | 82.90 |
| 1392864463204782082 | WarMonitors     |      17 |  2.75 |          28 |  4.53 |                573 | 92.72 |
|            14260960 | JustinTrudeau   |       1 |  0.16 |          45 |  7.35 |                566 | 92.48 |
| 1155845777039810560 | richimedhurst   |      17 |  2.78 |          22 |  3.60 |                572 | 93.62 |
|            55329156 | RNCResearch     |      31 |  5.12 |          19 |  3.14 |                556 | 91.75 |
|  930212715422998530 | Travis_4_Trump  |      28 |  4.62 |          16 |  2.64 |                562 | 92.74 |
|            62513246 | jk_rowling      |       0 |  0.00 |          50 |  8.31 |                552 | 91.69 |
| 1227799690579718144 | VivekGRamaswamy |      13 |  2.17 |          29 |  4.83 |                558 | 93.00 |
|            19923515 | abc_es          |      68 | 11.35 |          18 |  3.01 |                513 | 85.64 |
|           288277167 | atrupar         |      15 |  2.51 |          17 |  2.84 |                566 | 94.65 |
| 1434450096557596680 | PolitlcsUK      |      16 |  2.68 |          41 |  6.86 |                541 | 90.47 |
| 1661674273122160641 | \_maakun\_\_2   |     117 | 19.57 |           0 |  0.00 |                481 | 80.43 |
|            19017675 | Nigel_Farage    |       5 |  0.84 |          22 |  3.71 |                566 | 95.45 |
| 1158115510606815232 | therealbuni     |      65 | 10.98 |          22 |  3.72 |                505 | 85.30 |
|  918197046871502849 | siteptbr        |      71 | 12.05 |          16 |  2.72 |                502 | 85.23 |
|          4691437897 | darrengrimes\_  |      31 |  5.27 |          15 |  2.55 |                542 | 92.18 |
|  795188519115358208 | tweetsoku1      |     193 | 32.82 |           4 |  0.68 |                391 | 66.50 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.39 |                573 | 97.61 |
|            14173315 | NBCNews         |      11 |  1.88 |          17 |  2.91 |                557 | 95.21 |
| 1298372735383605249 | RonFilipkowski  |      15 |  2.57 |          30 |  5.14 |                539 | 92.29 |
|           195853497 | EuropeInvasions |      32 |  5.50 |          37 |  6.36 |                513 | 88.14 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      46 |  7.93 |           5 |  0.86 |                529 | 91.21 |
|           458008892 | EylonALevy      |       0 |  0.00 |          38 |  6.56 |                541 | 93.44 |
| 1483750637275860993 | catsscareme2021 |      21 |  3.64 |          10 |  1.73 |                546 | 94.63 |
|  818893114979061761 | JoJoFromJerz    |      10 |  1.74 |          16 |  2.79 |                548 | 95.47 |
| 1221462414744596483 | RpsAgainstTrump |       8 |  1.39 |          58 | 10.10 |                508 | 88.50 |
|            14377605 | TheDemocrats    |       2 |  0.35 |          19 |  3.35 |                547 | 96.30 |
| 1201670995435646976 | laurenboebert   |      17 |  3.00 |          22 |  3.88 |                528 | 93.12 |
| 1200616796295847936 | unusual_whales  |      73 | 12.99 |          21 |  3.74 |                468 | 83.27 |
| 1450241520859156483 | geoscience16    |     160 | 28.47 |           4 |  0.71 |                398 | 70.82 |
|              742143 | BBCWorld        |      20 |  3.60 |          32 |  5.76 |                504 | 90.65 |
|           611986351 | KimDotcom       |      27 |  4.86 |          32 |  5.76 |                497 | 89.39 |
| 1339166129110065152 | GBNEWS          |      23 |  4.14 |           7 |  1.26 |                525 | 94.59 |
| 1432287556129812484 | himuro398       |     113 | 20.47 |           6 |  1.09 |                433 | 78.44 |
|             5734902 | tagesschau      |      14 |  2.55 |          22 |  4.01 |                512 | 93.43 |
|            81371986 | LozzaFox        |      29 |  5.29 |          19 |  3.47 |                500 | 91.24 |
|            39692424 | AlertaNews24    |     104 | 19.29 |          16 |  2.97 |                419 | 77.74 |
|              807095 | nytimes         |      19 |  3.54 |          19 |  3.54 |                499 | 92.92 |
|           109065990 | RealAlexJones   |      39 |  7.30 |          13 |  2.43 |                482 | 90.26 |
|            65045121 | owenjonesjourno |       9 |  1.69 |          21 |  3.94 |                503 | 94.37 |
|            94324983 | FonsiLoaiza     |      10 |  1.89 |          16 |  3.03 |                502 | 95.08 |
| 1393726565809278976 | AdameMedia      |      12 |  2.27 |          21 |  3.98 |                495 | 93.75 |
|           371381075 | sandrousseau    |      28 |  5.38 |          43 |  8.27 |                449 | 86.35 |
| 1492677599390322689 | weirddalle      |      16 |  3.10 |          64 | 12.40 |                436 | 84.50 |
|  896466491587080194 | greg_price11    |      17 |  3.31 |          20 |  3.89 |                477 | 92.80 |
|            36749572 | Alphafox78      |      96 | 18.71 |          33 |  6.43 |                384 | 74.85 |
