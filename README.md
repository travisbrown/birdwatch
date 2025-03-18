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
|            44196397 | elonmusk        |     130 |  0.61 |        2352 | 11.03 |              18835 |  88.36 |
|           629698642 | BGatesIsaPyscho |    1218 | 22.08 |         142 |  2.57 |               4156 |  75.34 |
|          3376321847 | iluminatibot    |     871 | 19.25 |         114 |  2.52 |               3540 |  78.23 |
| 1151913018936053760 | jacksonhinklle  |     363 |  8.04 |         235 |  5.20 |               3919 |  86.76 |
| 1552795969959636992 | EndWokeness     |      73 |  1.82 |         171 |  4.26 |               3774 |  93.93 |
| 1349149096909668363 | POTUS46Archive  |      60 |  1.51 |         253 |  6.36 |               3664 |  92.13 |
|           191871143 | VigilantFox     |      35 |  0.96 |          53 |  1.45 |               3557 |  97.59 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         103 |  2.85 |               3507 |  97.12 |
| 1319287761048723458 | MarioNawfal     |     202 |  5.66 |         113 |  3.16 |               3257 |  91.18 |
| 1326229737551912960 | libsoftiktok    |      54 |  1.52 |         121 |  3.40 |               3380 |  95.08 |
|              939091 | JoeBiden        |      16 |  0.46 |         304 |  8.70 |               3173 |  90.84 |
|          1291128494 | ShaykhSulaiman  |     265 |  7.67 |         159 |  4.60 |               3029 |  87.72 |
| 1222773302441148416 | visegrad24      |     169 |  5.03 |         299 |  8.90 |               2891 |  86.07 |
|  953378142198161409 | choquei         |     476 | 15.18 |          74 |  2.36 |               2585 |  82.46 |
| 1446231057259433988 | DrLoupis        |     216 |  7.22 |         125 |  4.18 |               2652 |  88.61 |
|            16106584 | stillgray       |     317 | 11.25 |         114 |  4.04 |               2388 |  84.71 |
|          3260357396 | thehealthb0t    |     506 | 18.47 |          41 |  1.50 |               2193 |  80.04 |
|  896550698543874049 | UTDTrey         |     295 | 10.81 |          77 |  2.82 |               2357 |  86.37 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         111 |  4.56 |               2317 |  95.11 |
| 1099579684981944320 | WallStreetApes  |     121 |  5.01 |          29 |  1.20 |               2265 |  93.79 |
|          3096147528 | dom_lucre       |     297 | 12.42 |          95 |  3.97 |               2000 |  83.61 |
|            76766018 | Dexerto         |     244 | 10.61 |         186 |  8.09 |               1870 |  81.30 |
| 1429400366819512323 | RadioGenoa      |      61 |  2.70 |         124 |  5.49 |               2075 |  91.81 |
| 1374968033265864706 | TaraBull808     |     128 |  5.87 |          45 |  2.06 |               2007 |  92.06 |
| 1528943647185678336 | LeadingReport   |      57 |  2.64 |          42 |  1.94 |               2064 |  95.42 |
| 1366565625401909249 | WallStreetMav   |     106 |  4.97 |          78 |  3.66 |               1947 |  91.37 |
|  805532293951606785 | MattWallace888  |     244 | 11.69 |          91 |  4.36 |               1753 |  83.96 |
|  801203059359678464 | DrEliDavid      |      42 |  2.14 |         156 |  7.93 |               1769 |  89.93 |
|            80233893 | jakeshieldsajj  |     108 |  5.60 |         109 |  5.65 |               1713 |  88.76 |
|           537709549 | LauraLoomer     |     219 | 11.91 |          51 |  2.77 |               1569 |  85.32 |
|           971820228 | CerfiaFR        |      87 |  4.75 |         114 |  6.23 |               1630 |  89.02 |
|          2538322138 | PicturesFoIder  |     124 |  6.83 |         194 | 10.69 |               1497 |  82.48 |
|            49849732 | petrogustavo    |      42 |  2.33 |          48 |  2.67 |               1709 |  95.00 |
|            32867753 | silvano_trotta  |     390 | 22.22 |          15 |  0.85 |               1350 |  76.92 |
|            25073877 | realDonaldTrump |       5 |  0.29 |         181 | 10.38 |               1557 |  89.33 |
|  855481986290524160 | historyinmemes  |     149 |  8.71 |         178 | 10.41 |               1383 |  80.88 |
| 1389913567671975937 | cb_doge         |      38 |  2.23 |          97 |  5.70 |               1567 |  92.07 |
| 1355721251180961792 | GuntherEagleman |      53 |  3.17 |          41 |  2.45 |               1579 |  94.38 |
|           312696314 | Partisangirl    |     132 |  7.92 |          75 |  4.50 |               1460 |  87.58 |
|            15212187 | bennyjohnson    |      86 |  5.20 |          61 |  3.69 |               1507 |  91.11 |
|           133663801 | BFMTV           |     151 |  9.47 |          95 |  5.96 |               1349 |  84.58 |
|  890061634181373952 | CollinRugg      |      51 |  3.20 |          70 |  4.39 |               1473 |  92.41 |
| 1344356576786866176 | RepMTG          |      71 |  4.49 |          56 |  3.54 |               1453 |  91.96 |
|  855483824351924224 | liz_churchill10 |     158 | 10.12 |          30 |  1.92 |               1374 |  87.96 |
|           592730371 | JackPosobiec    |      90 |  5.88 |          45 |  2.94 |               1396 |  91.18 |
|  807355480188141568 | DramaAlert      |     244 | 16.04 |          92 |  6.05 |               1185 |  77.91 |
|            17469289 | nypost          |     112 |  7.43 |          66 |  4.38 |               1330 |  88.20 |
|           133938408 | krassenstein    |      15 |  1.00 |          99 |  6.58 |               1390 |  92.42 |
| 1486069441259397125 | DemocraticWins  |      18 |  1.21 |          56 |  3.77 |               1413 |  95.02 |
|            91416107 | OliLondonTV     |      65 |  4.45 |          85 |  5.81 |               1312 |  89.74 |
|  750683331260321792 | MAstronomers    |      71 |  5.00 |          84 |  5.92 |               1264 |  89.08 |
|           177101260 | Rainmaker1973   |      24 |  1.71 |         156 | 11.10 |               1225 |  87.19 |
|            16635277 | Acyn            |      50 |  3.59 |         107 |  7.69 |               1235 |  88.72 |
| 1455903807389458436 | BladeoftheS     |     168 | 12.17 |          25 |  1.81 |               1188 |  86.02 |
|            14281853 | Conservatives   |      72 |  5.31 |          42 |  3.10 |               1242 |  91.59 |
|           428454304 | harryjsisson    |      16 |  1.19 |          42 |  3.13 |               1285 |  95.68 |
|            62957739 | eduardomenoni   |     323 | 24.07 |          17 |  1.27 |               1002 |  74.66 |
|            14594813 | folha           |      46 |  3.45 |          36 |  2.70 |               1251 |  93.85 |
|            96684891 | kharaguchi      |     131 | 10.07 |           9 |  0.69 |               1161 |  89.24 |
|            52660746 | Israel          |       6 |  0.46 |         122 |  9.38 |               1172 |  90.15 |
|           452540168 | DailyLoud       |     318 | 25.08 |          77 |  6.07 |                873 |  68.85 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1139 |  89.97 |
|           132339474 | EdKrassen       |      23 |  1.87 |          62 |  5.05 |               1143 |  93.08 |
| 1121807798826930177 | MyLordBebo      |     137 | 11.18 |          34 |  2.78 |               1054 |  86.04 |
|             7587032 | SkyNews         |      38 |  3.12 |          48 |  3.94 |               1132 |  92.94 |
|           292929271 | charliekirk11   |      46 |  3.78 |          40 |  3.28 |               1132 |  92.94 |
| 1335132884278108161 | stats_feed      |     117 |  9.62 |          65 |  5.35 |               1034 |  85.03 |
|            37501849 | Quakeprediction |      41 |  3.38 |           2 |  0.16 |               1171 |  96.46 |
| 1486473049146904576 | InternetH0F     |     127 | 10.50 |         136 | 11.24 |                947 |  78.26 |
|            69156861 | TheChiefNerd    |       9 |  0.76 |          14 |  1.18 |               1165 |  98.06 |
| 1224185690713460736 | goddeketal      |      59 |  5.03 |          34 |  2.90 |               1080 |  92.07 |
|            51241574 | AP              |      19 |  1.62 |          53 |  4.52 |               1100 |  93.86 |
|          2670726740 | LulaOficial     |       5 |  0.43 |          25 |  2.15 |               1131 |  97.42 |
| 1138458175663988738 | PopBase         |      56 |  4.83 |          93 |  8.02 |               1010 |  87.14 |
|          1542228578 | JDVance         |       6 |  0.52 |          95 |  8.28 |               1047 |  91.20 |
| 1529763962094596097 | wideawake_media |      21 |  1.83 |          15 |  1.31 |               1112 |  96.86 |
| 1043185714437992449 | catturd2        |      51 |  4.49 |          51 |  4.49 |               1034 |  91.02 |
| 1471414739880189955 | kirawontmiss    |      54 |  4.77 |         168 | 14.85 |                909 |  80.37 |
|           255471924 | mfa_russia      |     137 | 12.13 |          53 |  4.69 |                939 |  83.17 |
|           138203134 | AOC             |      12 |  1.07 |          57 |  5.07 |               1055 |  93.86 |
|          3331429624 | Metropoles      |      91 |  8.20 |          27 |  2.43 |                992 |  89.37 |
|           423692278 | AkademiksTV     |      49 |  4.54 |          14 |  1.30 |               1016 |  94.16 |
|             4239551 | amuse           |      32 |  3.00 |          37 |  3.46 |                999 |  93.54 |
|           333357345 | Cobratate       |      57 |  5.34 |          90 |  8.43 |                920 |  86.22 |
|           242827267 | PierrePoilievre |       1 |  0.09 |          33 |  3.13 |               1019 |  96.77 |
|             7996082 | el_pais         |      41 |  3.97 |          30 |  2.91 |                961 |  93.12 |
|          2828212668 | AMAZlNGNATURE   |      73 |  7.09 |         128 | 12.43 |                829 |  80.49 |
| 1577761560394665984 | DiedSuddenly\_  |      57 |  5.54 |          23 |  2.24 |                948 |  92.22 |
| 1600964443122421769 | CNviolations    |      15 |  1.46 |         104 | 10.12 |                909 |  88.42 |
| 1356434353623093249 | greg16676935420 |      20 |  1.96 |         113 | 11.05 |                890 |  87.00 |
| 1342174584594960384 | iamyesyouareno  |      54 |  5.28 |          60 |  5.87 |                908 |  88.85 |
|           288277167 | atrupar         |      28 |  2.74 |          53 |  5.19 |                940 |  92.07 |
|           848279118 | Kahlissee       |      37 |  3.63 |          41 |  4.03 |                940 |  92.34 |
|           524869533 | QudsNen         |      58 |  5.70 |          31 |  3.05 |                928 |  91.25 |
| 1538230472299270144 | ayeejuju        |      81 |  7.96 |         113 | 11.11 |                823 |  80.92 |
| 1080188052365029376 | acnewsitics     |       9 |  0.89 |          63 |  6.26 |                935 |  92.85 |
|            90954365 | earthquakejapan |      89 |  8.86 |           0 |  0.00 |                915 |  91.14 |
| 1087757588622651397 | porqueTTarg     |     213 | 21.28 |          36 |  3.60 |                752 |  75.12 |
|  875856268056969216 | DC_Draino       |      25 |  2.50 |          22 |  2.20 |                953 |  95.30 |
|            22703645 | TuckerCarlson   |       1 |  0.10 |          93 |  9.34 |                902 |  90.56 |
|  780460754910732288 | DiscussingFilm  |      37 |  3.72 |         108 | 10.85 |                850 |  85.43 |
|  855300206086168576 | HumansNoContext |     102 | 10.29 |          82 |  8.27 |                807 |  81.43 |
|  795188519115358208 | tweetsoku1      |     294 | 29.76 |           7 |  0.71 |                687 |  69.53 |
| 1323730225067339784 | WhiteHouse46    |      33 |  3.35 |          35 |  3.56 |                916 |  93.09 |
| 1111976778065723393 | nocontextfooty  |     111 | 11.30 |          61 |  6.21 |                810 |  82.48 |
|           142393421 | GloboNews       |      25 |  2.56 |          18 |  1.85 |                932 |  95.59 |
|           374712154 | TRobinsonNewEra |      40 |  4.11 |          23 |  2.36 |                910 |  93.53 |
|           130557513 | mehdirhasan     |      14 |  1.44 |          31 |  3.19 |                927 |  95.37 |
| 1288319695658135552 | realstewpeters  |     190 | 19.77 |          37 |  3.85 |                734 |  76.38 |
| 1430497892314218502 | Resist_05       |      47 |  4.91 |          30 |  3.13 |                881 |  91.96 |
|            80820758 | JLMelenchon     |      61 |  6.41 |          67 |  7.05 |                823 |  86.54 |
|           706646642 | EmbassyofRussia |     106 | 11.19 |          33 |  3.48 |                808 |  85.32 |
|  959531564341317632 | AlertesInfos    |      39 |  4.15 |          66 |  7.02 |                835 |  88.83 |
|            37491797 | stkirsch        |      62 |  6.65 |          12 |  1.29 |                858 |  92.06 |
|           463142998 | SuppressedNws   |      14 |  1.51 |          37 |  3.99 |                876 |  94.50 |
|  918197046871502849 | siteptbr        |     105 | 11.33 |          28 |  3.02 |                794 |  85.65 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.89 |                890 |  96.11 |
| 1090084079964348419 | MrSinha\_       |      22 |  2.38 |          10 |  1.08 |                893 |  96.54 |
|  707231479047315456 | PeterSweden7    |      25 |  2.71 |          17 |  1.84 |                881 |  95.45 |
|            11347122 | GavinNewsom     |      10 |  1.08 |          23 |  2.49 |                889 |  96.42 |
| 1562038858988064768 | TheFigen\_      |     111 | 12.13 |          90 |  9.84 |                714 |  78.03 |
|           978932870 | CapitanBitcoin  |      39 |  4.27 |          24 |  2.63 |                850 |  93.10 |
|           294293982 | Rothmus         |      55 |  6.05 |          45 |  4.95 |                809 |  89.00 |
|  885786515640655873 | CR7Brasil       |     462 | 50.94 |          14 |  1.54 |                431 |  47.52 |
| 1393726565809278976 | AdameMedia      |      22 |  2.43 |          41 |  4.54 |                841 |  93.03 |
| 1128337957289697281 | SprinterObserve |     220 | 24.36 |          21 |  2.33 |                662 |  73.31 |
| 1432287556129812484 | himuro398       |     181 | 20.07 |          13 |  1.44 |                708 |  78.49 |
| 1187524450809536513 | vitoquiles      |      40 |  4.44 |          27 |  3.00 |                834 |  92.56 |
|            14128609 | felipeneto      |      18 |  2.00 |          31 |  3.44 |                851 |  94.56 |
|          4429003533 | PopCrave        |      51 |  5.67 |          72 |  8.01 |                776 |  86.32 |
|          1640929196 | mmpadellan      |      27 |  3.01 |          20 |  2.23 |                850 |  94.76 |
| 1225234593789423616 | Megatron_ron    |     114 | 12.78 |          43 |  4.82 |                735 |  82.40 |
|          1446465174 | akafaceUS       |      89 |  9.99 |          25 |  2.81 |                777 |  87.21 |
|           109398997 | stopvaccinating |      99 | 11.15 |          11 |  1.24 |                778 |  87.61 |
|           319774010 | kirinjisinken   |     473 | 53.81 |          11 |  1.25 |                395 |  44.94 |
|           109065990 | RealAlexJones   |      56 |  6.40 |          27 |  3.09 |                792 |  90.51 |
| 1492007194388279333 | LibertyCappy    |      30 |  3.45 |          62 |  7.13 |                778 |  89.43 |
| 1200616796295847936 | unusual_whales  |     100 | 11.53 |          36 |  4.15 |                731 |  84.31 |
|  818893114979061761 | JoJoFromJerz    |      12 |  1.39 |          33 |  3.82 |                818 |  94.79 |
| 1179892477714718721 | gunsnrosesgirl3 |      27 |  3.13 |         103 | 11.94 |                733 |  84.94 |
|          4020276615 | JMilei          |       4 |  0.47 |          62 |  7.24 |                790 |  92.29 |
| 1593929531148144645 | stairwayto3dom  |      58 |  6.80 |          31 |  3.63 |                764 |  89.57 |
| 1221462414744596483 | RpsAgainstTrump |       9 |  1.06 |          86 | 10.09 |                757 |  88.85 |
|           330262748 | FabrizioRomano  |      68 |  8.04 |         104 | 12.29 |                674 |  79.67 |
| 1431774993419956224 | jhmdrei         |     228 | 27.11 |           3 |  0.36 |                610 |  72.53 |
|  935142655213703168 | zoo_bear        |       1 |  0.12 |          14 |  1.68 |                817 |  98.20 |
|           535707261 | eldiarioes      |      18 |  2.17 |          15 |  1.81 |                798 |  96.03 |
|            14260960 | JustinTrudeau   |       1 |  0.12 |          65 |  7.87 |                760 |  92.01 |
|            19017675 | Nigel_Farage    |       7 |  0.85 |          35 |  4.25 |                782 |  94.90 |
|            46302096 | JoeyMannarinoUS |      60 |  7.30 |          24 |  2.92 |                738 |  89.78 |
| 1514045582968598530 | CFC_Janty       |      66 |  8.03 |          32 |  3.89 |                724 |  88.08 |
|            39692424 | AlertaNews24    |     138 | 16.89 |          26 |  3.18 |                653 |  79.93 |
| 1302674142630760450 | EverythingOOC   |      69 |  8.49 |          87 | 10.70 |                657 |  80.81 |
| 1686901686185721857 | TrumpDailyPosts |      11 |  1.35 |          35 |  4.31 |                766 |  94.33 |
|            27000730 | Timcast         |      15 |  1.85 |          41 |  5.06 |                755 |  93.09 |
|            39344374 | DonaldJTrumpJr  |      31 |  3.88 |          42 |  5.26 |                725 |  90.85 |
|  826065164504006657 | mtgreenee       |      37 |  4.68 |          38 |  4.81 |                715 |  90.51 |
|          1500129642 | MattWalshBlog   |      11 |  1.39 |          29 |  3.68 |                749 |  94.93 |
| 1157689921349521410 | esjesjesj       |      15 |  1.91 |          53 |  6.75 |                717 |  91.34 |
| 1625843518643085312 | creepydotorg    |      58 |  7.42 |          62 |  7.93 |                662 |  84.65 |
|          1626294277 | spectatorindex  |      12 |  1.55 |          50 |  6.45 |                713 |  92.00 |
|            14436030 | elmundoes       |      62 |  8.01 |          13 |  1.68 |                699 |  90.31 |
| 1138842105856573445 | NoContextHumans |      71 |  9.20 |          85 | 11.01 |                616 |  79.79 |
| 1849500209488752640 | Inevitablewest  |      24 |  3.11 |          18 |  2.33 |                730 |  94.56 |
|            19069018 | jreichelt       |      20 |  2.59 |          21 |  2.72 |                730 |  94.68 |
| 1434450096557596680 | PolitlcsUK      |      20 |  2.59 |          45 |  5.84 |                706 |  91.57 |
|           472412809 | f_philippot     |     116 | 15.18 |          17 |  2.23 |                631 |  82.59 |
| 1316995857242378240 | realMaalouf     |      44 |  5.80 |          53 |  6.99 |                661 |  87.20 |
| 1661674273122160641 | \_maakun\_\_2   |     139 | 18.39 |           0 |  0.00 |                617 |  81.61 |
|             1137701 | DavidSacks      |      21 |  2.81 |          63 |  8.43 |                663 |  88.76 |
|          2162812627 | nicksortor      |      37 |  4.95 |          22 |  2.95 |                688 |  92.10 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      55 |  7.46 |           7 |  0.95 |                675 |  91.59 |
|            18576537 | IDF             |       0 |  0.00 |          74 | 10.07 |                661 |  89.93 |
|           351491321 | wallstwolverine |      20 |  2.72 |          35 |  4.77 |                679 |  92.51 |
| 1062754443798532096 | MakisMD         |      20 |  2.72 |          10 |  1.36 |                704 |  95.91 |
| 1227799690579718144 | VivekGRamaswamy |      16 |  2.18 |          34 |  4.63 |                684 |  93.19 |
| 1485068728412913666 | interesting_aIl |      89 | 12.14 |          48 |  6.55 |                596 |  81.31 |
|              742143 | BBCWorld        |      29 |  3.96 |          37 |  5.05 |                666 |  90.98 |
|          4691437897 | darrengrimes\_  |      36 |  4.92 |          18 |  2.46 |                678 |  92.62 |
| 1158115510606815232 | therealbuni     |      83 | 11.39 |          28 |  3.84 |                618 |  84.77 |
|           195853497 | UpdateNews724   |      59 |  8.15 |          39 |  5.39 |                626 |  86.46 |
|            16906660 | Grummz          |       5 |  0.69 |          25 |  3.46 |                693 |  95.85 |
|            94324983 | FonsiLoaiza     |      13 |  1.80 |          20 |  2.77 |                689 |  95.43 |
| 1298372735383605249 | RonFilipkowski  |      18 |  2.51 |          35 |  4.88 |                664 |  92.61 |
| 1151126931439407107 | RimaHas         |      16 |  2.24 |          54 |  7.55 |                645 |  90.21 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  1.97 |                698 |  98.03 |
| 1392864463204782082 | WarMonitors     |      20 |  2.82 |          34 |  4.79 |                656 |  92.39 |
|          2161051908 | AvivaKlompas    |      13 |  1.84 |          63 |  8.92 |                630 |  89.24 |
|           221593437 | droidindo       |       0 |  0.00 |           0 |  0.00 |                699 | 100.00 |
|           112103858 | ALeaument       |      28 |  4.01 |          33 |  4.73 |                637 |  91.26 |
| 1604139215406727170 | CensoredMen     |      34 |  4.89 |          46 |  6.62 |                615 |  88.49 |
|          2470489110 | mattariver1     |     121 | 17.49 |          16 |  2.31 |                555 |  80.20 |
| 1450241520859156483 | geoscience16    |     183 | 26.45 |           5 |  0.72 |                504 |  72.83 |
|            19923515 | abc_es          |      73 | 10.56 |          20 |  2.89 |                598 |  86.54 |
| 1594871021978652673 | FearedBuck      |      87 | 12.59 |          57 |  8.25 |                547 |  79.16 |
| 1449140157903482882 | BRICSinfo       |      12 |  1.74 |          40 |  5.81 |                636 |  92.44 |
| 1711732984338878465 | NewsLiberdade   |      36 |  5.26 |          14 |  2.05 |                634 |  92.69 |
|            15801906 | TristanSnell    |      23 |  3.39 |          23 |  3.39 |                633 |  93.23 |
| 1587130893616955393 | cagiago\_       |      52 |  7.68 |          25 |  3.69 |                600 |  88.63 |
|              759251 | CNN             |      16 |  2.37 |          20 |  2.96 |                639 |  94.67 |
