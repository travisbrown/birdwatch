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
|            44196397 | elonmusk        |     126 |  0.63 |        2188 | 10.87 |              17822 | 88.51 |
|           629698642 | BGatesIsaPyscho |    1179 | 22.45 |         134 |  2.55 |               3939 | 75.00 |
| 1151913018936053760 | jacksonhinklle  |     363 |  8.11 |         230 |  5.14 |               3881 | 86.75 |
|          3376321847 | iluminatibot    |     815 | 18.66 |         106 |  2.43 |               3447 | 78.91 |
| 1349149096909668363 | POTUS           |      60 |  1.51 |         253 |  6.36 |               3664 | 92.13 |
| 1552795969959636992 | EndWokeness     |      73 |  1.88 |         165 |  4.26 |               3635 | 93.85 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |         103 |  2.85 |               3506 | 97.12 |
|           191871143 | VigilantFox     |      35 |  0.98 |          51 |  1.43 |               3484 | 97.59 |
|              939091 | JoeBiden        |      16 |  0.46 |         304 |  8.71 |               3172 | 90.84 |
|          1291128494 | ShaykhSulaiman  |     263 |  7.66 |         159 |  4.63 |               3010 | 87.70 |
| 1222773302441148416 | visegrad24      |     167 |  5.08 |         289 |  8.80 |               2829 | 86.12 |
| 1319287761048723458 | MarioNawfal     |     194 |  5.92 |         103 |  3.14 |               2980 | 90.94 |
| 1326229737551912960 | libsoftiktok    |      49 |  1.50 |         106 |  3.24 |               3121 | 95.27 |
|  953378142198161409 | choquei         |     461 | 14.99 |          71 |  2.31 |               2543 | 82.70 |
| 1446231057259433988 | DrLoupis        |     216 |  7.22 |         125 |  4.18 |               2652 | 88.61 |
|            16106584 | stillgray       |     305 | 11.18 |         112 |  4.10 |               2312 | 84.72 |
|          3260357396 | thehealthb0t    |     450 | 17.84 |          39 |  1.55 |               2034 | 80.62 |
|            30354991 | KamalaHarris    |       8 |  0.33 |         111 |  4.56 |               2317 | 95.11 |
|  896550698543874049 | UTDTrey         |     230 |  9.47 |          76 |  3.13 |               2124 | 87.41 |
|          3096147528 | dom_lucre       |     292 | 12.45 |          93 |  3.96 |               1961 | 83.59 |
|            76766018 | Dexerto         |     238 | 10.62 |         182 |  8.12 |               1822 | 81.27 |
| 1099579684981944320 | WallStreetApes  |     114 |  5.13 |          23 |  1.04 |               2084 | 93.83 |
| 1429400366819512323 | RadioGenoa      |      57 |  2.66 |         116 |  5.42 |               1967 | 91.92 |
| 1528943647185678336 | LeadingReport   |      56 |  2.66 |          38 |  1.80 |               2014 | 95.54 |
| 1374968033265864706 | TaraBull808     |     118 |  5.63 |          42 |  2.00 |               1936 | 92.37 |
| 1366565625401909249 | WallStreetSilv  |     105 |  5.09 |          74 |  3.59 |               1884 | 91.32 |
|  805532293951606785 | MattWallace888  |     238 | 11.61 |          89 |  4.34 |               1723 | 84.05 |
|  801203059359678464 | DrEliDavid      |      42 |  2.15 |         156 |  8.00 |               1751 | 89.84 |
|            80233893 | jakeshieldsajj  |     106 |  5.57 |         105 |  5.51 |               1693 | 88.92 |
|          2538322138 | PicturesFoIder  |     122 |  6.82 |         190 | 10.63 |               1476 | 82.55 |
|           537709549 | LauraLoomer     |     212 | 11.88 |          48 |  2.69 |               1524 | 85.43 |
|            49849732 | petrogustavo    |      41 |  2.36 |          48 |  2.76 |               1649 | 94.88 |
|           971820228 | CerfiaFR        |      79 |  4.56 |         107 |  6.18 |               1545 | 89.25 |
|  855481986290524160 | historyinmemes  |     148 |  8.67 |         178 | 10.43 |               1381 | 80.90 |
|            25073877 | realDonaldTrump |       5 |  0.30 |         158 |  9.40 |               1517 | 90.30 |
|            32867753 | silvano_trotta  |     383 | 22.80 |          15 |  0.89 |               1282 | 76.31 |
|           312696314 | Partisangirl    |     131 |  7.95 |          75 |  4.55 |               1441 | 87.49 |
| 1389913567671975937 | cb_doge         |      38 |  2.33 |          95 |  5.82 |               1500 | 91.86 |
|            15212187 | bennyjohnson    |      82 |  5.20 |          56 |  3.55 |               1438 | 91.24 |
| 1355721251180961792 | GuntherEagleman |      50 |  3.19 |          37 |  2.36 |               1482 | 94.46 |
| 1344356576786866176 | RepMTG          |      71 |  4.55 |          56 |  3.59 |               1434 | 91.86 |
|  890061634181373952 | CollinRugg      |      51 |  3.29 |          67 |  4.33 |               1431 | 92.38 |
|           133663801 | BFMTV           |     146 |  9.51 |          91 |  5.92 |               1299 | 84.57 |
|  855483824351924224 | liz_churchill10 |     156 | 10.20 |          30 |  1.96 |               1344 | 87.84 |
|           592730371 | JackPosobiec    |      84 |  5.65 |          41 |  2.76 |               1361 | 91.59 |
| 1486069441259397125 | harris_wins     |      18 |  1.23 |          54 |  3.68 |               1394 | 95.09 |
|  807355480188141568 | DramaAlert      |     239 | 16.33 |          83 |  5.67 |               1142 | 78.01 |
|            17469289 | nypost          |     111 |  7.61 |          59 |  4.04 |               1289 | 88.35 |
|            91416107 | OliLondonTV     |      64 |  4.43 |          81 |  5.61 |               1299 | 89.96 |
|           133938408 | krassenstein    |      15 |  1.06 |          91 |  6.40 |               1315 | 92.54 |
|  750683331260321792 | MAstronomers    |      71 |  5.04 |          82 |  5.82 |               1255 | 89.13 |
|           177101260 | Rainmaker1973   |      24 |  1.77 |         149 | 10.96 |               1186 | 87.27 |
| 1455903807389458436 | BladeoftheS     |     168 | 12.38 |          24 |  1.77 |               1165 | 85.85 |
|            14281853 | Conservatives   |      72 |  5.33 |          42 |  3.11 |               1236 | 91.56 |
|           428454304 | harryjsisson    |      16 |  1.21 |          42 |  3.17 |               1265 | 95.62 |
|            52660746 | Israel          |       6 |  0.47 |         122 |  9.46 |               1162 | 90.08 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1138 | 89.96 |
|            96684891 | kharaguchi      |     128 | 10.13 |           9 |  0.71 |               1126 | 89.15 |
|           452540168 | DailyLoud       |     313 | 25.08 |          76 |  6.09 |                859 | 68.83 |
|            16635277 | Acyn            |      40 |  3.21 |          84 |  6.74 |               1123 | 90.06 |
|            62957739 | eduardomenoni   |     304 | 24.48 |          17 |  1.37 |                921 | 74.15 |
|            14594813 | folha           |      45 |  3.71 |          31 |  2.56 |               1137 | 93.73 |
|            37501849 | Quakeprediction |      40 |  3.33 |           2 |  0.17 |               1160 | 96.51 |
|             7587032 | SkyNews         |      36 |  3.00 |          47 |  3.92 |               1116 | 93.08 |
| 1335132884278108161 | stats_feed      |     112 |  9.51 |          62 |  5.26 |               1004 | 85.23 |
| 1121807798826930177 | MyLordBebo      |     130 | 11.05 |          32 |  2.72 |               1015 | 86.24 |
| 1486473049146904576 | InternetH0F     |     122 | 10.40 |         135 | 11.51 |                916 | 78.09 |
|            69156861 | TheChiefNerd    |       9 |  0.77 |          13 |  1.11 |               1145 | 98.11 |
|           132339474 | EdKrassen       |      23 |  1.99 |          53 |  4.58 |               1082 | 93.44 |
| 1224185690713460736 | goddeketal      |      59 |  5.15 |          32 |  2.79 |               1055 | 92.06 |
|          2670726740 | LulaOficial     |       5 |  0.44 |          25 |  2.20 |               1105 | 97.36 |
| 1138458175663988738 | PopBase         |      55 |  4.86 |          90 |  7.96 |                986 | 87.18 |
|            51241574 | AP              |      19 |  1.68 |          49 |  4.34 |               1061 | 93.98 |
|           292929271 | charliekirk11   |      45 |  3.99 |          35 |  3.10 |               1048 | 92.91 |
|           255471924 | mfa_russia      |     137 | 12.18 |          53 |  4.71 |                935 | 83.11 |
|           138203134 | AOC             |      12 |  1.09 |          57 |  5.20 |               1028 | 93.71 |
| 1529763962094596097 | wideawake_media |      18 |  1.64 |          14 |  1.28 |               1065 | 97.08 |
| 1471414739880189955 | kirawontmiss    |      50 |  4.56 |         162 | 14.78 |                884 | 80.66 |
| 1043185714437992449 | catturd2        |      46 |  4.30 |          47 |  4.39 |                978 | 91.32 |
|          3331429624 | Metropoles      |      89 |  8.42 |          26 |  2.46 |                942 | 89.12 |
|           242827267 | PierrePoilievre |       1 |  0.10 |          31 |  3.02 |                995 | 96.88 |
|          1542228578 | JDVance         |       6 |  0.59 |          70 |  6.84 |                948 | 92.58 |
|           333357345 | Cobratate       |      51 |  5.00 |          89 |  8.73 |                879 | 86.26 |
|             7996082 | el_pais         |      40 |  3.94 |          30 |  2.96 |                944 | 93.10 |
|           524869533 | QudsNen         |      58 |  5.76 |          31 |  3.08 |                918 | 91.16 |
|           423692278 | AkademiksTV     |      46 |  4.58 |          14 |  1.39 |                944 | 94.02 |
|          2828212668 | AMAZlNGNATURE   |      71 |  7.07 |         125 | 12.45 |                808 | 80.48 |
|           848279118 | Kahlissee       |      37 |  3.69 |          41 |  4.09 |                925 | 92.22 |
| 1577761560394665984 | DiedSuddenly\_  |      55 |  5.49 |          23 |  2.30 |                924 | 92.22 |
|            90954365 | earthquakejapan |      87 |  8.73 |           0 |  0.00 |                910 | 91.27 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.35 |          35 |  3.56 |                916 | 93.09 |
| 1538230472299270144 | ayeejuju        |      81 |  8.27 |         106 | 10.82 |                793 | 80.92 |
| 1342174584594960384 | iamyesyouareno  |      54 |  5.53 |          59 |  6.04 |                864 | 88.43 |
| 1356434353623093249 | greg16676935420 |      20 |  2.05 |         106 | 10.86 |                850 | 87.09 |
|            22703645 | TuckerCarlson   |       1 |  0.10 |          91 |  9.37 |                879 | 90.53 |
| 1600964443122421769 | CNviolations    |      13 |  1.34 |         100 | 10.30 |                858 | 88.36 |
|           130557513 | mehdirhasan     |      14 |  1.45 |          30 |  3.10 |                923 | 95.45 |
| 1080188052365029376 | acnewsitics     |       9 |  0.94 |          59 |  6.14 |                893 | 92.92 |
|           374712154 | TRobinsonNewEra |      40 |  4.18 |          22 |  2.30 |                896 | 93.53 |
|  855300206086168576 | HumansNoContext |      99 | 10.33 |          80 |  8.35 |                779 | 81.32 |
| 1430497892314218502 | Resist_05       |      47 |  4.94 |          29 |  3.05 |                875 | 92.01 |
| 1087757588622651397 | porqueTTarg     |     206 | 21.68 |          29 |  3.05 |                715 | 75.26 |
|           706646642 | EmbassyofRussia |     106 | 11.19 |          33 |  3.48 |                808 | 85.32 |
|  780460754910732288 | DiscussingFilm  |      35 |  3.70 |         101 | 10.67 |                811 | 85.64 |
|  875856268056969216 | DC_Draino       |      17 |  1.82 |          21 |  2.24 |                898 | 95.94 |
| 1288319695658135552 | realstewpeters  |     187 | 20.00 |          36 |  3.85 |                712 | 76.15 |
|             4239551 | amuse           |      29 |  3.11 |          28 |  3.01 |                874 | 93.88 |
|            37491797 | stkirsch        |      62 |  6.67 |          12 |  1.29 |                855 | 92.03 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|            11347122 | GavinNewsom     |      10 |  1.09 |          23 |  2.50 |                886 | 96.41 |
|            80820758 | JLMelenchon     |      59 |  6.43 |          60 |  6.54 |                799 | 87.04 |
| 1111976778065723393 | nocontextfooty  |     106 | 11.55 |          59 |  6.43 |                753 | 82.03 |
|  795188519115358208 | tweetsoku1      |     280 | 30.63 |           7 |  0.77 |                627 | 68.60 |
|           288277167 | atrupar         |      22 |  2.44 |          37 |  4.10 |                843 | 93.46 |
| 1090084079964348419 | MrSinha\_       |      22 |  2.44 |           9 |  1.00 |                870 | 96.56 |
|            14128609 | felipeneto      |      18 |  2.00 |          31 |  3.44 |                851 | 94.56 |
| 1187524450809536513 | vitoquiles      |      40 |  4.48 |          26 |  2.91 |                827 | 92.61 |
| 1128337957289697281 | SprinterFamily  |     220 | 24.66 |          21 |  2.35 |                651 | 72.98 |
|           142393421 | GloboNews       |      21 |  2.36 |          17 |  1.91 |                853 | 95.74 |
| 1562038858988064768 | TheFigen\_      |     109 | 12.27 |          85 |  9.57 |                694 | 78.15 |
|  918197046871502849 | siteptbr        |     102 | 11.51 |          25 |  2.82 |                759 | 85.67 |
|           978932870 | CapitanBitcoin  |      38 |  4.31 |          23 |  2.61 |                820 | 93.08 |
|           463142998 | SuppressedNws   |      14 |  1.59 |          35 |  3.98 |                830 | 94.43 |
|           294293982 | Rothmus         |      54 |  6.16 |          44 |  5.02 |                779 | 88.83 |
|  959531564341317632 | AlertesInfos    |      38 |  4.34 |          58 |  6.63 |                779 | 89.03 |
| 1225234593789423616 | Megatron_ron    |     113 | 12.91 |          43 |  4.91 |                719 | 82.17 |
|          1446465174 | akafacehots     |      87 |  9.95 |          24 |  2.75 |                763 | 87.30 |
|          4429003533 | PopCrave        |      48 |  5.51 |          70 |  8.04 |                753 | 86.45 |
|  707231479047315456 | PeterSweden7    |      24 |  2.76 |          15 |  1.72 |                831 | 95.52 |
|          1640929196 | mmpadellan      |      26 |  3.00 |          18 |  2.07 |                824 | 94.93 |
| 1492007194388279333 | LibertyCappy    |      30 |  3.50 |          60 |  7.01 |                766 | 89.49 |
| 1393726565809278976 | AdameMedia      |      21 |  2.46 |          39 |  4.57 |                793 | 92.97 |
|           319774010 | kirinjisinken   |     455 | 53.72 |          11 |  1.30 |                381 | 44.98 |
|  818893114979061761 | JoJoFromJerz    |      12 |  1.42 |          32 |  3.78 |                802 | 94.80 |
|           109398997 | stopvaccinating |      93 | 11.01 |          10 |  1.18 |                742 | 87.81 |
| 1431774993419956224 | jhmdrei         |     228 | 27.11 |           3 |  0.36 |                610 | 72.53 |
| 1179892477714718721 | gunsnrosesgirl3 |      26 |  3.10 |          97 | 11.58 |                715 | 85.32 |
| 1593929531148144645 | stairwayto3dom  |      57 |  6.80 |          31 |  3.70 |                750 | 89.50 |
|          4020276615 | JMilei          |       4 |  0.48 |          56 |  6.76 |                769 | 92.76 |
|           330262748 | FabrizioRomano  |      67 |  8.15 |         101 | 12.29 |                654 | 79.56 |
|           535707261 | eldiarioes      |      17 |  2.07 |          15 |  1.83 |                788 | 96.10 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          14 |  1.71 |                805 | 98.29 |
|           109065990 | RealAlexJones   |      55 |  6.76 |          24 |  2.95 |                735 | 90.29 |
| 1302674142630760450 | EverythingOOC   |      69 |  8.50 |          87 | 10.71 |                656 | 80.79 |
| 1432287556129812484 | himuro398       |     165 | 20.32 |          11 |  1.35 |                636 | 78.33 |
|            46302096 | JoeyMannarinoUS |      60 |  7.43 |          24 |  2.97 |                723 | 89.59 |
|            19017675 | Nigel_Farage    |       7 |  0.87 |          33 |  4.11 |                763 | 95.02 |
| 1221462414744596483 | RpsAgainstTrump |       9 |  1.12 |          79 |  9.86 |                713 | 89.01 |
| 1686901686185721857 | TrumpDailyPosts |      11 |  1.37 |          34 |  4.24 |                756 | 94.38 |
|            14260960 | JustinTrudeau   |       1 |  0.13 |          60 |  7.53 |                736 | 92.35 |
| 1200616796295847936 | unusual_whales  |      95 | 11.99 |          32 |  4.04 |                665 | 83.96 |
|            27000730 | Timcast         |      15 |  1.91 |          37 |  4.72 |                732 | 93.37 |
|  826065164504006657 | mtgreenee       |      36 |  4.64 |          37 |  4.77 |                703 | 90.59 |
|            39344374 | DonaldJTrumpJr  |      31 |  4.01 |          41 |  5.30 |                702 | 90.70 |
|            39692424 | AlertaNews24    |     132 | 17.17 |          23 |  2.99 |                614 | 79.84 |
| 1625843518643085312 | creepydotorg    |      56 |  7.28 |          62 |  8.06 |                651 | 84.66 |
| 1434450096557596680 | PolitlcsUK      |      20 |  2.60 |          45 |  5.86 |                703 | 91.54 |
| 1514045582968598530 | CFC_Janty       |      63 |  8.24 |          28 |  3.66 |                674 | 88.10 |
|          1500129642 | MattWalshBlog   |      11 |  1.45 |          28 |  3.68 |                722 | 94.88 |
|            19069018 | jreichelt       |      20 |  2.64 |          21 |  2.77 |                717 | 94.59 |
|          1626294277 | spectatorindex  |      12 |  1.59 |          47 |  6.21 |                698 | 92.21 |
|            14436030 | elmundoes       |      62 |  8.22 |          13 |  1.72 |                679 | 90.05 |
| 1138842105856573445 | NoContextHumans |      69 |  9.20 |          84 | 11.20 |                597 | 79.60 |
| 1661674273122160641 | \_maakun\_\_2   |     139 | 18.61 |           0 |  0.00 |                608 | 81.39 |
| 1157689921349521410 | esjesjesj       |      15 |  2.01 |          51 |  6.85 |                679 | 91.14 |
| 1062754443798532096 | MakisMD         |      20 |  2.74 |          10 |  1.37 |                699 | 95.88 |
|            18576537 | IDF             |       0 |  0.00 |          72 |  9.90 |                655 | 90.10 |
|           472412809 | f_philippot     |     113 | 15.63 |          16 |  2.21 |                594 | 82.16 |
| 1227799690579718144 | VivekGRamaswamy |      16 |  2.21 |          34 |  4.70 |                673 | 93.08 |
|              742143 | BBCWorld        |      28 |  3.91 |          36 |  5.02 |                653 | 91.07 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      55 |  7.70 |           7 |  0.98 |                652 | 91.32 |
|          4691437897 | darrengrimes\_  |      36 |  5.06 |          18 |  2.53 |                658 | 92.42 |
|           351491321 | wallstwolverine |      18 |  2.54 |          34 |  4.80 |                657 | 92.67 |
| 1158115510606815232 | therealbuni     |      82 | 11.57 |          28 |  3.95 |                599 | 84.49 |
| 1298372735383605249 | RonFilipkowski  |      18 |  2.55 |          34 |  4.82 |                653 | 92.62 |
|             1137701 | DavidSacks      |      21 |  2.99 |          57 |  8.11 |                625 | 88.90 |
|          2161051908 | AvivaKlompas    |      13 |  1.86 |          61 |  8.74 |                624 | 89.40 |
|            94324983 | FonsiLoaiza     |      13 |  1.87 |          20 |  2.87 |                664 | 95.27 |
| 1392864463204782082 | WarMonitors     |      20 |  2.87 |          32 |  4.60 |                644 | 92.53 |
|           195853497 | EuropeInvasions |      53 |  7.63 |          39 |  5.61 |                603 | 86.76 |
| 1604139215406727170 | CensoredMen     |      34 |  4.89 |          46 |  6.62 |                615 | 88.49 |
| 1316995857242378240 | realMaalouf     |      41 |  5.91 |          51 |  7.35 |                602 | 86.74 |
| 1485068728412913666 | interesting_aIl |      83 | 11.99 |          46 |  6.65 |                563 | 81.36 |
| 1450241520859156483 | geoscience16    |     183 | 26.60 |           5 |  0.73 |                500 | 72.67 |
|           112103858 | ALeaument       |      26 |  3.80 |          33 |  4.82 |                625 | 91.37 |
|            19923515 | abc_es          |      72 | 10.56 |          19 |  2.79 |                591 | 86.66 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.06 |                665 | 97.94 |
|            16906660 | Grummz          |       5 |  0.74 |          25 |  3.70 |                646 | 95.56 |
|           337808606 | RobertKennedyJr |      11 |  1.63 |          33 |  4.90 |                630 | 93.47 |
|              759251 | CNN             |      16 |  2.39 |          19 |  2.84 |                634 | 94.77 |
|          2162812627 | nicksortor      |      34 |  5.12 |          21 |  3.16 |                609 | 91.72 |
| 1151126931439407107 | RimaHas         |      16 |  2.42 |          52 |  7.87 |                593 | 89.71 |
|            62513246 | jk_rowling      |       0 |  0.00 |          60 |  9.19 |                593 | 90.81 |
| 1339166129110065152 | GBNEWS          |      27 |  4.13 |          11 |  1.68 |                615 | 94.18 |
|            14377605 | TheDemocrats    |       4 |  0.61 |          25 |  3.84 |                622 | 95.55 |
| 1849500209488752640 |                 |      18 |  2.77 |          14 |  2.16 |                617 | 95.07 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
| 1587130893616955393 | cagiago\_       |      52 |  8.05 |          25 |  3.87 |                569 | 88.08 |
|            31139434 | gleisi          |      15 |  2.33 |          15 |  2.33 |                614 | 95.34 |
|             5734902 | tagesschau      |      15 |  2.34 |          24 |  3.74 |                602 | 93.92 |
