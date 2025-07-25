#[derive(Debug, Clone, Copy)]
pub enum Icon {
    ///[a-arrow-down](https://lucide.dev/icons/a-arrow-down) icon
    AArrowDown,
    ///[a-arrow-up](https://lucide.dev/icons/a-arrow-up) icon
    AArrowUp,
    ///[a-large-small](https://lucide.dev/icons/a-large-small) icon
    ALargeSmall,
    ///[accessibility](https://lucide.dev/icons/accessibility) icon
    Accessibility,
    ///[activity](https://lucide.dev/icons/activity) icon
    Activity,
    ///[air-vent](https://lucide.dev/icons/air-vent) icon
    AirVent,
    ///[airplay](https://lucide.dev/icons/airplay) icon
    Airplay,
    ///[alarm-clock](https://lucide.dev/icons/alarm-clock) icon
    AlarmClock,
    ///[alarm-clock-check](https://lucide.dev/icons/alarm-clock-check) icon
    AlarmClockCheck,
    ///[alarm-clock-minus](https://lucide.dev/icons/alarm-clock-minus) icon
    AlarmClockMinus,
    ///[alarm-clock-off](https://lucide.dev/icons/alarm-clock-off) icon
    AlarmClockOff,
    ///[alarm-clock-plus](https://lucide.dev/icons/alarm-clock-plus) icon
    AlarmClockPlus,
    ///[alarm-smoke](https://lucide.dev/icons/alarm-smoke) icon
    AlarmSmoke,
    ///[album](https://lucide.dev/icons/album) icon
    Album,
    ///[align-center](https://lucide.dev/icons/align-center) icon
    AlignCenter,
    ///[align-center-horizontal](https://lucide.dev/icons/align-center-horizontal) icon
    AlignCenterHorizontal,
    ///[align-center-vertical](https://lucide.dev/icons/align-center-vertical) icon
    AlignCenterVertical,
    ///[align-end-horizontal](https://lucide.dev/icons/align-end-horizontal) icon
    AlignEndHorizontal,
    ///[align-end-vertical](https://lucide.dev/icons/align-end-vertical) icon
    AlignEndVertical,
    ///[align-horizontal-distribute-center](https://lucide.dev/icons/align-horizontal-distribute-center) icon
    AlignHorizontalDistributeCenter,
    ///[align-horizontal-distribute-end](https://lucide.dev/icons/align-horizontal-distribute-end) icon
    AlignHorizontalDistributeEnd,
    ///[align-horizontal-distribute-start](https://lucide.dev/icons/align-horizontal-distribute-start) icon
    AlignHorizontalDistributeStart,
    ///[align-horizontal-justify-center](https://lucide.dev/icons/align-horizontal-justify-center) icon
    AlignHorizontalJustifyCenter,
    ///[align-horizontal-justify-end](https://lucide.dev/icons/align-horizontal-justify-end) icon
    AlignHorizontalJustifyEnd,
    ///[align-horizontal-justify-start](https://lucide.dev/icons/align-horizontal-justify-start) icon
    AlignHorizontalJustifyStart,
    ///[align-horizontal-space-around](https://lucide.dev/icons/align-horizontal-space-around) icon
    AlignHorizontalSpaceAround,
    ///[align-horizontal-space-between](https://lucide.dev/icons/align-horizontal-space-between) icon
    AlignHorizontalSpaceBetween,
    ///[align-justify](https://lucide.dev/icons/align-justify) icon
    AlignJustify,
    ///[align-left](https://lucide.dev/icons/align-left) icon
    AlignLeft,
    ///[align-right](https://lucide.dev/icons/align-right) icon
    AlignRight,
    ///[align-start-horizontal](https://lucide.dev/icons/align-start-horizontal) icon
    AlignStartHorizontal,
    ///[align-start-vertical](https://lucide.dev/icons/align-start-vertical) icon
    AlignStartVertical,
    ///[align-vertical-distribute-center](https://lucide.dev/icons/align-vertical-distribute-center) icon
    AlignVerticalDistributeCenter,
    ///[align-vertical-distribute-end](https://lucide.dev/icons/align-vertical-distribute-end) icon
    AlignVerticalDistributeEnd,
    ///[align-vertical-distribute-start](https://lucide.dev/icons/align-vertical-distribute-start) icon
    AlignVerticalDistributeStart,
    ///[align-vertical-justify-center](https://lucide.dev/icons/align-vertical-justify-center) icon
    AlignVerticalJustifyCenter,
    ///[align-vertical-justify-end](https://lucide.dev/icons/align-vertical-justify-end) icon
    AlignVerticalJustifyEnd,
    ///[align-vertical-justify-start](https://lucide.dev/icons/align-vertical-justify-start) icon
    AlignVerticalJustifyStart,
    ///[align-vertical-space-around](https://lucide.dev/icons/align-vertical-space-around) icon
    AlignVerticalSpaceAround,
    ///[align-vertical-space-between](https://lucide.dev/icons/align-vertical-space-between) icon
    AlignVerticalSpaceBetween,
    ///[ambulance](https://lucide.dev/icons/ambulance) icon
    Ambulance,
    ///[ampersand](https://lucide.dev/icons/ampersand) icon
    Ampersand,
    ///[ampersands](https://lucide.dev/icons/ampersands) icon
    Ampersands,
    ///[amphora](https://lucide.dev/icons/amphora) icon
    Amphora,
    ///[anchor](https://lucide.dev/icons/anchor) icon
    Anchor,
    ///[angry](https://lucide.dev/icons/angry) icon
    Angry,
    ///[annoyed](https://lucide.dev/icons/annoyed) icon
    Annoyed,
    ///[antenna](https://lucide.dev/icons/antenna) icon
    Antenna,
    ///[anvil](https://lucide.dev/icons/anvil) icon
    Anvil,
    ///[aperture](https://lucide.dev/icons/aperture) icon
    Aperture,
    ///[app-window](https://lucide.dev/icons/app-window) icon
    AppWindow,
    ///[app-window-mac](https://lucide.dev/icons/app-window-mac) icon
    AppWindowMac,
    ///[apple](https://lucide.dev/icons/apple) icon
    Apple,
    ///[archive](https://lucide.dev/icons/archive) icon
    Archive,
    ///[archive-restore](https://lucide.dev/icons/archive-restore) icon
    ArchiveRestore,
    ///[archive-x](https://lucide.dev/icons/archive-x) icon
    ArchiveX,
    ///[armchair](https://lucide.dev/icons/armchair) icon
    Armchair,
    ///[arrow-big-down](https://lucide.dev/icons/arrow-big-down) icon
    ArrowBigDown,
    ///[arrow-big-down-dash](https://lucide.dev/icons/arrow-big-down-dash) icon
    ArrowBigDownDash,
    ///[arrow-big-left](https://lucide.dev/icons/arrow-big-left) icon
    ArrowBigLeft,
    ///[arrow-big-left-dash](https://lucide.dev/icons/arrow-big-left-dash) icon
    ArrowBigLeftDash,
    ///[arrow-big-right](https://lucide.dev/icons/arrow-big-right) icon
    ArrowBigRight,
    ///[arrow-big-right-dash](https://lucide.dev/icons/arrow-big-right-dash) icon
    ArrowBigRightDash,
    ///[arrow-big-up](https://lucide.dev/icons/arrow-big-up) icon
    ArrowBigUp,
    ///[arrow-big-up-dash](https://lucide.dev/icons/arrow-big-up-dash) icon
    ArrowBigUpDash,
    ///[arrow-down](https://lucide.dev/icons/arrow-down) icon
    ArrowDown,
    ///[arrow-down-0-1](https://lucide.dev/icons/arrow-down-0-1) icon
    ArrowDown01,
    ///[arrow-down-1-0](https://lucide.dev/icons/arrow-down-1-0) icon
    ArrowDown10,
    ///[arrow-down-a-z](https://lucide.dev/icons/arrow-down-a-z) icon
    ArrowDownAZ,
    ///[arrow-down-from-line](https://lucide.dev/icons/arrow-down-from-line) icon
    ArrowDownFromLine,
    ///[arrow-down-left](https://lucide.dev/icons/arrow-down-left) icon
    ArrowDownLeft,
    ///[arrow-down-narrow-wide](https://lucide.dev/icons/arrow-down-narrow-wide) icon
    ArrowDownNarrowWide,
    ///[arrow-down-right](https://lucide.dev/icons/arrow-down-right) icon
    ArrowDownRight,
    ///[arrow-down-to-dot](https://lucide.dev/icons/arrow-down-to-dot) icon
    ArrowDownToDot,
    ///[arrow-down-to-line](https://lucide.dev/icons/arrow-down-to-line) icon
    ArrowDownToLine,
    ///[arrow-down-up](https://lucide.dev/icons/arrow-down-up) icon
    ArrowDownUp,
    ///[arrow-down-wide-narrow](https://lucide.dev/icons/arrow-down-wide-narrow) icon
    ArrowDownWideNarrow,
    ///[arrow-down-z-a](https://lucide.dev/icons/arrow-down-z-a) icon
    ArrowDownZA,
    ///[arrow-left](https://lucide.dev/icons/arrow-left) icon
    ArrowLeft,
    ///[arrow-left-from-line](https://lucide.dev/icons/arrow-left-from-line) icon
    ArrowLeftFromLine,
    ///[arrow-left-right](https://lucide.dev/icons/arrow-left-right) icon
    ArrowLeftRight,
    ///[arrow-left-to-line](https://lucide.dev/icons/arrow-left-to-line) icon
    ArrowLeftToLine,
    ///[arrow-right](https://lucide.dev/icons/arrow-right) icon
    ArrowRight,
    ///[arrow-right-from-line](https://lucide.dev/icons/arrow-right-from-line) icon
    ArrowRightFromLine,
    ///[arrow-right-left](https://lucide.dev/icons/arrow-right-left) icon
    ArrowRightLeft,
    ///[arrow-right-to-line](https://lucide.dev/icons/arrow-right-to-line) icon
    ArrowRightToLine,
    ///[arrow-up](https://lucide.dev/icons/arrow-up) icon
    ArrowUp,
    ///[arrow-up-0-1](https://lucide.dev/icons/arrow-up-0-1) icon
    ArrowUp01,
    ///[arrow-up-1-0](https://lucide.dev/icons/arrow-up-1-0) icon
    ArrowUp10,
    ///[arrow-up-a-z](https://lucide.dev/icons/arrow-up-a-z) icon
    ArrowUpAZ,
    ///[arrow-up-down](https://lucide.dev/icons/arrow-up-down) icon
    ArrowUpDown,
    ///[arrow-up-from-dot](https://lucide.dev/icons/arrow-up-from-dot) icon
    ArrowUpFromDot,
    ///[arrow-up-from-line](https://lucide.dev/icons/arrow-up-from-line) icon
    ArrowUpFromLine,
    ///[arrow-up-left](https://lucide.dev/icons/arrow-up-left) icon
    ArrowUpLeft,
    ///[arrow-up-narrow-wide](https://lucide.dev/icons/arrow-up-narrow-wide) icon
    ArrowUpNarrowWide,
    ///[arrow-up-right](https://lucide.dev/icons/arrow-up-right) icon
    ArrowUpRight,
    ///[arrow-up-to-line](https://lucide.dev/icons/arrow-up-to-line) icon
    ArrowUpToLine,
    ///[arrow-up-wide-narrow](https://lucide.dev/icons/arrow-up-wide-narrow) icon
    ArrowUpWideNarrow,
    ///[arrow-up-z-a](https://lucide.dev/icons/arrow-up-z-a) icon
    ArrowUpZA,
    ///[arrows-up-from-line](https://lucide.dev/icons/arrows-up-from-line) icon
    ArrowsUpFromLine,
    ///[asterisk](https://lucide.dev/icons/asterisk) icon
    Asterisk,
    ///[at-sign](https://lucide.dev/icons/at-sign) icon
    AtSign,
    ///[atom](https://lucide.dev/icons/atom) icon
    Atom,
    ///[audio-lines](https://lucide.dev/icons/audio-lines) icon
    AudioLines,
    ///[audio-waveform](https://lucide.dev/icons/audio-waveform) icon
    AudioWaveform,
    ///[award](https://lucide.dev/icons/award) icon
    Award,
    ///[axe](https://lucide.dev/icons/axe) icon
    Axe,
    ///[axis-3d](https://lucide.dev/icons/axis-3d) icon
    Axis3d,
    ///[baby](https://lucide.dev/icons/baby) icon
    Baby,
    ///[backpack](https://lucide.dev/icons/backpack) icon
    Backpack,
    ///[badge](https://lucide.dev/icons/badge) icon
    Badge,
    ///[badge-alert](https://lucide.dev/icons/badge-alert) icon
    BadgeAlert,
    ///[badge-cent](https://lucide.dev/icons/badge-cent) icon
    BadgeCent,
    ///[badge-check](https://lucide.dev/icons/badge-check) icon
    BadgeCheck,
    ///[badge-dollar-sign](https://lucide.dev/icons/badge-dollar-sign) icon
    BadgeDollarSign,
    ///[badge-euro](https://lucide.dev/icons/badge-euro) icon
    BadgeEuro,
    ///[badge-indian-rupee](https://lucide.dev/icons/badge-indian-rupee) icon
    BadgeIndianRupee,
    ///[badge-info](https://lucide.dev/icons/badge-info) icon
    BadgeInfo,
    ///[badge-japanese-yen](https://lucide.dev/icons/badge-japanese-yen) icon
    BadgeJapaneseYen,
    ///[badge-minus](https://lucide.dev/icons/badge-minus) icon
    BadgeMinus,
    ///[badge-percent](https://lucide.dev/icons/badge-percent) icon
    BadgePercent,
    ///[badge-plus](https://lucide.dev/icons/badge-plus) icon
    BadgePlus,
    ///[badge-pound-sterling](https://lucide.dev/icons/badge-pound-sterling) icon
    BadgePoundSterling,
    ///[badge-question-mark](https://lucide.dev/icons/badge-question-mark) icon
    BadgeQuestionMark,
    ///[badge-russian-ruble](https://lucide.dev/icons/badge-russian-ruble) icon
    BadgeRussianRuble,
    ///[badge-swiss-franc](https://lucide.dev/icons/badge-swiss-franc) icon
    BadgeSwissFranc,
    ///[badge-x](https://lucide.dev/icons/badge-x) icon
    BadgeX,
    ///[baggage-claim](https://lucide.dev/icons/baggage-claim) icon
    BaggageClaim,
    ///[ban](https://lucide.dev/icons/ban) icon
    Ban,
    ///[banana](https://lucide.dev/icons/banana) icon
    Banana,
    ///[bandage](https://lucide.dev/icons/bandage) icon
    Bandage,
    ///[banknote](https://lucide.dev/icons/banknote) icon
    Banknote,
    ///[banknote-arrow-down](https://lucide.dev/icons/banknote-arrow-down) icon
    BanknoteArrowDown,
    ///[banknote-arrow-up](https://lucide.dev/icons/banknote-arrow-up) icon
    BanknoteArrowUp,
    ///[banknote-x](https://lucide.dev/icons/banknote-x) icon
    BanknoteX,
    ///[barcode](https://lucide.dev/icons/barcode) icon
    Barcode,
    ///[barrel](https://lucide.dev/icons/barrel) icon
    Barrel,
    ///[baseline](https://lucide.dev/icons/baseline) icon
    Baseline,
    ///[bath](https://lucide.dev/icons/bath) icon
    Bath,
    ///[battery](https://lucide.dev/icons/battery) icon
    Battery,
    ///[battery-charging](https://lucide.dev/icons/battery-charging) icon
    BatteryCharging,
    ///[battery-full](https://lucide.dev/icons/battery-full) icon
    BatteryFull,
    ///[battery-low](https://lucide.dev/icons/battery-low) icon
    BatteryLow,
    ///[battery-medium](https://lucide.dev/icons/battery-medium) icon
    BatteryMedium,
    ///[battery-plus](https://lucide.dev/icons/battery-plus) icon
    BatteryPlus,
    ///[battery-warning](https://lucide.dev/icons/battery-warning) icon
    BatteryWarning,
    ///[beaker](https://lucide.dev/icons/beaker) icon
    Beaker,
    ///[bean](https://lucide.dev/icons/bean) icon
    Bean,
    ///[bean-off](https://lucide.dev/icons/bean-off) icon
    BeanOff,
    ///[bed](https://lucide.dev/icons/bed) icon
    Bed,
    ///[bed-double](https://lucide.dev/icons/bed-double) icon
    BedDouble,
    ///[bed-single](https://lucide.dev/icons/bed-single) icon
    BedSingle,
    ///[beef](https://lucide.dev/icons/beef) icon
    Beef,
    ///[beer](https://lucide.dev/icons/beer) icon
    Beer,
    ///[beer-off](https://lucide.dev/icons/beer-off) icon
    BeerOff,
    ///[bell](https://lucide.dev/icons/bell) icon
    Bell,
    ///[bell-dot](https://lucide.dev/icons/bell-dot) icon
    BellDot,
    ///[bell-electric](https://lucide.dev/icons/bell-electric) icon
    BellElectric,
    ///[bell-minus](https://lucide.dev/icons/bell-minus) icon
    BellMinus,
    ///[bell-off](https://lucide.dev/icons/bell-off) icon
    BellOff,
    ///[bell-plus](https://lucide.dev/icons/bell-plus) icon
    BellPlus,
    ///[bell-ring](https://lucide.dev/icons/bell-ring) icon
    BellRing,
    ///[between-horizontal-end](https://lucide.dev/icons/between-horizontal-end) icon
    BetweenHorizontalEnd,
    ///[between-horizontal-start](https://lucide.dev/icons/between-horizontal-start) icon
    BetweenHorizontalStart,
    ///[between-vertical-end](https://lucide.dev/icons/between-vertical-end) icon
    BetweenVerticalEnd,
    ///[between-vertical-start](https://lucide.dev/icons/between-vertical-start) icon
    BetweenVerticalStart,
    ///[biceps-flexed](https://lucide.dev/icons/biceps-flexed) icon
    BicepsFlexed,
    ///[bike](https://lucide.dev/icons/bike) icon
    Bike,
    ///[binary](https://lucide.dev/icons/binary) icon
    Binary,
    ///[binoculars](https://lucide.dev/icons/binoculars) icon
    Binoculars,
    ///[biohazard](https://lucide.dev/icons/biohazard) icon
    Biohazard,
    ///[bird](https://lucide.dev/icons/bird) icon
    Bird,
    ///[bitcoin](https://lucide.dev/icons/bitcoin) icon
    Bitcoin,
    ///[blend](https://lucide.dev/icons/blend) icon
    Blend,
    ///[blinds](https://lucide.dev/icons/blinds) icon
    Blinds,
    ///[blocks](https://lucide.dev/icons/blocks) icon
    Blocks,
    ///[bluetooth](https://lucide.dev/icons/bluetooth) icon
    Bluetooth,
    ///[bluetooth-connected](https://lucide.dev/icons/bluetooth-connected) icon
    BluetoothConnected,
    ///[bluetooth-off](https://lucide.dev/icons/bluetooth-off) icon
    BluetoothOff,
    ///[bluetooth-searching](https://lucide.dev/icons/bluetooth-searching) icon
    BluetoothSearching,
    ///[bold](https://lucide.dev/icons/bold) icon
    Bold,
    ///[bolt](https://lucide.dev/icons/bolt) icon
    Bolt,
    ///[bomb](https://lucide.dev/icons/bomb) icon
    Bomb,
    ///[bone](https://lucide.dev/icons/bone) icon
    Bone,
    ///[book](https://lucide.dev/icons/book) icon
    Book,
    ///[book-a](https://lucide.dev/icons/book-a) icon
    BookA,
    ///[book-alert](https://lucide.dev/icons/book-alert) icon
    BookAlert,
    ///[book-audio](https://lucide.dev/icons/book-audio) icon
    BookAudio,
    ///[book-check](https://lucide.dev/icons/book-check) icon
    BookCheck,
    ///[book-copy](https://lucide.dev/icons/book-copy) icon
    BookCopy,
    ///[book-dashed](https://lucide.dev/icons/book-dashed) icon
    BookDashed,
    ///[book-down](https://lucide.dev/icons/book-down) icon
    BookDown,
    ///[book-headphones](https://lucide.dev/icons/book-headphones) icon
    BookHeadphones,
    ///[book-heart](https://lucide.dev/icons/book-heart) icon
    BookHeart,
    ///[book-image](https://lucide.dev/icons/book-image) icon
    BookImage,
    ///[book-key](https://lucide.dev/icons/book-key) icon
    BookKey,
    ///[book-lock](https://lucide.dev/icons/book-lock) icon
    BookLock,
    ///[book-marked](https://lucide.dev/icons/book-marked) icon
    BookMarked,
    ///[book-minus](https://lucide.dev/icons/book-minus) icon
    BookMinus,
    ///[book-open](https://lucide.dev/icons/book-open) icon
    BookOpen,
    ///[book-open-check](https://lucide.dev/icons/book-open-check) icon
    BookOpenCheck,
    ///[book-open-text](https://lucide.dev/icons/book-open-text) icon
    BookOpenText,
    ///[book-plus](https://lucide.dev/icons/book-plus) icon
    BookPlus,
    ///[book-text](https://lucide.dev/icons/book-text) icon
    BookText,
    ///[book-type](https://lucide.dev/icons/book-type) icon
    BookType,
    ///[book-up](https://lucide.dev/icons/book-up) icon
    BookUp,
    ///[book-up-2](https://lucide.dev/icons/book-up-2) icon
    BookUp2,
    ///[book-user](https://lucide.dev/icons/book-user) icon
    BookUser,
    ///[book-x](https://lucide.dev/icons/book-x) icon
    BookX,
    ///[bookmark](https://lucide.dev/icons/bookmark) icon
    Bookmark,
    ///[bookmark-check](https://lucide.dev/icons/bookmark-check) icon
    BookmarkCheck,
    ///[bookmark-minus](https://lucide.dev/icons/bookmark-minus) icon
    BookmarkMinus,
    ///[bookmark-plus](https://lucide.dev/icons/bookmark-plus) icon
    BookmarkPlus,
    ///[bookmark-x](https://lucide.dev/icons/bookmark-x) icon
    BookmarkX,
    ///[boom-box](https://lucide.dev/icons/boom-box) icon
    BoomBox,
    ///[bot](https://lucide.dev/icons/bot) icon
    Bot,
    ///[bot-message-square](https://lucide.dev/icons/bot-message-square) icon
    BotMessageSquare,
    ///[bot-off](https://lucide.dev/icons/bot-off) icon
    BotOff,
    ///[bottle-wine](https://lucide.dev/icons/bottle-wine) icon
    BottleWine,
    ///[bow-arrow](https://lucide.dev/icons/bow-arrow) icon
    BowArrow,
    ///[box](https://lucide.dev/icons/box) icon
    Box,
    ///[boxes](https://lucide.dev/icons/boxes) icon
    Boxes,
    ///[braces](https://lucide.dev/icons/braces) icon
    Braces,
    ///[brackets](https://lucide.dev/icons/brackets) icon
    Brackets,
    ///[brain](https://lucide.dev/icons/brain) icon
    Brain,
    ///[brain-circuit](https://lucide.dev/icons/brain-circuit) icon
    BrainCircuit,
    ///[brain-cog](https://lucide.dev/icons/brain-cog) icon
    BrainCog,
    ///[brick-wall](https://lucide.dev/icons/brick-wall) icon
    BrickWall,
    ///[brick-wall-fire](https://lucide.dev/icons/brick-wall-fire) icon
    BrickWallFire,
    ///[briefcase](https://lucide.dev/icons/briefcase) icon
    Briefcase,
    ///[briefcase-business](https://lucide.dev/icons/briefcase-business) icon
    BriefcaseBusiness,
    ///[briefcase-conveyor-belt](https://lucide.dev/icons/briefcase-conveyor-belt) icon
    BriefcaseConveyorBelt,
    ///[briefcase-medical](https://lucide.dev/icons/briefcase-medical) icon
    BriefcaseMedical,
    ///[bring-to-front](https://lucide.dev/icons/bring-to-front) icon
    BringToFront,
    ///[brush](https://lucide.dev/icons/brush) icon
    Brush,
    ///[brush-cleaning](https://lucide.dev/icons/brush-cleaning) icon
    BrushCleaning,
    ///[bubbles](https://lucide.dev/icons/bubbles) icon
    Bubbles,
    ///[bug](https://lucide.dev/icons/bug) icon
    Bug,
    ///[bug-off](https://lucide.dev/icons/bug-off) icon
    BugOff,
    ///[bug-play](https://lucide.dev/icons/bug-play) icon
    BugPlay,
    ///[building](https://lucide.dev/icons/building) icon
    Building,
    ///[building-2](https://lucide.dev/icons/building-2) icon
    Building2,
    ///[bus](https://lucide.dev/icons/bus) icon
    Bus,
    ///[bus-front](https://lucide.dev/icons/bus-front) icon
    BusFront,
    ///[cable](https://lucide.dev/icons/cable) icon
    Cable,
    ///[cable-car](https://lucide.dev/icons/cable-car) icon
    CableCar,
    ///[cake](https://lucide.dev/icons/cake) icon
    Cake,
    ///[cake-slice](https://lucide.dev/icons/cake-slice) icon
    CakeSlice,
    ///[calculator](https://lucide.dev/icons/calculator) icon
    Calculator,
    ///[calendar](https://lucide.dev/icons/calendar) icon
    Calendar,
    ///[calendar-1](https://lucide.dev/icons/calendar-1) icon
    Calendar1,
    ///[calendar-arrow-down](https://lucide.dev/icons/calendar-arrow-down) icon
    CalendarArrowDown,
    ///[calendar-arrow-up](https://lucide.dev/icons/calendar-arrow-up) icon
    CalendarArrowUp,
    ///[calendar-check](https://lucide.dev/icons/calendar-check) icon
    CalendarCheck,
    ///[calendar-check-2](https://lucide.dev/icons/calendar-check-2) icon
    CalendarCheck2,
    ///[calendar-clock](https://lucide.dev/icons/calendar-clock) icon
    CalendarClock,
    ///[calendar-cog](https://lucide.dev/icons/calendar-cog) icon
    CalendarCog,
    ///[calendar-days](https://lucide.dev/icons/calendar-days) icon
    CalendarDays,
    ///[calendar-fold](https://lucide.dev/icons/calendar-fold) icon
    CalendarFold,
    ///[calendar-heart](https://lucide.dev/icons/calendar-heart) icon
    CalendarHeart,
    ///[calendar-minus](https://lucide.dev/icons/calendar-minus) icon
    CalendarMinus,
    ///[calendar-minus-2](https://lucide.dev/icons/calendar-minus-2) icon
    CalendarMinus2,
    ///[calendar-off](https://lucide.dev/icons/calendar-off) icon
    CalendarOff,
    ///[calendar-plus](https://lucide.dev/icons/calendar-plus) icon
    CalendarPlus,
    ///[calendar-plus-2](https://lucide.dev/icons/calendar-plus-2) icon
    CalendarPlus2,
    ///[calendar-range](https://lucide.dev/icons/calendar-range) icon
    CalendarRange,
    ///[calendar-search](https://lucide.dev/icons/calendar-search) icon
    CalendarSearch,
    ///[calendar-sync](https://lucide.dev/icons/calendar-sync) icon
    CalendarSync,
    ///[calendar-x](https://lucide.dev/icons/calendar-x) icon
    CalendarX,
    ///[calendar-x-2](https://lucide.dev/icons/calendar-x-2) icon
    CalendarX2,
    ///[camera](https://lucide.dev/icons/camera) icon
    Camera,
    ///[camera-off](https://lucide.dev/icons/camera-off) icon
    CameraOff,
    ///[candy](https://lucide.dev/icons/candy) icon
    Candy,
    ///[candy-cane](https://lucide.dev/icons/candy-cane) icon
    CandyCane,
    ///[candy-off](https://lucide.dev/icons/candy-off) icon
    CandyOff,
    ///[cannabis](https://lucide.dev/icons/cannabis) icon
    Cannabis,
    ///[captions](https://lucide.dev/icons/captions) icon
    Captions,
    ///[captions-off](https://lucide.dev/icons/captions-off) icon
    CaptionsOff,
    ///[car](https://lucide.dev/icons/car) icon
    Car,
    ///[car-front](https://lucide.dev/icons/car-front) icon
    CarFront,
    ///[car-taxi-front](https://lucide.dev/icons/car-taxi-front) icon
    CarTaxiFront,
    ///[caravan](https://lucide.dev/icons/caravan) icon
    Caravan,
    ///[card-sim](https://lucide.dev/icons/card-sim) icon
    CardSim,
    ///[carrot](https://lucide.dev/icons/carrot) icon
    Carrot,
    ///[case-lower](https://lucide.dev/icons/case-lower) icon
    CaseLower,
    ///[case-sensitive](https://lucide.dev/icons/case-sensitive) icon
    CaseSensitive,
    ///[case-upper](https://lucide.dev/icons/case-upper) icon
    CaseUpper,
    ///[cassette-tape](https://lucide.dev/icons/cassette-tape) icon
    CassetteTape,
    ///[cast](https://lucide.dev/icons/cast) icon
    Cast,
    ///[castle](https://lucide.dev/icons/castle) icon
    Castle,
    ///[cat](https://lucide.dev/icons/cat) icon
    Cat,
    ///[cctv](https://lucide.dev/icons/cctv) icon
    Cctv,
    ///[chart-area](https://lucide.dev/icons/chart-area) icon
    ChartArea,
    ///[chart-bar](https://lucide.dev/icons/chart-bar) icon
    ChartBar,
    ///[chart-bar-big](https://lucide.dev/icons/chart-bar-big) icon
    ChartBarBig,
    ///[chart-bar-decreasing](https://lucide.dev/icons/chart-bar-decreasing) icon
    ChartBarDecreasing,
    ///[chart-bar-increasing](https://lucide.dev/icons/chart-bar-increasing) icon
    ChartBarIncreasing,
    ///[chart-bar-stacked](https://lucide.dev/icons/chart-bar-stacked) icon
    ChartBarStacked,
    ///[chart-candlestick](https://lucide.dev/icons/chart-candlestick) icon
    ChartCandlestick,
    ///[chart-column](https://lucide.dev/icons/chart-column) icon
    ChartColumn,
    ///[chart-column-big](https://lucide.dev/icons/chart-column-big) icon
    ChartColumnBig,
    ///[chart-column-decreasing](https://lucide.dev/icons/chart-column-decreasing) icon
    ChartColumnDecreasing,
    ///[chart-column-increasing](https://lucide.dev/icons/chart-column-increasing) icon
    ChartColumnIncreasing,
    ///[chart-column-stacked](https://lucide.dev/icons/chart-column-stacked) icon
    ChartColumnStacked,
    ///[chart-gantt](https://lucide.dev/icons/chart-gantt) icon
    ChartGantt,
    ///[chart-line](https://lucide.dev/icons/chart-line) icon
    ChartLine,
    ///[chart-network](https://lucide.dev/icons/chart-network) icon
    ChartNetwork,
    ///[chart-no-axes-column](https://lucide.dev/icons/chart-no-axes-column) icon
    ChartNoAxesColumn,
    ///[chart-no-axes-column-decreasing](https://lucide.dev/icons/chart-no-axes-column-decreasing) icon
    ChartNoAxesColumnDecreasing,
    ///[chart-no-axes-column-increasing](https://lucide.dev/icons/chart-no-axes-column-increasing) icon
    ChartNoAxesColumnIncreasing,
    ///[chart-no-axes-combined](https://lucide.dev/icons/chart-no-axes-combined) icon
    ChartNoAxesCombined,
    ///[chart-no-axes-gantt](https://lucide.dev/icons/chart-no-axes-gantt) icon
    ChartNoAxesGantt,
    ///[chart-pie](https://lucide.dev/icons/chart-pie) icon
    ChartPie,
    ///[chart-scatter](https://lucide.dev/icons/chart-scatter) icon
    ChartScatter,
    ///[chart-spline](https://lucide.dev/icons/chart-spline) icon
    ChartSpline,
    ///[check](https://lucide.dev/icons/check) icon
    Check,
    ///[check-check](https://lucide.dev/icons/check-check) icon
    CheckCheck,
    ///[check-line](https://lucide.dev/icons/check-line) icon
    CheckLine,
    ///[chef-hat](https://lucide.dev/icons/chef-hat) icon
    ChefHat,
    ///[cherry](https://lucide.dev/icons/cherry) icon
    Cherry,
    ///[chevron-down](https://lucide.dev/icons/chevron-down) icon
    ChevronDown,
    ///[chevron-first](https://lucide.dev/icons/chevron-first) icon
    ChevronFirst,
    ///[chevron-last](https://lucide.dev/icons/chevron-last) icon
    ChevronLast,
    ///[chevron-left](https://lucide.dev/icons/chevron-left) icon
    ChevronLeft,
    ///[chevron-right](https://lucide.dev/icons/chevron-right) icon
    ChevronRight,
    ///[chevron-up](https://lucide.dev/icons/chevron-up) icon
    ChevronUp,
    ///[chevrons-down](https://lucide.dev/icons/chevrons-down) icon
    ChevronsDown,
    ///[chevrons-down-up](https://lucide.dev/icons/chevrons-down-up) icon
    ChevronsDownUp,
    ///[chevrons-left](https://lucide.dev/icons/chevrons-left) icon
    ChevronsLeft,
    ///[chevrons-left-right](https://lucide.dev/icons/chevrons-left-right) icon
    ChevronsLeftRight,
    ///[chevrons-left-right-ellipsis](https://lucide.dev/icons/chevrons-left-right-ellipsis) icon
    ChevronsLeftRightEllipsis,
    ///[chevrons-right](https://lucide.dev/icons/chevrons-right) icon
    ChevronsRight,
    ///[chevrons-right-left](https://lucide.dev/icons/chevrons-right-left) icon
    ChevronsRightLeft,
    ///[chevrons-up](https://lucide.dev/icons/chevrons-up) icon
    ChevronsUp,
    ///[chevrons-up-down](https://lucide.dev/icons/chevrons-up-down) icon
    ChevronsUpDown,
    ///[chrome](https://lucide.dev/icons/chrome) icon
    Chrome,
    ///[church](https://lucide.dev/icons/church) icon
    Church,
    ///[cigarette](https://lucide.dev/icons/cigarette) icon
    Cigarette,
    ///[cigarette-off](https://lucide.dev/icons/cigarette-off) icon
    CigaretteOff,
    ///[circle](https://lucide.dev/icons/circle) icon
    Circle,
    ///[circle-alert](https://lucide.dev/icons/circle-alert) icon
    CircleAlert,
    ///[circle-arrow-down](https://lucide.dev/icons/circle-arrow-down) icon
    CircleArrowDown,
    ///[circle-arrow-left](https://lucide.dev/icons/circle-arrow-left) icon
    CircleArrowLeft,
    ///[circle-arrow-out-down-left](https://lucide.dev/icons/circle-arrow-out-down-left) icon
    CircleArrowOutDownLeft,
    ///[circle-arrow-out-down-right](https://lucide.dev/icons/circle-arrow-out-down-right) icon
    CircleArrowOutDownRight,
    ///[circle-arrow-out-up-left](https://lucide.dev/icons/circle-arrow-out-up-left) icon
    CircleArrowOutUpLeft,
    ///[circle-arrow-out-up-right](https://lucide.dev/icons/circle-arrow-out-up-right) icon
    CircleArrowOutUpRight,
    ///[circle-arrow-right](https://lucide.dev/icons/circle-arrow-right) icon
    CircleArrowRight,
    ///[circle-arrow-up](https://lucide.dev/icons/circle-arrow-up) icon
    CircleArrowUp,
    ///[circle-check](https://lucide.dev/icons/circle-check) icon
    CircleCheck,
    ///[circle-check-big](https://lucide.dev/icons/circle-check-big) icon
    CircleCheckBig,
    ///[circle-chevron-down](https://lucide.dev/icons/circle-chevron-down) icon
    CircleChevronDown,
    ///[circle-chevron-left](https://lucide.dev/icons/circle-chevron-left) icon
    CircleChevronLeft,
    ///[circle-chevron-right](https://lucide.dev/icons/circle-chevron-right) icon
    CircleChevronRight,
    ///[circle-chevron-up](https://lucide.dev/icons/circle-chevron-up) icon
    CircleChevronUp,
    ///[circle-dashed](https://lucide.dev/icons/circle-dashed) icon
    CircleDashed,
    ///[circle-divide](https://lucide.dev/icons/circle-divide) icon
    CircleDivide,
    ///[circle-dollar-sign](https://lucide.dev/icons/circle-dollar-sign) icon
    CircleDollarSign,
    ///[circle-dot](https://lucide.dev/icons/circle-dot) icon
    CircleDot,
    ///[circle-dot-dashed](https://lucide.dev/icons/circle-dot-dashed) icon
    CircleDotDashed,
    ///[circle-ellipsis](https://lucide.dev/icons/circle-ellipsis) icon
    CircleEllipsis,
    ///[circle-equal](https://lucide.dev/icons/circle-equal) icon
    CircleEqual,
    ///[circle-fading-arrow-up](https://lucide.dev/icons/circle-fading-arrow-up) icon
    CircleFadingArrowUp,
    ///[circle-fading-plus](https://lucide.dev/icons/circle-fading-plus) icon
    CircleFadingPlus,
    ///[circle-gauge](https://lucide.dev/icons/circle-gauge) icon
    CircleGauge,
    ///[circle-minus](https://lucide.dev/icons/circle-minus) icon
    CircleMinus,
    ///[circle-off](https://lucide.dev/icons/circle-off) icon
    CircleOff,
    ///[circle-parking](https://lucide.dev/icons/circle-parking) icon
    CircleParking,
    ///[circle-parking-off](https://lucide.dev/icons/circle-parking-off) icon
    CircleParkingOff,
    ///[circle-pause](https://lucide.dev/icons/circle-pause) icon
    CirclePause,
    ///[circle-percent](https://lucide.dev/icons/circle-percent) icon
    CirclePercent,
    ///[circle-play](https://lucide.dev/icons/circle-play) icon
    CirclePlay,
    ///[circle-plus](https://lucide.dev/icons/circle-plus) icon
    CirclePlus,
    ///[circle-pound-sterling](https://lucide.dev/icons/circle-pound-sterling) icon
    CirclePoundSterling,
    ///[circle-power](https://lucide.dev/icons/circle-power) icon
    CirclePower,
    ///[circle-question-mark](https://lucide.dev/icons/circle-question-mark) icon
    CircleQuestionMark,
    ///[circle-slash](https://lucide.dev/icons/circle-slash) icon
    CircleSlash,
    ///[circle-slash-2](https://lucide.dev/icons/circle-slash-2) icon
    CircleSlash2,
    ///[circle-small](https://lucide.dev/icons/circle-small) icon
    CircleSmall,
    ///[circle-stop](https://lucide.dev/icons/circle-stop) icon
    CircleStop,
    ///[circle-user](https://lucide.dev/icons/circle-user) icon
    CircleUser,
    ///[circle-user-round](https://lucide.dev/icons/circle-user-round) icon
    CircleUserRound,
    ///[circle-x](https://lucide.dev/icons/circle-x) icon
    CircleX,
    ///[circuit-board](https://lucide.dev/icons/circuit-board) icon
    CircuitBoard,
    ///[citrus](https://lucide.dev/icons/citrus) icon
    Citrus,
    ///[clapperboard](https://lucide.dev/icons/clapperboard) icon
    Clapperboard,
    ///[clipboard](https://lucide.dev/icons/clipboard) icon
    Clipboard,
    ///[clipboard-check](https://lucide.dev/icons/clipboard-check) icon
    ClipboardCheck,
    ///[clipboard-copy](https://lucide.dev/icons/clipboard-copy) icon
    ClipboardCopy,
    ///[clipboard-list](https://lucide.dev/icons/clipboard-list) icon
    ClipboardList,
    ///[clipboard-minus](https://lucide.dev/icons/clipboard-minus) icon
    ClipboardMinus,
    ///[clipboard-paste](https://lucide.dev/icons/clipboard-paste) icon
    ClipboardPaste,
    ///[clipboard-pen](https://lucide.dev/icons/clipboard-pen) icon
    ClipboardPen,
    ///[clipboard-pen-line](https://lucide.dev/icons/clipboard-pen-line) icon
    ClipboardPenLine,
    ///[clipboard-plus](https://lucide.dev/icons/clipboard-plus) icon
    ClipboardPlus,
    ///[clipboard-type](https://lucide.dev/icons/clipboard-type) icon
    ClipboardType,
    ///[clipboard-x](https://lucide.dev/icons/clipboard-x) icon
    ClipboardX,
    ///[clock](https://lucide.dev/icons/clock) icon
    Clock,
    ///[clock-1](https://lucide.dev/icons/clock-1) icon
    Clock1,
    ///[clock-10](https://lucide.dev/icons/clock-10) icon
    Clock10,
    ///[clock-11](https://lucide.dev/icons/clock-11) icon
    Clock11,
    ///[clock-12](https://lucide.dev/icons/clock-12) icon
    Clock12,
    ///[clock-2](https://lucide.dev/icons/clock-2) icon
    Clock2,
    ///[clock-3](https://lucide.dev/icons/clock-3) icon
    Clock3,
    ///[clock-4](https://lucide.dev/icons/clock-4) icon
    Clock4,
    ///[clock-5](https://lucide.dev/icons/clock-5) icon
    Clock5,
    ///[clock-6](https://lucide.dev/icons/clock-6) icon
    Clock6,
    ///[clock-7](https://lucide.dev/icons/clock-7) icon
    Clock7,
    ///[clock-8](https://lucide.dev/icons/clock-8) icon
    Clock8,
    ///[clock-9](https://lucide.dev/icons/clock-9) icon
    Clock9,
    ///[clock-alert](https://lucide.dev/icons/clock-alert) icon
    ClockAlert,
    ///[clock-arrow-down](https://lucide.dev/icons/clock-arrow-down) icon
    ClockArrowDown,
    ///[clock-arrow-up](https://lucide.dev/icons/clock-arrow-up) icon
    ClockArrowUp,
    ///[clock-fading](https://lucide.dev/icons/clock-fading) icon
    ClockFading,
    ///[clock-plus](https://lucide.dev/icons/clock-plus) icon
    ClockPlus,
    ///[cloud](https://lucide.dev/icons/cloud) icon
    Cloud,
    ///[cloud-alert](https://lucide.dev/icons/cloud-alert) icon
    CloudAlert,
    ///[cloud-check](https://lucide.dev/icons/cloud-check) icon
    CloudCheck,
    ///[cloud-cog](https://lucide.dev/icons/cloud-cog) icon
    CloudCog,
    ///[cloud-download](https://lucide.dev/icons/cloud-download) icon
    CloudDownload,
    ///[cloud-drizzle](https://lucide.dev/icons/cloud-drizzle) icon
    CloudDrizzle,
    ///[cloud-fog](https://lucide.dev/icons/cloud-fog) icon
    CloudFog,
    ///[cloud-hail](https://lucide.dev/icons/cloud-hail) icon
    CloudHail,
    ///[cloud-lightning](https://lucide.dev/icons/cloud-lightning) icon
    CloudLightning,
    ///[cloud-moon](https://lucide.dev/icons/cloud-moon) icon
    CloudMoon,
    ///[cloud-moon-rain](https://lucide.dev/icons/cloud-moon-rain) icon
    CloudMoonRain,
    ///[cloud-off](https://lucide.dev/icons/cloud-off) icon
    CloudOff,
    ///[cloud-rain](https://lucide.dev/icons/cloud-rain) icon
    CloudRain,
    ///[cloud-rain-wind](https://lucide.dev/icons/cloud-rain-wind) icon
    CloudRainWind,
    ///[cloud-snow](https://lucide.dev/icons/cloud-snow) icon
    CloudSnow,
    ///[cloud-sun](https://lucide.dev/icons/cloud-sun) icon
    CloudSun,
    ///[cloud-sun-rain](https://lucide.dev/icons/cloud-sun-rain) icon
    CloudSunRain,
    ///[cloud-upload](https://lucide.dev/icons/cloud-upload) icon
    CloudUpload,
    ///[cloudy](https://lucide.dev/icons/cloudy) icon
    Cloudy,
    ///[clover](https://lucide.dev/icons/clover) icon
    Clover,
    ///[club](https://lucide.dev/icons/club) icon
    Club,
    ///[code](https://lucide.dev/icons/code) icon
    Code,
    ///[code-xml](https://lucide.dev/icons/code-xml) icon
    CodeXml,
    ///[codepen](https://lucide.dev/icons/codepen) icon
    Codepen,
    ///[codesandbox](https://lucide.dev/icons/codesandbox) icon
    Codesandbox,
    ///[coffee](https://lucide.dev/icons/coffee) icon
    Coffee,
    ///[cog](https://lucide.dev/icons/cog) icon
    Cog,
    ///[coins](https://lucide.dev/icons/coins) icon
    Coins,
    ///[columns-2](https://lucide.dev/icons/columns-2) icon
    Columns2,
    ///[columns-3](https://lucide.dev/icons/columns-3) icon
    Columns3,
    ///[columns-3-cog](https://lucide.dev/icons/columns-3-cog) icon
    Columns3Cog,
    ///[columns-4](https://lucide.dev/icons/columns-4) icon
    Columns4,
    ///[combine](https://lucide.dev/icons/combine) icon
    Combine,
    ///[command](https://lucide.dev/icons/command) icon
    Command,
    ///[compass](https://lucide.dev/icons/compass) icon
    Compass,
    ///[component](https://lucide.dev/icons/component) icon
    Component,
    ///[computer](https://lucide.dev/icons/computer) icon
    Computer,
    ///[concierge-bell](https://lucide.dev/icons/concierge-bell) icon
    ConciergeBell,
    ///[cone](https://lucide.dev/icons/cone) icon
    Cone,
    ///[construction](https://lucide.dev/icons/construction) icon
    Construction,
    ///[contact](https://lucide.dev/icons/contact) icon
    Contact,
    ///[contact-round](https://lucide.dev/icons/contact-round) icon
    ContactRound,
    ///[container](https://lucide.dev/icons/container) icon
    Container,
    ///[contrast](https://lucide.dev/icons/contrast) icon
    Contrast,
    ///[cookie](https://lucide.dev/icons/cookie) icon
    Cookie,
    ///[cooking-pot](https://lucide.dev/icons/cooking-pot) icon
    CookingPot,
    ///[copy](https://lucide.dev/icons/copy) icon
    Copy,
    ///[copy-check](https://lucide.dev/icons/copy-check) icon
    CopyCheck,
    ///[copy-minus](https://lucide.dev/icons/copy-minus) icon
    CopyMinus,
    ///[copy-plus](https://lucide.dev/icons/copy-plus) icon
    CopyPlus,
    ///[copy-slash](https://lucide.dev/icons/copy-slash) icon
    CopySlash,
    ///[copy-x](https://lucide.dev/icons/copy-x) icon
    CopyX,
    ///[copyleft](https://lucide.dev/icons/copyleft) icon
    Copyleft,
    ///[copyright](https://lucide.dev/icons/copyright) icon
    Copyright,
    ///[corner-down-left](https://lucide.dev/icons/corner-down-left) icon
    CornerDownLeft,
    ///[corner-down-right](https://lucide.dev/icons/corner-down-right) icon
    CornerDownRight,
    ///[corner-left-down](https://lucide.dev/icons/corner-left-down) icon
    CornerLeftDown,
    ///[corner-left-up](https://lucide.dev/icons/corner-left-up) icon
    CornerLeftUp,
    ///[corner-right-down](https://lucide.dev/icons/corner-right-down) icon
    CornerRightDown,
    ///[corner-right-up](https://lucide.dev/icons/corner-right-up) icon
    CornerRightUp,
    ///[corner-up-left](https://lucide.dev/icons/corner-up-left) icon
    CornerUpLeft,
    ///[corner-up-right](https://lucide.dev/icons/corner-up-right) icon
    CornerUpRight,
    ///[cpu](https://lucide.dev/icons/cpu) icon
    Cpu,
    ///[creative-commons](https://lucide.dev/icons/creative-commons) icon
    CreativeCommons,
    ///[credit-card](https://lucide.dev/icons/credit-card) icon
    CreditCard,
    ///[croissant](https://lucide.dev/icons/croissant) icon
    Croissant,
    ///[crop](https://lucide.dev/icons/crop) icon
    Crop,
    ///[cross](https://lucide.dev/icons/cross) icon
    Cross,
    ///[crosshair](https://lucide.dev/icons/crosshair) icon
    Crosshair,
    ///[crown](https://lucide.dev/icons/crown) icon
    Crown,
    ///[cuboid](https://lucide.dev/icons/cuboid) icon
    Cuboid,
    ///[cup-soda](https://lucide.dev/icons/cup-soda) icon
    CupSoda,
    ///[currency](https://lucide.dev/icons/currency) icon
    Currency,
    ///[cylinder](https://lucide.dev/icons/cylinder) icon
    Cylinder,
    ///[dam](https://lucide.dev/icons/dam) icon
    Dam,
    ///[database](https://lucide.dev/icons/database) icon
    Database,
    ///[database-backup](https://lucide.dev/icons/database-backup) icon
    DatabaseBackup,
    ///[database-zap](https://lucide.dev/icons/database-zap) icon
    DatabaseZap,
    ///[decimals-arrow-left](https://lucide.dev/icons/decimals-arrow-left) icon
    DecimalsArrowLeft,
    ///[decimals-arrow-right](https://lucide.dev/icons/decimals-arrow-right) icon
    DecimalsArrowRight,
    ///[delete](https://lucide.dev/icons/delete) icon
    Delete,
    ///[dessert](https://lucide.dev/icons/dessert) icon
    Dessert,
    ///[diameter](https://lucide.dev/icons/diameter) icon
    Diameter,
    ///[diamond](https://lucide.dev/icons/diamond) icon
    Diamond,
    ///[diamond-minus](https://lucide.dev/icons/diamond-minus) icon
    DiamondMinus,
    ///[diamond-percent](https://lucide.dev/icons/diamond-percent) icon
    DiamondPercent,
    ///[diamond-plus](https://lucide.dev/icons/diamond-plus) icon
    DiamondPlus,
    ///[dice-1](https://lucide.dev/icons/dice-1) icon
    Dice1,
    ///[dice-2](https://lucide.dev/icons/dice-2) icon
    Dice2,
    ///[dice-3](https://lucide.dev/icons/dice-3) icon
    Dice3,
    ///[dice-4](https://lucide.dev/icons/dice-4) icon
    Dice4,
    ///[dice-5](https://lucide.dev/icons/dice-5) icon
    Dice5,
    ///[dice-6](https://lucide.dev/icons/dice-6) icon
    Dice6,
    ///[dices](https://lucide.dev/icons/dices) icon
    Dices,
    ///[diff](https://lucide.dev/icons/diff) icon
    Diff,
    ///[disc](https://lucide.dev/icons/disc) icon
    Disc,
    ///[disc-2](https://lucide.dev/icons/disc-2) icon
    Disc2,
    ///[disc-3](https://lucide.dev/icons/disc-3) icon
    Disc3,
    ///[disc-album](https://lucide.dev/icons/disc-album) icon
    DiscAlbum,
    ///[divide](https://lucide.dev/icons/divide) icon
    Divide,
    ///[dna](https://lucide.dev/icons/dna) icon
    Dna,
    ///[dna-off](https://lucide.dev/icons/dna-off) icon
    DnaOff,
    ///[dock](https://lucide.dev/icons/dock) icon
    Dock,
    ///[dog](https://lucide.dev/icons/dog) icon
    Dog,
    ///[dollar-sign](https://lucide.dev/icons/dollar-sign) icon
    DollarSign,
    ///[donut](https://lucide.dev/icons/donut) icon
    Donut,
    ///[door-closed](https://lucide.dev/icons/door-closed) icon
    DoorClosed,
    ///[door-closed-locked](https://lucide.dev/icons/door-closed-locked) icon
    DoorClosedLocked,
    ///[door-open](https://lucide.dev/icons/door-open) icon
    DoorOpen,
    ///[dot](https://lucide.dev/icons/dot) icon
    Dot,
    ///[download](https://lucide.dev/icons/download) icon
    Download,
    ///[drafting-compass](https://lucide.dev/icons/drafting-compass) icon
    DraftingCompass,
    ///[drama](https://lucide.dev/icons/drama) icon
    Drama,
    ///[dribbble](https://lucide.dev/icons/dribbble) icon
    Dribbble,
    ///[drill](https://lucide.dev/icons/drill) icon
    Drill,
    ///[drone](https://lucide.dev/icons/drone) icon
    Drone,
    ///[droplet](https://lucide.dev/icons/droplet) icon
    Droplet,
    ///[droplet-off](https://lucide.dev/icons/droplet-off) icon
    DropletOff,
    ///[droplets](https://lucide.dev/icons/droplets) icon
    Droplets,
    ///[drum](https://lucide.dev/icons/drum) icon
    Drum,
    ///[drumstick](https://lucide.dev/icons/drumstick) icon
    Drumstick,
    ///[dumbbell](https://lucide.dev/icons/dumbbell) icon
    Dumbbell,
    ///[ear](https://lucide.dev/icons/ear) icon
    Ear,
    ///[ear-off](https://lucide.dev/icons/ear-off) icon
    EarOff,
    ///[earth](https://lucide.dev/icons/earth) icon
    Earth,
    ///[earth-lock](https://lucide.dev/icons/earth-lock) icon
    EarthLock,
    ///[eclipse](https://lucide.dev/icons/eclipse) icon
    Eclipse,
    ///[egg](https://lucide.dev/icons/egg) icon
    Egg,
    ///[egg-fried](https://lucide.dev/icons/egg-fried) icon
    EggFried,
    ///[egg-off](https://lucide.dev/icons/egg-off) icon
    EggOff,
    ///[ellipsis](https://lucide.dev/icons/ellipsis) icon
    Ellipsis,
    ///[ellipsis-vertical](https://lucide.dev/icons/ellipsis-vertical) icon
    EllipsisVertical,
    ///[equal](https://lucide.dev/icons/equal) icon
    Equal,
    ///[equal-approximately](https://lucide.dev/icons/equal-approximately) icon
    EqualApproximately,
    ///[equal-not](https://lucide.dev/icons/equal-not) icon
    EqualNot,
    ///[eraser](https://lucide.dev/icons/eraser) icon
    Eraser,
    ///[ethernet-port](https://lucide.dev/icons/ethernet-port) icon
    EthernetPort,
    ///[euro](https://lucide.dev/icons/euro) icon
    Euro,
    ///[expand](https://lucide.dev/icons/expand) icon
    Expand,
    ///[external-link](https://lucide.dev/icons/external-link) icon
    ExternalLink,
    ///[eye](https://lucide.dev/icons/eye) icon
    Eye,
    ///[eye-closed](https://lucide.dev/icons/eye-closed) icon
    EyeClosed,
    ///[eye-off](https://lucide.dev/icons/eye-off) icon
    EyeOff,
    ///[facebook](https://lucide.dev/icons/facebook) icon
    Facebook,
    ///[factory](https://lucide.dev/icons/factory) icon
    Factory,
    ///[fan](https://lucide.dev/icons/fan) icon
    Fan,
    ///[fast-forward](https://lucide.dev/icons/fast-forward) icon
    FastForward,
    ///[feather](https://lucide.dev/icons/feather) icon
    Feather,
    ///[fence](https://lucide.dev/icons/fence) icon
    Fence,
    ///[ferris-wheel](https://lucide.dev/icons/ferris-wheel) icon
    FerrisWheel,
    ///[figma](https://lucide.dev/icons/figma) icon
    Figma,
    ///[file](https://lucide.dev/icons/file) icon
    File,
    ///[file-archive](https://lucide.dev/icons/file-archive) icon
    FileArchive,
    ///[file-audio](https://lucide.dev/icons/file-audio) icon
    FileAudio,
    ///[file-audio-2](https://lucide.dev/icons/file-audio-2) icon
    FileAudio2,
    ///[file-axis-3d](https://lucide.dev/icons/file-axis-3d) icon
    FileAxis3d,
    ///[file-badge](https://lucide.dev/icons/file-badge) icon
    FileBadge,
    ///[file-badge-2](https://lucide.dev/icons/file-badge-2) icon
    FileBadge2,
    ///[file-box](https://lucide.dev/icons/file-box) icon
    FileBox,
    ///[file-chart-column](https://lucide.dev/icons/file-chart-column) icon
    FileChartColumn,
    ///[file-chart-column-increasing](https://lucide.dev/icons/file-chart-column-increasing) icon
    FileChartColumnIncreasing,
    ///[file-chart-line](https://lucide.dev/icons/file-chart-line) icon
    FileChartLine,
    ///[file-chart-pie](https://lucide.dev/icons/file-chart-pie) icon
    FileChartPie,
    ///[file-check](https://lucide.dev/icons/file-check) icon
    FileCheck,
    ///[file-check-2](https://lucide.dev/icons/file-check-2) icon
    FileCheck2,
    ///[file-clock](https://lucide.dev/icons/file-clock) icon
    FileClock,
    ///[file-code](https://lucide.dev/icons/file-code) icon
    FileCode,
    ///[file-code-2](https://lucide.dev/icons/file-code-2) icon
    FileCode2,
    ///[file-cog](https://lucide.dev/icons/file-cog) icon
    FileCog,
    ///[file-diff](https://lucide.dev/icons/file-diff) icon
    FileDiff,
    ///[file-digit](https://lucide.dev/icons/file-digit) icon
    FileDigit,
    ///[file-down](https://lucide.dev/icons/file-down) icon
    FileDown,
    ///[file-heart](https://lucide.dev/icons/file-heart) icon
    FileHeart,
    ///[file-image](https://lucide.dev/icons/file-image) icon
    FileImage,
    ///[file-input](https://lucide.dev/icons/file-input) icon
    FileInput,
    ///[file-json](https://lucide.dev/icons/file-json) icon
    FileJson,
    ///[file-json-2](https://lucide.dev/icons/file-json-2) icon
    FileJson2,
    ///[file-key](https://lucide.dev/icons/file-key) icon
    FileKey,
    ///[file-key-2](https://lucide.dev/icons/file-key-2) icon
    FileKey2,
    ///[file-lock](https://lucide.dev/icons/file-lock) icon
    FileLock,
    ///[file-lock-2](https://lucide.dev/icons/file-lock-2) icon
    FileLock2,
    ///[file-minus](https://lucide.dev/icons/file-minus) icon
    FileMinus,
    ///[file-minus-2](https://lucide.dev/icons/file-minus-2) icon
    FileMinus2,
    ///[file-music](https://lucide.dev/icons/file-music) icon
    FileMusic,
    ///[file-output](https://lucide.dev/icons/file-output) icon
    FileOutput,
    ///[file-pen](https://lucide.dev/icons/file-pen) icon
    FilePen,
    ///[file-pen-line](https://lucide.dev/icons/file-pen-line) icon
    FilePenLine,
    ///[file-plus](https://lucide.dev/icons/file-plus) icon
    FilePlus,
    ///[file-plus-2](https://lucide.dev/icons/file-plus-2) icon
    FilePlus2,
    ///[file-question-mark](https://lucide.dev/icons/file-question-mark) icon
    FileQuestionMark,
    ///[file-scan](https://lucide.dev/icons/file-scan) icon
    FileScan,
    ///[file-search](https://lucide.dev/icons/file-search) icon
    FileSearch,
    ///[file-search-2](https://lucide.dev/icons/file-search-2) icon
    FileSearch2,
    ///[file-sliders](https://lucide.dev/icons/file-sliders) icon
    FileSliders,
    ///[file-spreadsheet](https://lucide.dev/icons/file-spreadsheet) icon
    FileSpreadsheet,
    ///[file-stack](https://lucide.dev/icons/file-stack) icon
    FileStack,
    ///[file-symlink](https://lucide.dev/icons/file-symlink) icon
    FileSymlink,
    ///[file-terminal](https://lucide.dev/icons/file-terminal) icon
    FileTerminal,
    ///[file-text](https://lucide.dev/icons/file-text) icon
    FileText,
    ///[file-type](https://lucide.dev/icons/file-type) icon
    FileType,
    ///[file-type-2](https://lucide.dev/icons/file-type-2) icon
    FileType2,
    ///[file-up](https://lucide.dev/icons/file-up) icon
    FileUp,
    ///[file-user](https://lucide.dev/icons/file-user) icon
    FileUser,
    ///[file-video](https://lucide.dev/icons/file-video) icon
    FileVideo,
    ///[file-video-2](https://lucide.dev/icons/file-video-2) icon
    FileVideo2,
    ///[file-volume](https://lucide.dev/icons/file-volume) icon
    FileVolume,
    ///[file-volume-2](https://lucide.dev/icons/file-volume-2) icon
    FileVolume2,
    ///[file-warning](https://lucide.dev/icons/file-warning) icon
    FileWarning,
    ///[file-x](https://lucide.dev/icons/file-x) icon
    FileX,
    ///[file-x-2](https://lucide.dev/icons/file-x-2) icon
    FileX2,
    ///[files](https://lucide.dev/icons/files) icon
    Files,
    ///[film](https://lucide.dev/icons/film) icon
    Film,
    ///[fingerprint](https://lucide.dev/icons/fingerprint) icon
    Fingerprint,
    ///[fire-extinguisher](https://lucide.dev/icons/fire-extinguisher) icon
    FireExtinguisher,
    ///[fish](https://lucide.dev/icons/fish) icon
    Fish,
    ///[fish-off](https://lucide.dev/icons/fish-off) icon
    FishOff,
    ///[fish-symbol](https://lucide.dev/icons/fish-symbol) icon
    FishSymbol,
    ///[flag](https://lucide.dev/icons/flag) icon
    Flag,
    ///[flag-off](https://lucide.dev/icons/flag-off) icon
    FlagOff,
    ///[flag-triangle-left](https://lucide.dev/icons/flag-triangle-left) icon
    FlagTriangleLeft,
    ///[flag-triangle-right](https://lucide.dev/icons/flag-triangle-right) icon
    FlagTriangleRight,
    ///[flame](https://lucide.dev/icons/flame) icon
    Flame,
    ///[flame-kindling](https://lucide.dev/icons/flame-kindling) icon
    FlameKindling,
    ///[flashlight](https://lucide.dev/icons/flashlight) icon
    Flashlight,
    ///[flashlight-off](https://lucide.dev/icons/flashlight-off) icon
    FlashlightOff,
    ///[flask-conical](https://lucide.dev/icons/flask-conical) icon
    FlaskConical,
    ///[flask-conical-off](https://lucide.dev/icons/flask-conical-off) icon
    FlaskConicalOff,
    ///[flask-round](https://lucide.dev/icons/flask-round) icon
    FlaskRound,
    ///[flip-horizontal](https://lucide.dev/icons/flip-horizontal) icon
    FlipHorizontal,
    ///[flip-horizontal-2](https://lucide.dev/icons/flip-horizontal-2) icon
    FlipHorizontal2,
    ///[flip-vertical](https://lucide.dev/icons/flip-vertical) icon
    FlipVertical,
    ///[flip-vertical-2](https://lucide.dev/icons/flip-vertical-2) icon
    FlipVertical2,
    ///[flower](https://lucide.dev/icons/flower) icon
    Flower,
    ///[flower-2](https://lucide.dev/icons/flower-2) icon
    Flower2,
    ///[focus](https://lucide.dev/icons/focus) icon
    Focus,
    ///[fold-horizontal](https://lucide.dev/icons/fold-horizontal) icon
    FoldHorizontal,
    ///[fold-vertical](https://lucide.dev/icons/fold-vertical) icon
    FoldVertical,
    ///[folder](https://lucide.dev/icons/folder) icon
    Folder,
    ///[folder-archive](https://lucide.dev/icons/folder-archive) icon
    FolderArchive,
    ///[folder-check](https://lucide.dev/icons/folder-check) icon
    FolderCheck,
    ///[folder-clock](https://lucide.dev/icons/folder-clock) icon
    FolderClock,
    ///[folder-closed](https://lucide.dev/icons/folder-closed) icon
    FolderClosed,
    ///[folder-code](https://lucide.dev/icons/folder-code) icon
    FolderCode,
    ///[folder-cog](https://lucide.dev/icons/folder-cog) icon
    FolderCog,
    ///[folder-dot](https://lucide.dev/icons/folder-dot) icon
    FolderDot,
    ///[folder-down](https://lucide.dev/icons/folder-down) icon
    FolderDown,
    ///[folder-git](https://lucide.dev/icons/folder-git) icon
    FolderGit,
    ///[folder-git-2](https://lucide.dev/icons/folder-git-2) icon
    FolderGit2,
    ///[folder-heart](https://lucide.dev/icons/folder-heart) icon
    FolderHeart,
    ///[folder-input](https://lucide.dev/icons/folder-input) icon
    FolderInput,
    ///[folder-kanban](https://lucide.dev/icons/folder-kanban) icon
    FolderKanban,
    ///[folder-key](https://lucide.dev/icons/folder-key) icon
    FolderKey,
    ///[folder-lock](https://lucide.dev/icons/folder-lock) icon
    FolderLock,
    ///[folder-minus](https://lucide.dev/icons/folder-minus) icon
    FolderMinus,
    ///[folder-open](https://lucide.dev/icons/folder-open) icon
    FolderOpen,
    ///[folder-open-dot](https://lucide.dev/icons/folder-open-dot) icon
    FolderOpenDot,
    ///[folder-output](https://lucide.dev/icons/folder-output) icon
    FolderOutput,
    ///[folder-pen](https://lucide.dev/icons/folder-pen) icon
    FolderPen,
    ///[folder-plus](https://lucide.dev/icons/folder-plus) icon
    FolderPlus,
    ///[folder-root](https://lucide.dev/icons/folder-root) icon
    FolderRoot,
    ///[folder-search](https://lucide.dev/icons/folder-search) icon
    FolderSearch,
    ///[folder-search-2](https://lucide.dev/icons/folder-search-2) icon
    FolderSearch2,
    ///[folder-symlink](https://lucide.dev/icons/folder-symlink) icon
    FolderSymlink,
    ///[folder-sync](https://lucide.dev/icons/folder-sync) icon
    FolderSync,
    ///[folder-tree](https://lucide.dev/icons/folder-tree) icon
    FolderTree,
    ///[folder-up](https://lucide.dev/icons/folder-up) icon
    FolderUp,
    ///[folder-x](https://lucide.dev/icons/folder-x) icon
    FolderX,
    ///[folders](https://lucide.dev/icons/folders) icon
    Folders,
    ///[footprints](https://lucide.dev/icons/footprints) icon
    Footprints,
    ///[forklift](https://lucide.dev/icons/forklift) icon
    Forklift,
    ///[forward](https://lucide.dev/icons/forward) icon
    Forward,
    ///[frame](https://lucide.dev/icons/frame) icon
    Frame,
    ///[framer](https://lucide.dev/icons/framer) icon
    Framer,
    ///[frown](https://lucide.dev/icons/frown) icon
    Frown,
    ///[fuel](https://lucide.dev/icons/fuel) icon
    Fuel,
    ///[fullscreen](https://lucide.dev/icons/fullscreen) icon
    Fullscreen,
    ///[funnel](https://lucide.dev/icons/funnel) icon
    Funnel,
    ///[funnel-plus](https://lucide.dev/icons/funnel-plus) icon
    FunnelPlus,
    ///[funnel-x](https://lucide.dev/icons/funnel-x) icon
    FunnelX,
    ///[gallery-horizontal](https://lucide.dev/icons/gallery-horizontal) icon
    GalleryHorizontal,
    ///[gallery-horizontal-end](https://lucide.dev/icons/gallery-horizontal-end) icon
    GalleryHorizontalEnd,
    ///[gallery-thumbnails](https://lucide.dev/icons/gallery-thumbnails) icon
    GalleryThumbnails,
    ///[gallery-vertical](https://lucide.dev/icons/gallery-vertical) icon
    GalleryVertical,
    ///[gallery-vertical-end](https://lucide.dev/icons/gallery-vertical-end) icon
    GalleryVerticalEnd,
    ///[gamepad](https://lucide.dev/icons/gamepad) icon
    Gamepad,
    ///[gamepad-2](https://lucide.dev/icons/gamepad-2) icon
    Gamepad2,
    ///[gauge](https://lucide.dev/icons/gauge) icon
    Gauge,
    ///[gavel](https://lucide.dev/icons/gavel) icon
    Gavel,
    ///[gem](https://lucide.dev/icons/gem) icon
    Gem,
    ///[georgian-lari](https://lucide.dev/icons/georgian-lari) icon
    GeorgianLari,
    ///[ghost](https://lucide.dev/icons/ghost) icon
    Ghost,
    ///[gift](https://lucide.dev/icons/gift) icon
    Gift,
    ///[git-branch](https://lucide.dev/icons/git-branch) icon
    GitBranch,
    ///[git-branch-plus](https://lucide.dev/icons/git-branch-plus) icon
    GitBranchPlus,
    ///[git-commit-horizontal](https://lucide.dev/icons/git-commit-horizontal) icon
    GitCommitHorizontal,
    ///[git-commit-vertical](https://lucide.dev/icons/git-commit-vertical) icon
    GitCommitVertical,
    ///[git-compare](https://lucide.dev/icons/git-compare) icon
    GitCompare,
    ///[git-compare-arrows](https://lucide.dev/icons/git-compare-arrows) icon
    GitCompareArrows,
    ///[git-fork](https://lucide.dev/icons/git-fork) icon
    GitFork,
    ///[git-graph](https://lucide.dev/icons/git-graph) icon
    GitGraph,
    ///[git-merge](https://lucide.dev/icons/git-merge) icon
    GitMerge,
    ///[git-pull-request](https://lucide.dev/icons/git-pull-request) icon
    GitPullRequest,
    ///[git-pull-request-arrow](https://lucide.dev/icons/git-pull-request-arrow) icon
    GitPullRequestArrow,
    ///[git-pull-request-closed](https://lucide.dev/icons/git-pull-request-closed) icon
    GitPullRequestClosed,
    ///[git-pull-request-create](https://lucide.dev/icons/git-pull-request-create) icon
    GitPullRequestCreate,
    ///[git-pull-request-create-arrow](https://lucide.dev/icons/git-pull-request-create-arrow) icon
    GitPullRequestCreateArrow,
    ///[git-pull-request-draft](https://lucide.dev/icons/git-pull-request-draft) icon
    GitPullRequestDraft,
    ///[github](https://lucide.dev/icons/github) icon
    Github,
    ///[gitlab](https://lucide.dev/icons/gitlab) icon
    Gitlab,
    ///[glass-water](https://lucide.dev/icons/glass-water) icon
    GlassWater,
    ///[glasses](https://lucide.dev/icons/glasses) icon
    Glasses,
    ///[globe](https://lucide.dev/icons/globe) icon
    Globe,
    ///[globe-lock](https://lucide.dev/icons/globe-lock) icon
    GlobeLock,
    ///[goal](https://lucide.dev/icons/goal) icon
    Goal,
    ///[gpu](https://lucide.dev/icons/gpu) icon
    Gpu,
    ///[grab](https://lucide.dev/icons/grab) icon
    Grab,
    ///[graduation-cap](https://lucide.dev/icons/graduation-cap) icon
    GraduationCap,
    ///[grape](https://lucide.dev/icons/grape) icon
    Grape,
    ///[grid-2x2](https://lucide.dev/icons/grid-2x2) icon
    Grid2x2,
    ///[grid-2x2-check](https://lucide.dev/icons/grid-2x2-check) icon
    Grid2x2Check,
    ///[grid-2x2-plus](https://lucide.dev/icons/grid-2x2-plus) icon
    Grid2x2Plus,
    ///[grid-2x2-x](https://lucide.dev/icons/grid-2x2-x) icon
    Grid2x2X,
    ///[grid-3x2](https://lucide.dev/icons/grid-3x2) icon
    Grid3x2,
    ///[grid-3x3](https://lucide.dev/icons/grid-3x3) icon
    Grid3x3,
    ///[grip](https://lucide.dev/icons/grip) icon
    Grip,
    ///[grip-horizontal](https://lucide.dev/icons/grip-horizontal) icon
    GripHorizontal,
    ///[grip-vertical](https://lucide.dev/icons/grip-vertical) icon
    GripVertical,
    ///[group](https://lucide.dev/icons/group) icon
    Group,
    ///[guitar](https://lucide.dev/icons/guitar) icon
    Guitar,
    ///[ham](https://lucide.dev/icons/ham) icon
    Ham,
    ///[hamburger](https://lucide.dev/icons/hamburger) icon
    Hamburger,
    ///[hammer](https://lucide.dev/icons/hammer) icon
    Hammer,
    ///[hand](https://lucide.dev/icons/hand) icon
    Hand,
    ///[hand-coins](https://lucide.dev/icons/hand-coins) icon
    HandCoins,
    ///[hand-heart](https://lucide.dev/icons/hand-heart) icon
    HandHeart,
    ///[hand-helping](https://lucide.dev/icons/hand-helping) icon
    HandHelping,
    ///[hand-metal](https://lucide.dev/icons/hand-metal) icon
    HandMetal,
    ///[hand-platter](https://lucide.dev/icons/hand-platter) icon
    HandPlatter,
    ///[handshake](https://lucide.dev/icons/handshake) icon
    Handshake,
    ///[hard-drive](https://lucide.dev/icons/hard-drive) icon
    HardDrive,
    ///[hard-drive-download](https://lucide.dev/icons/hard-drive-download) icon
    HardDriveDownload,
    ///[hard-drive-upload](https://lucide.dev/icons/hard-drive-upload) icon
    HardDriveUpload,
    ///[hard-hat](https://lucide.dev/icons/hard-hat) icon
    HardHat,
    ///[hash](https://lucide.dev/icons/hash) icon
    Hash,
    ///[haze](https://lucide.dev/icons/haze) icon
    Haze,
    ///[hdmi-port](https://lucide.dev/icons/hdmi-port) icon
    HdmiPort,
    ///[heading](https://lucide.dev/icons/heading) icon
    Heading,
    ///[heading-1](https://lucide.dev/icons/heading-1) icon
    Heading1,
    ///[heading-2](https://lucide.dev/icons/heading-2) icon
    Heading2,
    ///[heading-3](https://lucide.dev/icons/heading-3) icon
    Heading3,
    ///[heading-4](https://lucide.dev/icons/heading-4) icon
    Heading4,
    ///[heading-5](https://lucide.dev/icons/heading-5) icon
    Heading5,
    ///[heading-6](https://lucide.dev/icons/heading-6) icon
    Heading6,
    ///[headphone-off](https://lucide.dev/icons/headphone-off) icon
    HeadphoneOff,
    ///[headphones](https://lucide.dev/icons/headphones) icon
    Headphones,
    ///[headset](https://lucide.dev/icons/headset) icon
    Headset,
    ///[heart](https://lucide.dev/icons/heart) icon
    Heart,
    ///[heart-crack](https://lucide.dev/icons/heart-crack) icon
    HeartCrack,
    ///[heart-handshake](https://lucide.dev/icons/heart-handshake) icon
    HeartHandshake,
    ///[heart-minus](https://lucide.dev/icons/heart-minus) icon
    HeartMinus,
    ///[heart-off](https://lucide.dev/icons/heart-off) icon
    HeartOff,
    ///[heart-plus](https://lucide.dev/icons/heart-plus) icon
    HeartPlus,
    ///[heart-pulse](https://lucide.dev/icons/heart-pulse) icon
    HeartPulse,
    ///[heater](https://lucide.dev/icons/heater) icon
    Heater,
    ///[hexagon](https://lucide.dev/icons/hexagon) icon
    Hexagon,
    ///[highlighter](https://lucide.dev/icons/highlighter) icon
    Highlighter,
    ///[history](https://lucide.dev/icons/history) icon
    History,
    ///[hop](https://lucide.dev/icons/hop) icon
    Hop,
    ///[hop-off](https://lucide.dev/icons/hop-off) icon
    HopOff,
    ///[hospital](https://lucide.dev/icons/hospital) icon
    Hospital,
    ///[hotel](https://lucide.dev/icons/hotel) icon
    Hotel,
    ///[hourglass](https://lucide.dev/icons/hourglass) icon
    Hourglass,
    ///[house](https://lucide.dev/icons/house) icon
    House,
    ///[house-plug](https://lucide.dev/icons/house-plug) icon
    HousePlug,
    ///[house-plus](https://lucide.dev/icons/house-plus) icon
    HousePlus,
    ///[house-wifi](https://lucide.dev/icons/house-wifi) icon
    HouseWifi,
    ///[ice-cream-bowl](https://lucide.dev/icons/ice-cream-bowl) icon
    IceCreamBowl,
    ///[ice-cream-cone](https://lucide.dev/icons/ice-cream-cone) icon
    IceCreamCone,
    ///[id-card](https://lucide.dev/icons/id-card) icon
    IdCard,
    ///[id-card-lanyard](https://lucide.dev/icons/id-card-lanyard) icon
    IdCardLanyard,
    ///[image](https://lucide.dev/icons/image) icon
    Image,
    ///[image-down](https://lucide.dev/icons/image-down) icon
    ImageDown,
    ///[image-minus](https://lucide.dev/icons/image-minus) icon
    ImageMinus,
    ///[image-off](https://lucide.dev/icons/image-off) icon
    ImageOff,
    ///[image-play](https://lucide.dev/icons/image-play) icon
    ImagePlay,
    ///[image-plus](https://lucide.dev/icons/image-plus) icon
    ImagePlus,
    ///[image-up](https://lucide.dev/icons/image-up) icon
    ImageUp,
    ///[image-upscale](https://lucide.dev/icons/image-upscale) icon
    ImageUpscale,
    ///[images](https://lucide.dev/icons/images) icon
    Images,
    ///[import](https://lucide.dev/icons/import) icon
    Import,
    ///[inbox](https://lucide.dev/icons/inbox) icon
    Inbox,
    ///[indent-decrease](https://lucide.dev/icons/indent-decrease) icon
    IndentDecrease,
    ///[indent-increase](https://lucide.dev/icons/indent-increase) icon
    IndentIncrease,
    ///[indian-rupee](https://lucide.dev/icons/indian-rupee) icon
    IndianRupee,
    ///[infinity](https://lucide.dev/icons/infinity) icon
    Infinity,
    ///[info](https://lucide.dev/icons/info) icon
    Info,
    ///[inspection-panel](https://lucide.dev/icons/inspection-panel) icon
    InspectionPanel,
    ///[instagram](https://lucide.dev/icons/instagram) icon
    Instagram,
    ///[italic](https://lucide.dev/icons/italic) icon
    Italic,
    ///[iteration-ccw](https://lucide.dev/icons/iteration-ccw) icon
    IterationCcw,
    ///[iteration-cw](https://lucide.dev/icons/iteration-cw) icon
    IterationCw,
    ///[japanese-yen](https://lucide.dev/icons/japanese-yen) icon
    JapaneseYen,
    ///[joystick](https://lucide.dev/icons/joystick) icon
    Joystick,
    ///[kanban](https://lucide.dev/icons/kanban) icon
    Kanban,
    ///[key](https://lucide.dev/icons/key) icon
    Key,
    ///[key-round](https://lucide.dev/icons/key-round) icon
    KeyRound,
    ///[key-square](https://lucide.dev/icons/key-square) icon
    KeySquare,
    ///[keyboard](https://lucide.dev/icons/keyboard) icon
    Keyboard,
    ///[keyboard-music](https://lucide.dev/icons/keyboard-music) icon
    KeyboardMusic,
    ///[keyboard-off](https://lucide.dev/icons/keyboard-off) icon
    KeyboardOff,
    ///[lamp](https://lucide.dev/icons/lamp) icon
    Lamp,
    ///[lamp-ceiling](https://lucide.dev/icons/lamp-ceiling) icon
    LampCeiling,
    ///[lamp-desk](https://lucide.dev/icons/lamp-desk) icon
    LampDesk,
    ///[lamp-floor](https://lucide.dev/icons/lamp-floor) icon
    LampFloor,
    ///[lamp-wall-down](https://lucide.dev/icons/lamp-wall-down) icon
    LampWallDown,
    ///[lamp-wall-up](https://lucide.dev/icons/lamp-wall-up) icon
    LampWallUp,
    ///[land-plot](https://lucide.dev/icons/land-plot) icon
    LandPlot,
    ///[landmark](https://lucide.dev/icons/landmark) icon
    Landmark,
    ///[languages](https://lucide.dev/icons/languages) icon
    Languages,
    ///[laptop](https://lucide.dev/icons/laptop) icon
    Laptop,
    ///[laptop-minimal](https://lucide.dev/icons/laptop-minimal) icon
    LaptopMinimal,
    ///[laptop-minimal-check](https://lucide.dev/icons/laptop-minimal-check) icon
    LaptopMinimalCheck,
    ///[lasso](https://lucide.dev/icons/lasso) icon
    Lasso,
    ///[lasso-select](https://lucide.dev/icons/lasso-select) icon
    LassoSelect,
    ///[laugh](https://lucide.dev/icons/laugh) icon
    Laugh,
    ///[layers](https://lucide.dev/icons/layers) icon
    Layers,
    ///[layers-2](https://lucide.dev/icons/layers-2) icon
    Layers2,
    ///[layout-dashboard](https://lucide.dev/icons/layout-dashboard) icon
    LayoutDashboard,
    ///[layout-grid](https://lucide.dev/icons/layout-grid) icon
    LayoutGrid,
    ///[layout-list](https://lucide.dev/icons/layout-list) icon
    LayoutList,
    ///[layout-panel-left](https://lucide.dev/icons/layout-panel-left) icon
    LayoutPanelLeft,
    ///[layout-panel-top](https://lucide.dev/icons/layout-panel-top) icon
    LayoutPanelTop,
    ///[layout-template](https://lucide.dev/icons/layout-template) icon
    LayoutTemplate,
    ///[leaf](https://lucide.dev/icons/leaf) icon
    Leaf,
    ///[leafy-green](https://lucide.dev/icons/leafy-green) icon
    LeafyGreen,
    ///[lectern](https://lucide.dev/icons/lectern) icon
    Lectern,
    ///[letter-text](https://lucide.dev/icons/letter-text) icon
    LetterText,
    ///[library](https://lucide.dev/icons/library) icon
    Library,
    ///[library-big](https://lucide.dev/icons/library-big) icon
    LibraryBig,
    ///[life-buoy](https://lucide.dev/icons/life-buoy) icon
    LifeBuoy,
    ///[ligature](https://lucide.dev/icons/ligature) icon
    Ligature,
    ///[lightbulb](https://lucide.dev/icons/lightbulb) icon
    Lightbulb,
    ///[lightbulb-off](https://lucide.dev/icons/lightbulb-off) icon
    LightbulbOff,
    ///[line-squiggle](https://lucide.dev/icons/line-squiggle) icon
    LineSquiggle,
    ///[link](https://lucide.dev/icons/link) icon
    Link,
    ///[link-2](https://lucide.dev/icons/link-2) icon
    Link2,
    ///[link-2-off](https://lucide.dev/icons/link-2-off) icon
    Link2Off,
    ///[linkedin](https://lucide.dev/icons/linkedin) icon
    Linkedin,
    ///[list](https://lucide.dev/icons/list) icon
    List,
    ///[list-check](https://lucide.dev/icons/list-check) icon
    ListCheck,
    ///[list-checks](https://lucide.dev/icons/list-checks) icon
    ListChecks,
    ///[list-collapse](https://lucide.dev/icons/list-collapse) icon
    ListCollapse,
    ///[list-end](https://lucide.dev/icons/list-end) icon
    ListEnd,
    ///[list-filter](https://lucide.dev/icons/list-filter) icon
    ListFilter,
    ///[list-filter-plus](https://lucide.dev/icons/list-filter-plus) icon
    ListFilterPlus,
    ///[list-minus](https://lucide.dev/icons/list-minus) icon
    ListMinus,
    ///[list-music](https://lucide.dev/icons/list-music) icon
    ListMusic,
    ///[list-ordered](https://lucide.dev/icons/list-ordered) icon
    ListOrdered,
    ///[list-plus](https://lucide.dev/icons/list-plus) icon
    ListPlus,
    ///[list-restart](https://lucide.dev/icons/list-restart) icon
    ListRestart,
    ///[list-start](https://lucide.dev/icons/list-start) icon
    ListStart,
    ///[list-todo](https://lucide.dev/icons/list-todo) icon
    ListTodo,
    ///[list-tree](https://lucide.dev/icons/list-tree) icon
    ListTree,
    ///[list-video](https://lucide.dev/icons/list-video) icon
    ListVideo,
    ///[list-x](https://lucide.dev/icons/list-x) icon
    ListX,
    ///[loader](https://lucide.dev/icons/loader) icon
    Loader,
    ///[loader-circle](https://lucide.dev/icons/loader-circle) icon
    LoaderCircle,
    ///[loader-pinwheel](https://lucide.dev/icons/loader-pinwheel) icon
    LoaderPinwheel,
    ///[locate](https://lucide.dev/icons/locate) icon
    Locate,
    ///[locate-fixed](https://lucide.dev/icons/locate-fixed) icon
    LocateFixed,
    ///[locate-off](https://lucide.dev/icons/locate-off) icon
    LocateOff,
    ///[location-edit](https://lucide.dev/icons/location-edit) icon
    LocationEdit,
    ///[lock](https://lucide.dev/icons/lock) icon
    Lock,
    ///[lock-keyhole](https://lucide.dev/icons/lock-keyhole) icon
    LockKeyhole,
    ///[lock-keyhole-open](https://lucide.dev/icons/lock-keyhole-open) icon
    LockKeyholeOpen,
    ///[lock-open](https://lucide.dev/icons/lock-open) icon
    LockOpen,
    ///[log-in](https://lucide.dev/icons/log-in) icon
    LogIn,
    ///[log-out](https://lucide.dev/icons/log-out) icon
    LogOut,
    ///[logs](https://lucide.dev/icons/logs) icon
    Logs,
    ///[lollipop](https://lucide.dev/icons/lollipop) icon
    Lollipop,
    ///[luggage](https://lucide.dev/icons/luggage) icon
    Luggage,
    ///[magnet](https://lucide.dev/icons/magnet) icon
    Magnet,
    ///[mail](https://lucide.dev/icons/mail) icon
    Mail,
    ///[mail-check](https://lucide.dev/icons/mail-check) icon
    MailCheck,
    ///[mail-minus](https://lucide.dev/icons/mail-minus) icon
    MailMinus,
    ///[mail-open](https://lucide.dev/icons/mail-open) icon
    MailOpen,
    ///[mail-plus](https://lucide.dev/icons/mail-plus) icon
    MailPlus,
    ///[mail-question-mark](https://lucide.dev/icons/mail-question-mark) icon
    MailQuestionMark,
    ///[mail-search](https://lucide.dev/icons/mail-search) icon
    MailSearch,
    ///[mail-warning](https://lucide.dev/icons/mail-warning) icon
    MailWarning,
    ///[mail-x](https://lucide.dev/icons/mail-x) icon
    MailX,
    ///[mailbox](https://lucide.dev/icons/mailbox) icon
    Mailbox,
    ///[mails](https://lucide.dev/icons/mails) icon
    Mails,
    ///[map](https://lucide.dev/icons/map) icon
    Map,
    ///[map-pin](https://lucide.dev/icons/map-pin) icon
    MapPin,
    ///[map-pin-check](https://lucide.dev/icons/map-pin-check) icon
    MapPinCheck,
    ///[map-pin-check-inside](https://lucide.dev/icons/map-pin-check-inside) icon
    MapPinCheckInside,
    ///[map-pin-house](https://lucide.dev/icons/map-pin-house) icon
    MapPinHouse,
    ///[map-pin-minus](https://lucide.dev/icons/map-pin-minus) icon
    MapPinMinus,
    ///[map-pin-minus-inside](https://lucide.dev/icons/map-pin-minus-inside) icon
    MapPinMinusInside,
    ///[map-pin-off](https://lucide.dev/icons/map-pin-off) icon
    MapPinOff,
    ///[map-pin-plus](https://lucide.dev/icons/map-pin-plus) icon
    MapPinPlus,
    ///[map-pin-plus-inside](https://lucide.dev/icons/map-pin-plus-inside) icon
    MapPinPlusInside,
    ///[map-pin-x](https://lucide.dev/icons/map-pin-x) icon
    MapPinX,
    ///[map-pin-x-inside](https://lucide.dev/icons/map-pin-x-inside) icon
    MapPinXInside,
    ///[map-pinned](https://lucide.dev/icons/map-pinned) icon
    MapPinned,
    ///[map-plus](https://lucide.dev/icons/map-plus) icon
    MapPlus,
    ///[mars](https://lucide.dev/icons/mars) icon
    Mars,
    ///[mars-stroke](https://lucide.dev/icons/mars-stroke) icon
    MarsStroke,
    ///[martini](https://lucide.dev/icons/martini) icon
    Martini,
    ///[maximize](https://lucide.dev/icons/maximize) icon
    Maximize,
    ///[maximize-2](https://lucide.dev/icons/maximize-2) icon
    Maximize2,
    ///[medal](https://lucide.dev/icons/medal) icon
    Medal,
    ///[megaphone](https://lucide.dev/icons/megaphone) icon
    Megaphone,
    ///[megaphone-off](https://lucide.dev/icons/megaphone-off) icon
    MegaphoneOff,
    ///[meh](https://lucide.dev/icons/meh) icon
    Meh,
    ///[memory-stick](https://lucide.dev/icons/memory-stick) icon
    MemoryStick,
    ///[menu](https://lucide.dev/icons/menu) icon
    Menu,
    ///[merge](https://lucide.dev/icons/merge) icon
    Merge,
    ///[message-circle](https://lucide.dev/icons/message-circle) icon
    MessageCircle,
    ///[message-circle-code](https://lucide.dev/icons/message-circle-code) icon
    MessageCircleCode,
    ///[message-circle-dashed](https://lucide.dev/icons/message-circle-dashed) icon
    MessageCircleDashed,
    ///[message-circle-heart](https://lucide.dev/icons/message-circle-heart) icon
    MessageCircleHeart,
    ///[message-circle-more](https://lucide.dev/icons/message-circle-more) icon
    MessageCircleMore,
    ///[message-circle-off](https://lucide.dev/icons/message-circle-off) icon
    MessageCircleOff,
    ///[message-circle-plus](https://lucide.dev/icons/message-circle-plus) icon
    MessageCirclePlus,
    ///[message-circle-question-mark](https://lucide.dev/icons/message-circle-question-mark) icon
    MessageCircleQuestionMark,
    ///[message-circle-reply](https://lucide.dev/icons/message-circle-reply) icon
    MessageCircleReply,
    ///[message-circle-warning](https://lucide.dev/icons/message-circle-warning) icon
    MessageCircleWarning,
    ///[message-circle-x](https://lucide.dev/icons/message-circle-x) icon
    MessageCircleX,
    ///[message-square](https://lucide.dev/icons/message-square) icon
    MessageSquare,
    ///[message-square-code](https://lucide.dev/icons/message-square-code) icon
    MessageSquareCode,
    ///[message-square-dashed](https://lucide.dev/icons/message-square-dashed) icon
    MessageSquareDashed,
    ///[message-square-diff](https://lucide.dev/icons/message-square-diff) icon
    MessageSquareDiff,
    ///[message-square-dot](https://lucide.dev/icons/message-square-dot) icon
    MessageSquareDot,
    ///[message-square-heart](https://lucide.dev/icons/message-square-heart) icon
    MessageSquareHeart,
    ///[message-square-lock](https://lucide.dev/icons/message-square-lock) icon
    MessageSquareLock,
    ///[message-square-more](https://lucide.dev/icons/message-square-more) icon
    MessageSquareMore,
    ///[message-square-off](https://lucide.dev/icons/message-square-off) icon
    MessageSquareOff,
    ///[message-square-plus](https://lucide.dev/icons/message-square-plus) icon
    MessageSquarePlus,
    ///[message-square-quote](https://lucide.dev/icons/message-square-quote) icon
    MessageSquareQuote,
    ///[message-square-reply](https://lucide.dev/icons/message-square-reply) icon
    MessageSquareReply,
    ///[message-square-share](https://lucide.dev/icons/message-square-share) icon
    MessageSquareShare,
    ///[message-square-text](https://lucide.dev/icons/message-square-text) icon
    MessageSquareText,
    ///[message-square-warning](https://lucide.dev/icons/message-square-warning) icon
    MessageSquareWarning,
    ///[message-square-x](https://lucide.dev/icons/message-square-x) icon
    MessageSquareX,
    ///[messages-square](https://lucide.dev/icons/messages-square) icon
    MessagesSquare,
    ///[mic](https://lucide.dev/icons/mic) icon
    Mic,
    ///[mic-off](https://lucide.dev/icons/mic-off) icon
    MicOff,
    ///[mic-vocal](https://lucide.dev/icons/mic-vocal) icon
    MicVocal,
    ///[microchip](https://lucide.dev/icons/microchip) icon
    Microchip,
    ///[microscope](https://lucide.dev/icons/microscope) icon
    Microscope,
    ///[microwave](https://lucide.dev/icons/microwave) icon
    Microwave,
    ///[milestone](https://lucide.dev/icons/milestone) icon
    Milestone,
    ///[milk](https://lucide.dev/icons/milk) icon
    Milk,
    ///[milk-off](https://lucide.dev/icons/milk-off) icon
    MilkOff,
    ///[minimize](https://lucide.dev/icons/minimize) icon
    Minimize,
    ///[minimize-2](https://lucide.dev/icons/minimize-2) icon
    Minimize2,
    ///[minus](https://lucide.dev/icons/minus) icon
    Minus,
    ///[monitor](https://lucide.dev/icons/monitor) icon
    Monitor,
    ///[monitor-check](https://lucide.dev/icons/monitor-check) icon
    MonitorCheck,
    ///[monitor-cog](https://lucide.dev/icons/monitor-cog) icon
    MonitorCog,
    ///[monitor-dot](https://lucide.dev/icons/monitor-dot) icon
    MonitorDot,
    ///[monitor-down](https://lucide.dev/icons/monitor-down) icon
    MonitorDown,
    ///[monitor-off](https://lucide.dev/icons/monitor-off) icon
    MonitorOff,
    ///[monitor-pause](https://lucide.dev/icons/monitor-pause) icon
    MonitorPause,
    ///[monitor-play](https://lucide.dev/icons/monitor-play) icon
    MonitorPlay,
    ///[monitor-smartphone](https://lucide.dev/icons/monitor-smartphone) icon
    MonitorSmartphone,
    ///[monitor-speaker](https://lucide.dev/icons/monitor-speaker) icon
    MonitorSpeaker,
    ///[monitor-stop](https://lucide.dev/icons/monitor-stop) icon
    MonitorStop,
    ///[monitor-up](https://lucide.dev/icons/monitor-up) icon
    MonitorUp,
    ///[monitor-x](https://lucide.dev/icons/monitor-x) icon
    MonitorX,
    ///[moon](https://lucide.dev/icons/moon) icon
    Moon,
    ///[moon-star](https://lucide.dev/icons/moon-star) icon
    MoonStar,
    ///[mountain](https://lucide.dev/icons/mountain) icon
    Mountain,
    ///[mountain-snow](https://lucide.dev/icons/mountain-snow) icon
    MountainSnow,
    ///[mouse](https://lucide.dev/icons/mouse) icon
    Mouse,
    ///[mouse-off](https://lucide.dev/icons/mouse-off) icon
    MouseOff,
    ///[mouse-pointer](https://lucide.dev/icons/mouse-pointer) icon
    MousePointer,
    ///[mouse-pointer-2](https://lucide.dev/icons/mouse-pointer-2) icon
    MousePointer2,
    ///[mouse-pointer-ban](https://lucide.dev/icons/mouse-pointer-ban) icon
    MousePointerBan,
    ///[mouse-pointer-click](https://lucide.dev/icons/mouse-pointer-click) icon
    MousePointerClick,
    ///[move](https://lucide.dev/icons/move) icon
    Move,
    ///[move-3d](https://lucide.dev/icons/move-3d) icon
    Move3d,
    ///[move-diagonal](https://lucide.dev/icons/move-diagonal) icon
    MoveDiagonal,
    ///[move-diagonal-2](https://lucide.dev/icons/move-diagonal-2) icon
    MoveDiagonal2,
    ///[move-down](https://lucide.dev/icons/move-down) icon
    MoveDown,
    ///[move-down-left](https://lucide.dev/icons/move-down-left) icon
    MoveDownLeft,
    ///[move-down-right](https://lucide.dev/icons/move-down-right) icon
    MoveDownRight,
    ///[move-horizontal](https://lucide.dev/icons/move-horizontal) icon
    MoveHorizontal,
    ///[move-left](https://lucide.dev/icons/move-left) icon
    MoveLeft,
    ///[move-right](https://lucide.dev/icons/move-right) icon
    MoveRight,
    ///[move-up](https://lucide.dev/icons/move-up) icon
    MoveUp,
    ///[move-up-left](https://lucide.dev/icons/move-up-left) icon
    MoveUpLeft,
    ///[move-up-right](https://lucide.dev/icons/move-up-right) icon
    MoveUpRight,
    ///[move-vertical](https://lucide.dev/icons/move-vertical) icon
    MoveVertical,
    ///[music](https://lucide.dev/icons/music) icon
    Music,
    ///[music-2](https://lucide.dev/icons/music-2) icon
    Music2,
    ///[music-3](https://lucide.dev/icons/music-3) icon
    Music3,
    ///[music-4](https://lucide.dev/icons/music-4) icon
    Music4,
    ///[navigation](https://lucide.dev/icons/navigation) icon
    Navigation,
    ///[navigation-2](https://lucide.dev/icons/navigation-2) icon
    Navigation2,
    ///[navigation-2-off](https://lucide.dev/icons/navigation-2-off) icon
    Navigation2Off,
    ///[navigation-off](https://lucide.dev/icons/navigation-off) icon
    NavigationOff,
    ///[network](https://lucide.dev/icons/network) icon
    Network,
    ///[newspaper](https://lucide.dev/icons/newspaper) icon
    Newspaper,
    ///[nfc](https://lucide.dev/icons/nfc) icon
    Nfc,
    ///[non-binary](https://lucide.dev/icons/non-binary) icon
    NonBinary,
    ///[notebook](https://lucide.dev/icons/notebook) icon
    Notebook,
    ///[notebook-pen](https://lucide.dev/icons/notebook-pen) icon
    NotebookPen,
    ///[notebook-tabs](https://lucide.dev/icons/notebook-tabs) icon
    NotebookTabs,
    ///[notebook-text](https://lucide.dev/icons/notebook-text) icon
    NotebookText,
    ///[notepad-text](https://lucide.dev/icons/notepad-text) icon
    NotepadText,
    ///[notepad-text-dashed](https://lucide.dev/icons/notepad-text-dashed) icon
    NotepadTextDashed,
    ///[nut](https://lucide.dev/icons/nut) icon
    Nut,
    ///[nut-off](https://lucide.dev/icons/nut-off) icon
    NutOff,
    ///[octagon](https://lucide.dev/icons/octagon) icon
    Octagon,
    ///[octagon-alert](https://lucide.dev/icons/octagon-alert) icon
    OctagonAlert,
    ///[octagon-minus](https://lucide.dev/icons/octagon-minus) icon
    OctagonMinus,
    ///[octagon-pause](https://lucide.dev/icons/octagon-pause) icon
    OctagonPause,
    ///[octagon-x](https://lucide.dev/icons/octagon-x) icon
    OctagonX,
    ///[omega](https://lucide.dev/icons/omega) icon
    Omega,
    ///[option](https://lucide.dev/icons/option) icon
    Option,
    ///[orbit](https://lucide.dev/icons/orbit) icon
    Orbit,
    ///[origami](https://lucide.dev/icons/origami) icon
    Origami,
    ///[package](https://lucide.dev/icons/package) icon
    Package,
    ///[package-2](https://lucide.dev/icons/package-2) icon
    Package2,
    ///[package-check](https://lucide.dev/icons/package-check) icon
    PackageCheck,
    ///[package-minus](https://lucide.dev/icons/package-minus) icon
    PackageMinus,
    ///[package-open](https://lucide.dev/icons/package-open) icon
    PackageOpen,
    ///[package-plus](https://lucide.dev/icons/package-plus) icon
    PackagePlus,
    ///[package-search](https://lucide.dev/icons/package-search) icon
    PackageSearch,
    ///[package-x](https://lucide.dev/icons/package-x) icon
    PackageX,
    ///[paint-bucket](https://lucide.dev/icons/paint-bucket) icon
    PaintBucket,
    ///[paint-roller](https://lucide.dev/icons/paint-roller) icon
    PaintRoller,
    ///[paintbrush](https://lucide.dev/icons/paintbrush) icon
    Paintbrush,
    ///[paintbrush-vertical](https://lucide.dev/icons/paintbrush-vertical) icon
    PaintbrushVertical,
    ///[palette](https://lucide.dev/icons/palette) icon
    Palette,
    ///[panda](https://lucide.dev/icons/panda) icon
    Panda,
    ///[panel-bottom](https://lucide.dev/icons/panel-bottom) icon
    PanelBottom,
    ///[panel-bottom-close](https://lucide.dev/icons/panel-bottom-close) icon
    PanelBottomClose,
    ///[panel-bottom-dashed](https://lucide.dev/icons/panel-bottom-dashed) icon
    PanelBottomDashed,
    ///[panel-bottom-open](https://lucide.dev/icons/panel-bottom-open) icon
    PanelBottomOpen,
    ///[panel-left](https://lucide.dev/icons/panel-left) icon
    PanelLeft,
    ///[panel-left-close](https://lucide.dev/icons/panel-left-close) icon
    PanelLeftClose,
    ///[panel-left-dashed](https://lucide.dev/icons/panel-left-dashed) icon
    PanelLeftDashed,
    ///[panel-left-open](https://lucide.dev/icons/panel-left-open) icon
    PanelLeftOpen,
    ///[panel-right](https://lucide.dev/icons/panel-right) icon
    PanelRight,
    ///[panel-right-close](https://lucide.dev/icons/panel-right-close) icon
    PanelRightClose,
    ///[panel-right-dashed](https://lucide.dev/icons/panel-right-dashed) icon
    PanelRightDashed,
    ///[panel-right-open](https://lucide.dev/icons/panel-right-open) icon
    PanelRightOpen,
    ///[panel-top](https://lucide.dev/icons/panel-top) icon
    PanelTop,
    ///[panel-top-close](https://lucide.dev/icons/panel-top-close) icon
    PanelTopClose,
    ///[panel-top-dashed](https://lucide.dev/icons/panel-top-dashed) icon
    PanelTopDashed,
    ///[panel-top-open](https://lucide.dev/icons/panel-top-open) icon
    PanelTopOpen,
    ///[panels-left-bottom](https://lucide.dev/icons/panels-left-bottom) icon
    PanelsLeftBottom,
    ///[panels-right-bottom](https://lucide.dev/icons/panels-right-bottom) icon
    PanelsRightBottom,
    ///[panels-top-left](https://lucide.dev/icons/panels-top-left) icon
    PanelsTopLeft,
    ///[paperclip](https://lucide.dev/icons/paperclip) icon
    Paperclip,
    ///[parentheses](https://lucide.dev/icons/parentheses) icon
    Parentheses,
    ///[parking-meter](https://lucide.dev/icons/parking-meter) icon
    ParkingMeter,
    ///[party-popper](https://lucide.dev/icons/party-popper) icon
    PartyPopper,
    ///[pause](https://lucide.dev/icons/pause) icon
    Pause,
    ///[paw-print](https://lucide.dev/icons/paw-print) icon
    PawPrint,
    ///[pc-case](https://lucide.dev/icons/pc-case) icon
    PcCase,
    ///[pen](https://lucide.dev/icons/pen) icon
    Pen,
    ///[pen-line](https://lucide.dev/icons/pen-line) icon
    PenLine,
    ///[pen-off](https://lucide.dev/icons/pen-off) icon
    PenOff,
    ///[pen-tool](https://lucide.dev/icons/pen-tool) icon
    PenTool,
    ///[pencil](https://lucide.dev/icons/pencil) icon
    Pencil,
    ///[pencil-line](https://lucide.dev/icons/pencil-line) icon
    PencilLine,
    ///[pencil-off](https://lucide.dev/icons/pencil-off) icon
    PencilOff,
    ///[pencil-ruler](https://lucide.dev/icons/pencil-ruler) icon
    PencilRuler,
    ///[pentagon](https://lucide.dev/icons/pentagon) icon
    Pentagon,
    ///[percent](https://lucide.dev/icons/percent) icon
    Percent,
    ///[person-standing](https://lucide.dev/icons/person-standing) icon
    PersonStanding,
    ///[philippine-peso](https://lucide.dev/icons/philippine-peso) icon
    PhilippinePeso,
    ///[phone](https://lucide.dev/icons/phone) icon
    Phone,
    ///[phone-call](https://lucide.dev/icons/phone-call) icon
    PhoneCall,
    ///[phone-forwarded](https://lucide.dev/icons/phone-forwarded) icon
    PhoneForwarded,
    ///[phone-incoming](https://lucide.dev/icons/phone-incoming) icon
    PhoneIncoming,
    ///[phone-missed](https://lucide.dev/icons/phone-missed) icon
    PhoneMissed,
    ///[phone-off](https://lucide.dev/icons/phone-off) icon
    PhoneOff,
    ///[phone-outgoing](https://lucide.dev/icons/phone-outgoing) icon
    PhoneOutgoing,
    ///[pi](https://lucide.dev/icons/pi) icon
    Pi,
    ///[piano](https://lucide.dev/icons/piano) icon
    Piano,
    ///[pickaxe](https://lucide.dev/icons/pickaxe) icon
    Pickaxe,
    ///[picture-in-picture](https://lucide.dev/icons/picture-in-picture) icon
    PictureInPicture,
    ///[picture-in-picture-2](https://lucide.dev/icons/picture-in-picture-2) icon
    PictureInPicture2,
    ///[piggy-bank](https://lucide.dev/icons/piggy-bank) icon
    PiggyBank,
    ///[pilcrow](https://lucide.dev/icons/pilcrow) icon
    Pilcrow,
    ///[pilcrow-left](https://lucide.dev/icons/pilcrow-left) icon
    PilcrowLeft,
    ///[pilcrow-right](https://lucide.dev/icons/pilcrow-right) icon
    PilcrowRight,
    ///[pill](https://lucide.dev/icons/pill) icon
    Pill,
    ///[pill-bottle](https://lucide.dev/icons/pill-bottle) icon
    PillBottle,
    ///[pin](https://lucide.dev/icons/pin) icon
    Pin,
    ///[pin-off](https://lucide.dev/icons/pin-off) icon
    PinOff,
    ///[pipette](https://lucide.dev/icons/pipette) icon
    Pipette,
    ///[pizza](https://lucide.dev/icons/pizza) icon
    Pizza,
    ///[plane](https://lucide.dev/icons/plane) icon
    Plane,
    ///[plane-landing](https://lucide.dev/icons/plane-landing) icon
    PlaneLanding,
    ///[plane-takeoff](https://lucide.dev/icons/plane-takeoff) icon
    PlaneTakeoff,
    ///[play](https://lucide.dev/icons/play) icon
    Play,
    ///[plug](https://lucide.dev/icons/plug) icon
    Plug,
    ///[plug-2](https://lucide.dev/icons/plug-2) icon
    Plug2,
    ///[plug-zap](https://lucide.dev/icons/plug-zap) icon
    PlugZap,
    ///[plus](https://lucide.dev/icons/plus) icon
    Plus,
    ///[pocket](https://lucide.dev/icons/pocket) icon
    Pocket,
    ///[pocket-knife](https://lucide.dev/icons/pocket-knife) icon
    PocketKnife,
    ///[podcast](https://lucide.dev/icons/podcast) icon
    Podcast,
    ///[pointer](https://lucide.dev/icons/pointer) icon
    Pointer,
    ///[pointer-off](https://lucide.dev/icons/pointer-off) icon
    PointerOff,
    ///[popcorn](https://lucide.dev/icons/popcorn) icon
    Popcorn,
    ///[popsicle](https://lucide.dev/icons/popsicle) icon
    Popsicle,
    ///[pound-sterling](https://lucide.dev/icons/pound-sterling) icon
    PoundSterling,
    ///[power](https://lucide.dev/icons/power) icon
    Power,
    ///[power-off](https://lucide.dev/icons/power-off) icon
    PowerOff,
    ///[presentation](https://lucide.dev/icons/presentation) icon
    Presentation,
    ///[printer](https://lucide.dev/icons/printer) icon
    Printer,
    ///[printer-check](https://lucide.dev/icons/printer-check) icon
    PrinterCheck,
    ///[projector](https://lucide.dev/icons/projector) icon
    Projector,
    ///[proportions](https://lucide.dev/icons/proportions) icon
    Proportions,
    ///[puzzle](https://lucide.dev/icons/puzzle) icon
    Puzzle,
    ///[pyramid](https://lucide.dev/icons/pyramid) icon
    Pyramid,
    ///[qr-code](https://lucide.dev/icons/qr-code) icon
    QrCode,
    ///[quote](https://lucide.dev/icons/quote) icon
    Quote,
    ///[rabbit](https://lucide.dev/icons/rabbit) icon
    Rabbit,
    ///[radar](https://lucide.dev/icons/radar) icon
    Radar,
    ///[radiation](https://lucide.dev/icons/radiation) icon
    Radiation,
    ///[radical](https://lucide.dev/icons/radical) icon
    Radical,
    ///[radio](https://lucide.dev/icons/radio) icon
    Radio,
    ///[radio-receiver](https://lucide.dev/icons/radio-receiver) icon
    RadioReceiver,
    ///[radio-tower](https://lucide.dev/icons/radio-tower) icon
    RadioTower,
    ///[radius](https://lucide.dev/icons/radius) icon
    Radius,
    ///[rail-symbol](https://lucide.dev/icons/rail-symbol) icon
    RailSymbol,
    ///[rainbow](https://lucide.dev/icons/rainbow) icon
    Rainbow,
    ///[rat](https://lucide.dev/icons/rat) icon
    Rat,
    ///[ratio](https://lucide.dev/icons/ratio) icon
    Ratio,
    ///[receipt](https://lucide.dev/icons/receipt) icon
    Receipt,
    ///[receipt-cent](https://lucide.dev/icons/receipt-cent) icon
    ReceiptCent,
    ///[receipt-euro](https://lucide.dev/icons/receipt-euro) icon
    ReceiptEuro,
    ///[receipt-indian-rupee](https://lucide.dev/icons/receipt-indian-rupee) icon
    ReceiptIndianRupee,
    ///[receipt-japanese-yen](https://lucide.dev/icons/receipt-japanese-yen) icon
    ReceiptJapaneseYen,
    ///[receipt-pound-sterling](https://lucide.dev/icons/receipt-pound-sterling) icon
    ReceiptPoundSterling,
    ///[receipt-russian-ruble](https://lucide.dev/icons/receipt-russian-ruble) icon
    ReceiptRussianRuble,
    ///[receipt-swiss-franc](https://lucide.dev/icons/receipt-swiss-franc) icon
    ReceiptSwissFranc,
    ///[receipt-text](https://lucide.dev/icons/receipt-text) icon
    ReceiptText,
    ///[rectangle-circle](https://lucide.dev/icons/rectangle-circle) icon
    RectangleCircle,
    ///[rectangle-ellipsis](https://lucide.dev/icons/rectangle-ellipsis) icon
    RectangleEllipsis,
    ///[rectangle-goggles](https://lucide.dev/icons/rectangle-goggles) icon
    RectangleGoggles,
    ///[rectangle-horizontal](https://lucide.dev/icons/rectangle-horizontal) icon
    RectangleHorizontal,
    ///[rectangle-vertical](https://lucide.dev/icons/rectangle-vertical) icon
    RectangleVertical,
    ///[recycle](https://lucide.dev/icons/recycle) icon
    Recycle,
    ///[redo](https://lucide.dev/icons/redo) icon
    Redo,
    ///[redo-2](https://lucide.dev/icons/redo-2) icon
    Redo2,
    ///[redo-dot](https://lucide.dev/icons/redo-dot) icon
    RedoDot,
    ///[refresh-ccw](https://lucide.dev/icons/refresh-ccw) icon
    RefreshCcw,
    ///[refresh-ccw-dot](https://lucide.dev/icons/refresh-ccw-dot) icon
    RefreshCcwDot,
    ///[refresh-cw](https://lucide.dev/icons/refresh-cw) icon
    RefreshCw,
    ///[refresh-cw-off](https://lucide.dev/icons/refresh-cw-off) icon
    RefreshCwOff,
    ///[refrigerator](https://lucide.dev/icons/refrigerator) icon
    Refrigerator,
    ///[regex](https://lucide.dev/icons/regex) icon
    Regex,
    ///[remove-formatting](https://lucide.dev/icons/remove-formatting) icon
    RemoveFormatting,
    ///[repeat](https://lucide.dev/icons/repeat) icon
    Repeat,
    ///[repeat-1](https://lucide.dev/icons/repeat-1) icon
    Repeat1,
    ///[repeat-2](https://lucide.dev/icons/repeat-2) icon
    Repeat2,
    ///[replace](https://lucide.dev/icons/replace) icon
    Replace,
    ///[replace-all](https://lucide.dev/icons/replace-all) icon
    ReplaceAll,
    ///[reply](https://lucide.dev/icons/reply) icon
    Reply,
    ///[reply-all](https://lucide.dev/icons/reply-all) icon
    ReplyAll,
    ///[rewind](https://lucide.dev/icons/rewind) icon
    Rewind,
    ///[ribbon](https://lucide.dev/icons/ribbon) icon
    Ribbon,
    ///[rocket](https://lucide.dev/icons/rocket) icon
    Rocket,
    ///[rocking-chair](https://lucide.dev/icons/rocking-chair) icon
    RockingChair,
    ///[roller-coaster](https://lucide.dev/icons/roller-coaster) icon
    RollerCoaster,
    ///[rotate-3d](https://lucide.dev/icons/rotate-3d) icon
    Rotate3d,
    ///[rotate-ccw](https://lucide.dev/icons/rotate-ccw) icon
    RotateCcw,
    ///[rotate-ccw-key](https://lucide.dev/icons/rotate-ccw-key) icon
    RotateCcwKey,
    ///[rotate-ccw-square](https://lucide.dev/icons/rotate-ccw-square) icon
    RotateCcwSquare,
    ///[rotate-cw](https://lucide.dev/icons/rotate-cw) icon
    RotateCw,
    ///[rotate-cw-square](https://lucide.dev/icons/rotate-cw-square) icon
    RotateCwSquare,
    ///[route](https://lucide.dev/icons/route) icon
    Route,
    ///[route-off](https://lucide.dev/icons/route-off) icon
    RouteOff,
    ///[router](https://lucide.dev/icons/router) icon
    Router,
    ///[rows-2](https://lucide.dev/icons/rows-2) icon
    Rows2,
    ///[rows-3](https://lucide.dev/icons/rows-3) icon
    Rows3,
    ///[rows-4](https://lucide.dev/icons/rows-4) icon
    Rows4,
    ///[rss](https://lucide.dev/icons/rss) icon
    Rss,
    ///[ruler](https://lucide.dev/icons/ruler) icon
    Ruler,
    ///[ruler-dimension-line](https://lucide.dev/icons/ruler-dimension-line) icon
    RulerDimensionLine,
    ///[russian-ruble](https://lucide.dev/icons/russian-ruble) icon
    RussianRuble,
    ///[sailboat](https://lucide.dev/icons/sailboat) icon
    Sailboat,
    ///[salad](https://lucide.dev/icons/salad) icon
    Salad,
    ///[sandwich](https://lucide.dev/icons/sandwich) icon
    Sandwich,
    ///[satellite](https://lucide.dev/icons/satellite) icon
    Satellite,
    ///[satellite-dish](https://lucide.dev/icons/satellite-dish) icon
    SatelliteDish,
    ///[saudi-riyal](https://lucide.dev/icons/saudi-riyal) icon
    SaudiRiyal,
    ///[save](https://lucide.dev/icons/save) icon
    Save,
    ///[save-all](https://lucide.dev/icons/save-all) icon
    SaveAll,
    ///[save-off](https://lucide.dev/icons/save-off) icon
    SaveOff,
    ///[scale](https://lucide.dev/icons/scale) icon
    Scale,
    ///[scale-3d](https://lucide.dev/icons/scale-3d) icon
    Scale3d,
    ///[scaling](https://lucide.dev/icons/scaling) icon
    Scaling,
    ///[scan](https://lucide.dev/icons/scan) icon
    Scan,
    ///[scan-barcode](https://lucide.dev/icons/scan-barcode) icon
    ScanBarcode,
    ///[scan-eye](https://lucide.dev/icons/scan-eye) icon
    ScanEye,
    ///[scan-face](https://lucide.dev/icons/scan-face) icon
    ScanFace,
    ///[scan-heart](https://lucide.dev/icons/scan-heart) icon
    ScanHeart,
    ///[scan-line](https://lucide.dev/icons/scan-line) icon
    ScanLine,
    ///[scan-qr-code](https://lucide.dev/icons/scan-qr-code) icon
    ScanQrCode,
    ///[scan-search](https://lucide.dev/icons/scan-search) icon
    ScanSearch,
    ///[scan-text](https://lucide.dev/icons/scan-text) icon
    ScanText,
    ///[school](https://lucide.dev/icons/school) icon
    School,
    ///[scissors](https://lucide.dev/icons/scissors) icon
    Scissors,
    ///[scissors-line-dashed](https://lucide.dev/icons/scissors-line-dashed) icon
    ScissorsLineDashed,
    ///[screen-share](https://lucide.dev/icons/screen-share) icon
    ScreenShare,
    ///[screen-share-off](https://lucide.dev/icons/screen-share-off) icon
    ScreenShareOff,
    ///[scroll](https://lucide.dev/icons/scroll) icon
    Scroll,
    ///[scroll-text](https://lucide.dev/icons/scroll-text) icon
    ScrollText,
    ///[search](https://lucide.dev/icons/search) icon
    Search,
    ///[search-check](https://lucide.dev/icons/search-check) icon
    SearchCheck,
    ///[search-code](https://lucide.dev/icons/search-code) icon
    SearchCode,
    ///[search-slash](https://lucide.dev/icons/search-slash) icon
    SearchSlash,
    ///[search-x](https://lucide.dev/icons/search-x) icon
    SearchX,
    ///[section](https://lucide.dev/icons/section) icon
    Section,
    ///[send](https://lucide.dev/icons/send) icon
    Send,
    ///[send-horizontal](https://lucide.dev/icons/send-horizontal) icon
    SendHorizontal,
    ///[send-to-back](https://lucide.dev/icons/send-to-back) icon
    SendToBack,
    ///[separator-horizontal](https://lucide.dev/icons/separator-horizontal) icon
    SeparatorHorizontal,
    ///[separator-vertical](https://lucide.dev/icons/separator-vertical) icon
    SeparatorVertical,
    ///[server](https://lucide.dev/icons/server) icon
    Server,
    ///[server-cog](https://lucide.dev/icons/server-cog) icon
    ServerCog,
    ///[server-crash](https://lucide.dev/icons/server-crash) icon
    ServerCrash,
    ///[server-off](https://lucide.dev/icons/server-off) icon
    ServerOff,
    ///[settings](https://lucide.dev/icons/settings) icon
    Settings,
    ///[settings-2](https://lucide.dev/icons/settings-2) icon
    Settings2,
    ///[shapes](https://lucide.dev/icons/shapes) icon
    Shapes,
    ///[share](https://lucide.dev/icons/share) icon
    Share,
    ///[share-2](https://lucide.dev/icons/share-2) icon
    Share2,
    ///[sheet](https://lucide.dev/icons/sheet) icon
    Sheet,
    ///[shell](https://lucide.dev/icons/shell) icon
    Shell,
    ///[shield](https://lucide.dev/icons/shield) icon
    Shield,
    ///[shield-alert](https://lucide.dev/icons/shield-alert) icon
    ShieldAlert,
    ///[shield-ban](https://lucide.dev/icons/shield-ban) icon
    ShieldBan,
    ///[shield-check](https://lucide.dev/icons/shield-check) icon
    ShieldCheck,
    ///[shield-ellipsis](https://lucide.dev/icons/shield-ellipsis) icon
    ShieldEllipsis,
    ///[shield-half](https://lucide.dev/icons/shield-half) icon
    ShieldHalf,
    ///[shield-minus](https://lucide.dev/icons/shield-minus) icon
    ShieldMinus,
    ///[shield-off](https://lucide.dev/icons/shield-off) icon
    ShieldOff,
    ///[shield-plus](https://lucide.dev/icons/shield-plus) icon
    ShieldPlus,
    ///[shield-question-mark](https://lucide.dev/icons/shield-question-mark) icon
    ShieldQuestionMark,
    ///[shield-user](https://lucide.dev/icons/shield-user) icon
    ShieldUser,
    ///[shield-x](https://lucide.dev/icons/shield-x) icon
    ShieldX,
    ///[ship](https://lucide.dev/icons/ship) icon
    Ship,
    ///[ship-wheel](https://lucide.dev/icons/ship-wheel) icon
    ShipWheel,
    ///[shirt](https://lucide.dev/icons/shirt) icon
    Shirt,
    ///[shopping-bag](https://lucide.dev/icons/shopping-bag) icon
    ShoppingBag,
    ///[shopping-basket](https://lucide.dev/icons/shopping-basket) icon
    ShoppingBasket,
    ///[shopping-cart](https://lucide.dev/icons/shopping-cart) icon
    ShoppingCart,
    ///[shovel](https://lucide.dev/icons/shovel) icon
    Shovel,
    ///[shower-head](https://lucide.dev/icons/shower-head) icon
    ShowerHead,
    ///[shredder](https://lucide.dev/icons/shredder) icon
    Shredder,
    ///[shrimp](https://lucide.dev/icons/shrimp) icon
    Shrimp,
    ///[shrink](https://lucide.dev/icons/shrink) icon
    Shrink,
    ///[shrub](https://lucide.dev/icons/shrub) icon
    Shrub,
    ///[shuffle](https://lucide.dev/icons/shuffle) icon
    Shuffle,
    ///[sigma](https://lucide.dev/icons/sigma) icon
    Sigma,
    ///[signal](https://lucide.dev/icons/signal) icon
    Signal,
    ///[signal-high](https://lucide.dev/icons/signal-high) icon
    SignalHigh,
    ///[signal-low](https://lucide.dev/icons/signal-low) icon
    SignalLow,
    ///[signal-medium](https://lucide.dev/icons/signal-medium) icon
    SignalMedium,
    ///[signal-zero](https://lucide.dev/icons/signal-zero) icon
    SignalZero,
    ///[signature](https://lucide.dev/icons/signature) icon
    Signature,
    ///[signpost](https://lucide.dev/icons/signpost) icon
    Signpost,
    ///[signpost-big](https://lucide.dev/icons/signpost-big) icon
    SignpostBig,
    ///[siren](https://lucide.dev/icons/siren) icon
    Siren,
    ///[skip-back](https://lucide.dev/icons/skip-back) icon
    SkipBack,
    ///[skip-forward](https://lucide.dev/icons/skip-forward) icon
    SkipForward,
    ///[skull](https://lucide.dev/icons/skull) icon
    Skull,
    ///[slack](https://lucide.dev/icons/slack) icon
    Slack,
    ///[slash](https://lucide.dev/icons/slash) icon
    Slash,
    ///[slice](https://lucide.dev/icons/slice) icon
    Slice,
    ///[sliders-horizontal](https://lucide.dev/icons/sliders-horizontal) icon
    SlidersHorizontal,
    ///[sliders-vertical](https://lucide.dev/icons/sliders-vertical) icon
    SlidersVertical,
    ///[smartphone](https://lucide.dev/icons/smartphone) icon
    Smartphone,
    ///[smartphone-charging](https://lucide.dev/icons/smartphone-charging) icon
    SmartphoneCharging,
    ///[smartphone-nfc](https://lucide.dev/icons/smartphone-nfc) icon
    SmartphoneNfc,
    ///[smile](https://lucide.dev/icons/smile) icon
    Smile,
    ///[smile-plus](https://lucide.dev/icons/smile-plus) icon
    SmilePlus,
    ///[snail](https://lucide.dev/icons/snail) icon
    Snail,
    ///[snowflake](https://lucide.dev/icons/snowflake) icon
    Snowflake,
    ///[soap-dispenser-droplet](https://lucide.dev/icons/soap-dispenser-droplet) icon
    SoapDispenserDroplet,
    ///[sofa](https://lucide.dev/icons/sofa) icon
    Sofa,
    ///[soup](https://lucide.dev/icons/soup) icon
    Soup,
    ///[space](https://lucide.dev/icons/space) icon
    Space,
    ///[spade](https://lucide.dev/icons/spade) icon
    Spade,
    ///[sparkle](https://lucide.dev/icons/sparkle) icon
    Sparkle,
    ///[sparkles](https://lucide.dev/icons/sparkles) icon
    Sparkles,
    ///[speaker](https://lucide.dev/icons/speaker) icon
    Speaker,
    ///[speech](https://lucide.dev/icons/speech) icon
    Speech,
    ///[spell-check](https://lucide.dev/icons/spell-check) icon
    SpellCheck,
    ///[spell-check-2](https://lucide.dev/icons/spell-check-2) icon
    SpellCheck2,
    ///[spline](https://lucide.dev/icons/spline) icon
    Spline,
    ///[spline-pointer](https://lucide.dev/icons/spline-pointer) icon
    SplinePointer,
    ///[split](https://lucide.dev/icons/split) icon
    Split,
    ///[spool](https://lucide.dev/icons/spool) icon
    Spool,
    ///[spray-can](https://lucide.dev/icons/spray-can) icon
    SprayCan,
    ///[sprout](https://lucide.dev/icons/sprout) icon
    Sprout,
    ///[square](https://lucide.dev/icons/square) icon
    Square,
    ///[square-activity](https://lucide.dev/icons/square-activity) icon
    SquareActivity,
    ///[square-arrow-down](https://lucide.dev/icons/square-arrow-down) icon
    SquareArrowDown,
    ///[square-arrow-down-left](https://lucide.dev/icons/square-arrow-down-left) icon
    SquareArrowDownLeft,
    ///[square-arrow-down-right](https://lucide.dev/icons/square-arrow-down-right) icon
    SquareArrowDownRight,
    ///[square-arrow-left](https://lucide.dev/icons/square-arrow-left) icon
    SquareArrowLeft,
    ///[square-arrow-out-down-left](https://lucide.dev/icons/square-arrow-out-down-left) icon
    SquareArrowOutDownLeft,
    ///[square-arrow-out-down-right](https://lucide.dev/icons/square-arrow-out-down-right) icon
    SquareArrowOutDownRight,
    ///[square-arrow-out-up-left](https://lucide.dev/icons/square-arrow-out-up-left) icon
    SquareArrowOutUpLeft,
    ///[square-arrow-out-up-right](https://lucide.dev/icons/square-arrow-out-up-right) icon
    SquareArrowOutUpRight,
    ///[square-arrow-right](https://lucide.dev/icons/square-arrow-right) icon
    SquareArrowRight,
    ///[square-arrow-up](https://lucide.dev/icons/square-arrow-up) icon
    SquareArrowUp,
    ///[square-arrow-up-left](https://lucide.dev/icons/square-arrow-up-left) icon
    SquareArrowUpLeft,
    ///[square-arrow-up-right](https://lucide.dev/icons/square-arrow-up-right) icon
    SquareArrowUpRight,
    ///[square-asterisk](https://lucide.dev/icons/square-asterisk) icon
    SquareAsterisk,
    ///[square-bottom-dashed-scissors](https://lucide.dev/icons/square-bottom-dashed-scissors) icon
    SquareBottomDashedScissors,
    ///[square-chart-gantt](https://lucide.dev/icons/square-chart-gantt) icon
    SquareChartGantt,
    ///[square-check](https://lucide.dev/icons/square-check) icon
    SquareCheck,
    ///[square-check-big](https://lucide.dev/icons/square-check-big) icon
    SquareCheckBig,
    ///[square-chevron-down](https://lucide.dev/icons/square-chevron-down) icon
    SquareChevronDown,
    ///[square-chevron-left](https://lucide.dev/icons/square-chevron-left) icon
    SquareChevronLeft,
    ///[square-chevron-right](https://lucide.dev/icons/square-chevron-right) icon
    SquareChevronRight,
    ///[square-chevron-up](https://lucide.dev/icons/square-chevron-up) icon
    SquareChevronUp,
    ///[square-code](https://lucide.dev/icons/square-code) icon
    SquareCode,
    ///[square-dashed](https://lucide.dev/icons/square-dashed) icon
    SquareDashed,
    ///[square-dashed-bottom](https://lucide.dev/icons/square-dashed-bottom) icon
    SquareDashedBottom,
    ///[square-dashed-bottom-code](https://lucide.dev/icons/square-dashed-bottom-code) icon
    SquareDashedBottomCode,
    ///[square-dashed-kanban](https://lucide.dev/icons/square-dashed-kanban) icon
    SquareDashedKanban,
    ///[square-dashed-mouse-pointer](https://lucide.dev/icons/square-dashed-mouse-pointer) icon
    SquareDashedMousePointer,
    ///[square-dashed-top-solid](https://lucide.dev/icons/square-dashed-top-solid) icon
    SquareDashedTopSolid,
    ///[square-divide](https://lucide.dev/icons/square-divide) icon
    SquareDivide,
    ///[square-dot](https://lucide.dev/icons/square-dot) icon
    SquareDot,
    ///[square-equal](https://lucide.dev/icons/square-equal) icon
    SquareEqual,
    ///[square-function](https://lucide.dev/icons/square-function) icon
    SquareFunction,
    ///[square-kanban](https://lucide.dev/icons/square-kanban) icon
    SquareKanban,
    ///[square-library](https://lucide.dev/icons/square-library) icon
    SquareLibrary,
    ///[square-m](https://lucide.dev/icons/square-m) icon
    SquareM,
    ///[square-menu](https://lucide.dev/icons/square-menu) icon
    SquareMenu,
    ///[square-minus](https://lucide.dev/icons/square-minus) icon
    SquareMinus,
    ///[square-mouse-pointer](https://lucide.dev/icons/square-mouse-pointer) icon
    SquareMousePointer,
    ///[square-parking](https://lucide.dev/icons/square-parking) icon
    SquareParking,
    ///[square-parking-off](https://lucide.dev/icons/square-parking-off) icon
    SquareParkingOff,
    ///[square-pen](https://lucide.dev/icons/square-pen) icon
    SquarePen,
    ///[square-percent](https://lucide.dev/icons/square-percent) icon
    SquarePercent,
    ///[square-pi](https://lucide.dev/icons/square-pi) icon
    SquarePi,
    ///[square-pilcrow](https://lucide.dev/icons/square-pilcrow) icon
    SquarePilcrow,
    ///[square-play](https://lucide.dev/icons/square-play) icon
    SquarePlay,
    ///[square-plus](https://lucide.dev/icons/square-plus) icon
    SquarePlus,
    ///[square-power](https://lucide.dev/icons/square-power) icon
    SquarePower,
    ///[square-radical](https://lucide.dev/icons/square-radical) icon
    SquareRadical,
    ///[square-round-corner](https://lucide.dev/icons/square-round-corner) icon
    SquareRoundCorner,
    ///[square-scissors](https://lucide.dev/icons/square-scissors) icon
    SquareScissors,
    ///[square-sigma](https://lucide.dev/icons/square-sigma) icon
    SquareSigma,
    ///[square-slash](https://lucide.dev/icons/square-slash) icon
    SquareSlash,
    ///[square-split-horizontal](https://lucide.dev/icons/square-split-horizontal) icon
    SquareSplitHorizontal,
    ///[square-split-vertical](https://lucide.dev/icons/square-split-vertical) icon
    SquareSplitVertical,
    ///[square-square](https://lucide.dev/icons/square-square) icon
    SquareSquare,
    ///[square-stack](https://lucide.dev/icons/square-stack) icon
    SquareStack,
    ///[square-terminal](https://lucide.dev/icons/square-terminal) icon
    SquareTerminal,
    ///[square-user](https://lucide.dev/icons/square-user) icon
    SquareUser,
    ///[square-user-round](https://lucide.dev/icons/square-user-round) icon
    SquareUserRound,
    ///[square-x](https://lucide.dev/icons/square-x) icon
    SquareX,
    ///[squares-exclude](https://lucide.dev/icons/squares-exclude) icon
    SquaresExclude,
    ///[squares-intersect](https://lucide.dev/icons/squares-intersect) icon
    SquaresIntersect,
    ///[squares-subtract](https://lucide.dev/icons/squares-subtract) icon
    SquaresSubtract,
    ///[squares-unite](https://lucide.dev/icons/squares-unite) icon
    SquaresUnite,
    ///[squircle](https://lucide.dev/icons/squircle) icon
    Squircle,
    ///[squircle-dashed](https://lucide.dev/icons/squircle-dashed) icon
    SquircleDashed,
    ///[squirrel](https://lucide.dev/icons/squirrel) icon
    Squirrel,
    ///[stamp](https://lucide.dev/icons/stamp) icon
    Stamp,
    ///[star](https://lucide.dev/icons/star) icon
    Star,
    ///[star-half](https://lucide.dev/icons/star-half) icon
    StarHalf,
    ///[star-off](https://lucide.dev/icons/star-off) icon
    StarOff,
    ///[step-back](https://lucide.dev/icons/step-back) icon
    StepBack,
    ///[step-forward](https://lucide.dev/icons/step-forward) icon
    StepForward,
    ///[stethoscope](https://lucide.dev/icons/stethoscope) icon
    Stethoscope,
    ///[sticker](https://lucide.dev/icons/sticker) icon
    Sticker,
    ///[sticky-note](https://lucide.dev/icons/sticky-note) icon
    StickyNote,
    ///[store](https://lucide.dev/icons/store) icon
    Store,
    ///[stretch-horizontal](https://lucide.dev/icons/stretch-horizontal) icon
    StretchHorizontal,
    ///[stretch-vertical](https://lucide.dev/icons/stretch-vertical) icon
    StretchVertical,
    ///[strikethrough](https://lucide.dev/icons/strikethrough) icon
    Strikethrough,
    ///[subscript](https://lucide.dev/icons/subscript) icon
    Subscript,
    ///[sun](https://lucide.dev/icons/sun) icon
    Sun,
    ///[sun-dim](https://lucide.dev/icons/sun-dim) icon
    SunDim,
    ///[sun-medium](https://lucide.dev/icons/sun-medium) icon
    SunMedium,
    ///[sun-moon](https://lucide.dev/icons/sun-moon) icon
    SunMoon,
    ///[sun-snow](https://lucide.dev/icons/sun-snow) icon
    SunSnow,
    ///[sunrise](https://lucide.dev/icons/sunrise) icon
    Sunrise,
    ///[sunset](https://lucide.dev/icons/sunset) icon
    Sunset,
    ///[superscript](https://lucide.dev/icons/superscript) icon
    Superscript,
    ///[swatch-book](https://lucide.dev/icons/swatch-book) icon
    SwatchBook,
    ///[swiss-franc](https://lucide.dev/icons/swiss-franc) icon
    SwissFranc,
    ///[switch-camera](https://lucide.dev/icons/switch-camera) icon
    SwitchCamera,
    ///[sword](https://lucide.dev/icons/sword) icon
    Sword,
    ///[swords](https://lucide.dev/icons/swords) icon
    Swords,
    ///[syringe](https://lucide.dev/icons/syringe) icon
    Syringe,
    ///[table](https://lucide.dev/icons/table) icon
    Table,
    ///[table-2](https://lucide.dev/icons/table-2) icon
    Table2,
    ///[table-cells-merge](https://lucide.dev/icons/table-cells-merge) icon
    TableCellsMerge,
    ///[table-cells-split](https://lucide.dev/icons/table-cells-split) icon
    TableCellsSplit,
    ///[table-columns-split](https://lucide.dev/icons/table-columns-split) icon
    TableColumnsSplit,
    ///[table-of-contents](https://lucide.dev/icons/table-of-contents) icon
    TableOfContents,
    ///[table-properties](https://lucide.dev/icons/table-properties) icon
    TableProperties,
    ///[table-rows-split](https://lucide.dev/icons/table-rows-split) icon
    TableRowsSplit,
    ///[tablet](https://lucide.dev/icons/tablet) icon
    Tablet,
    ///[tablet-smartphone](https://lucide.dev/icons/tablet-smartphone) icon
    TabletSmartphone,
    ///[tablets](https://lucide.dev/icons/tablets) icon
    Tablets,
    ///[tag](https://lucide.dev/icons/tag) icon
    Tag,
    ///[tags](https://lucide.dev/icons/tags) icon
    Tags,
    ///[tally-1](https://lucide.dev/icons/tally-1) icon
    Tally1,
    ///[tally-2](https://lucide.dev/icons/tally-2) icon
    Tally2,
    ///[tally-3](https://lucide.dev/icons/tally-3) icon
    Tally3,
    ///[tally-4](https://lucide.dev/icons/tally-4) icon
    Tally4,
    ///[tally-5](https://lucide.dev/icons/tally-5) icon
    Tally5,
    ///[tangent](https://lucide.dev/icons/tangent) icon
    Tangent,
    ///[target](https://lucide.dev/icons/target) icon
    Target,
    ///[telescope](https://lucide.dev/icons/telescope) icon
    Telescope,
    ///[tent](https://lucide.dev/icons/tent) icon
    Tent,
    ///[tent-tree](https://lucide.dev/icons/tent-tree) icon
    TentTree,
    ///[terminal](https://lucide.dev/icons/terminal) icon
    Terminal,
    ///[test-tube](https://lucide.dev/icons/test-tube) icon
    TestTube,
    ///[test-tube-diagonal](https://lucide.dev/icons/test-tube-diagonal) icon
    TestTubeDiagonal,
    ///[test-tubes](https://lucide.dev/icons/test-tubes) icon
    TestTubes,
    ///[text](https://lucide.dev/icons/text) icon
    Text,
    ///[text-cursor](https://lucide.dev/icons/text-cursor) icon
    TextCursor,
    ///[text-cursor-input](https://lucide.dev/icons/text-cursor-input) icon
    TextCursorInput,
    ///[text-quote](https://lucide.dev/icons/text-quote) icon
    TextQuote,
    ///[text-search](https://lucide.dev/icons/text-search) icon
    TextSearch,
    ///[text-select](https://lucide.dev/icons/text-select) icon
    TextSelect,
    ///[theater](https://lucide.dev/icons/theater) icon
    Theater,
    ///[thermometer](https://lucide.dev/icons/thermometer) icon
    Thermometer,
    ///[thermometer-snowflake](https://lucide.dev/icons/thermometer-snowflake) icon
    ThermometerSnowflake,
    ///[thermometer-sun](https://lucide.dev/icons/thermometer-sun) icon
    ThermometerSun,
    ///[thumbs-down](https://lucide.dev/icons/thumbs-down) icon
    ThumbsDown,
    ///[thumbs-up](https://lucide.dev/icons/thumbs-up) icon
    ThumbsUp,
    ///[ticket](https://lucide.dev/icons/ticket) icon
    Ticket,
    ///[ticket-check](https://lucide.dev/icons/ticket-check) icon
    TicketCheck,
    ///[ticket-minus](https://lucide.dev/icons/ticket-minus) icon
    TicketMinus,
    ///[ticket-percent](https://lucide.dev/icons/ticket-percent) icon
    TicketPercent,
    ///[ticket-plus](https://lucide.dev/icons/ticket-plus) icon
    TicketPlus,
    ///[ticket-slash](https://lucide.dev/icons/ticket-slash) icon
    TicketSlash,
    ///[ticket-x](https://lucide.dev/icons/ticket-x) icon
    TicketX,
    ///[tickets](https://lucide.dev/icons/tickets) icon
    Tickets,
    ///[tickets-plane](https://lucide.dev/icons/tickets-plane) icon
    TicketsPlane,
    ///[timer](https://lucide.dev/icons/timer) icon
    Timer,
    ///[timer-off](https://lucide.dev/icons/timer-off) icon
    TimerOff,
    ///[timer-reset](https://lucide.dev/icons/timer-reset) icon
    TimerReset,
    ///[toggle-left](https://lucide.dev/icons/toggle-left) icon
    ToggleLeft,
    ///[toggle-right](https://lucide.dev/icons/toggle-right) icon
    ToggleRight,
    ///[toilet](https://lucide.dev/icons/toilet) icon
    Toilet,
    ///[tool-case](https://lucide.dev/icons/tool-case) icon
    ToolCase,
    ///[tornado](https://lucide.dev/icons/tornado) icon
    Tornado,
    ///[torus](https://lucide.dev/icons/torus) icon
    Torus,
    ///[touchpad](https://lucide.dev/icons/touchpad) icon
    Touchpad,
    ///[touchpad-off](https://lucide.dev/icons/touchpad-off) icon
    TouchpadOff,
    ///[tower-control](https://lucide.dev/icons/tower-control) icon
    TowerControl,
    ///[toy-brick](https://lucide.dev/icons/toy-brick) icon
    ToyBrick,
    ///[tractor](https://lucide.dev/icons/tractor) icon
    Tractor,
    ///[traffic-cone](https://lucide.dev/icons/traffic-cone) icon
    TrafficCone,
    ///[train-front](https://lucide.dev/icons/train-front) icon
    TrainFront,
    ///[train-front-tunnel](https://lucide.dev/icons/train-front-tunnel) icon
    TrainFrontTunnel,
    ///[train-track](https://lucide.dev/icons/train-track) icon
    TrainTrack,
    ///[tram-front](https://lucide.dev/icons/tram-front) icon
    TramFront,
    ///[transgender](https://lucide.dev/icons/transgender) icon
    Transgender,
    ///[trash](https://lucide.dev/icons/trash) icon
    Trash,
    ///[trash-2](https://lucide.dev/icons/trash-2) icon
    Trash2,
    ///[tree-deciduous](https://lucide.dev/icons/tree-deciduous) icon
    TreeDeciduous,
    ///[tree-palm](https://lucide.dev/icons/tree-palm) icon
    TreePalm,
    ///[tree-pine](https://lucide.dev/icons/tree-pine) icon
    TreePine,
    ///[trees](https://lucide.dev/icons/trees) icon
    Trees,
    ///[trello](https://lucide.dev/icons/trello) icon
    Trello,
    ///[trending-down](https://lucide.dev/icons/trending-down) icon
    TrendingDown,
    ///[trending-up](https://lucide.dev/icons/trending-up) icon
    TrendingUp,
    ///[trending-up-down](https://lucide.dev/icons/trending-up-down) icon
    TrendingUpDown,
    ///[triangle](https://lucide.dev/icons/triangle) icon
    Triangle,
    ///[triangle-alert](https://lucide.dev/icons/triangle-alert) icon
    TriangleAlert,
    ///[triangle-dashed](https://lucide.dev/icons/triangle-dashed) icon
    TriangleDashed,
    ///[triangle-right](https://lucide.dev/icons/triangle-right) icon
    TriangleRight,
    ///[trophy](https://lucide.dev/icons/trophy) icon
    Trophy,
    ///[truck](https://lucide.dev/icons/truck) icon
    Truck,
    ///[truck-electric](https://lucide.dev/icons/truck-electric) icon
    TruckElectric,
    ///[turtle](https://lucide.dev/icons/turtle) icon
    Turtle,
    ///[tv](https://lucide.dev/icons/tv) icon
    Tv,
    ///[tv-minimal](https://lucide.dev/icons/tv-minimal) icon
    TvMinimal,
    ///[tv-minimal-play](https://lucide.dev/icons/tv-minimal-play) icon
    TvMinimalPlay,
    ///[twitch](https://lucide.dev/icons/twitch) icon
    Twitch,
    ///[twitter](https://lucide.dev/icons/twitter) icon
    Twitter,
    ///[type](https://lucide.dev/icons/type) icon
    Type,
    ///[type-outline](https://lucide.dev/icons/type-outline) icon
    TypeOutline,
    ///[umbrella](https://lucide.dev/icons/umbrella) icon
    Umbrella,
    ///[umbrella-off](https://lucide.dev/icons/umbrella-off) icon
    UmbrellaOff,
    ///[underline](https://lucide.dev/icons/underline) icon
    Underline,
    ///[undo](https://lucide.dev/icons/undo) icon
    Undo,
    ///[undo-2](https://lucide.dev/icons/undo-2) icon
    Undo2,
    ///[undo-dot](https://lucide.dev/icons/undo-dot) icon
    UndoDot,
    ///[unfold-horizontal](https://lucide.dev/icons/unfold-horizontal) icon
    UnfoldHorizontal,
    ///[unfold-vertical](https://lucide.dev/icons/unfold-vertical) icon
    UnfoldVertical,
    ///[ungroup](https://lucide.dev/icons/ungroup) icon
    Ungroup,
    ///[university](https://lucide.dev/icons/university) icon
    University,
    ///[unlink](https://lucide.dev/icons/unlink) icon
    Unlink,
    ///[unlink-2](https://lucide.dev/icons/unlink-2) icon
    Unlink2,
    ///[unplug](https://lucide.dev/icons/unplug) icon
    Unplug,
    ///[upload](https://lucide.dev/icons/upload) icon
    Upload,
    ///[usb](https://lucide.dev/icons/usb) icon
    Usb,
    ///[user](https://lucide.dev/icons/user) icon
    User,
    ///[user-check](https://lucide.dev/icons/user-check) icon
    UserCheck,
    ///[user-cog](https://lucide.dev/icons/user-cog) icon
    UserCog,
    ///[user-lock](https://lucide.dev/icons/user-lock) icon
    UserLock,
    ///[user-minus](https://lucide.dev/icons/user-minus) icon
    UserMinus,
    ///[user-pen](https://lucide.dev/icons/user-pen) icon
    UserPen,
    ///[user-plus](https://lucide.dev/icons/user-plus) icon
    UserPlus,
    ///[user-round](https://lucide.dev/icons/user-round) icon
    UserRound,
    ///[user-round-check](https://lucide.dev/icons/user-round-check) icon
    UserRoundCheck,
    ///[user-round-cog](https://lucide.dev/icons/user-round-cog) icon
    UserRoundCog,
    ///[user-round-minus](https://lucide.dev/icons/user-round-minus) icon
    UserRoundMinus,
    ///[user-round-pen](https://lucide.dev/icons/user-round-pen) icon
    UserRoundPen,
    ///[user-round-plus](https://lucide.dev/icons/user-round-plus) icon
    UserRoundPlus,
    ///[user-round-search](https://lucide.dev/icons/user-round-search) icon
    UserRoundSearch,
    ///[user-round-x](https://lucide.dev/icons/user-round-x) icon
    UserRoundX,
    ///[user-search](https://lucide.dev/icons/user-search) icon
    UserSearch,
    ///[user-x](https://lucide.dev/icons/user-x) icon
    UserX,
    ///[users](https://lucide.dev/icons/users) icon
    Users,
    ///[users-round](https://lucide.dev/icons/users-round) icon
    UsersRound,
    ///[utensils](https://lucide.dev/icons/utensils) icon
    Utensils,
    ///[utensils-crossed](https://lucide.dev/icons/utensils-crossed) icon
    UtensilsCrossed,
    ///[utility-pole](https://lucide.dev/icons/utility-pole) icon
    UtilityPole,
    ///[variable](https://lucide.dev/icons/variable) icon
    Variable,
    ///[vault](https://lucide.dev/icons/vault) icon
    Vault,
    ///[vector-square](https://lucide.dev/icons/vector-square) icon
    VectorSquare,
    ///[vegan](https://lucide.dev/icons/vegan) icon
    Vegan,
    ///[venetian-mask](https://lucide.dev/icons/venetian-mask) icon
    VenetianMask,
    ///[venus](https://lucide.dev/icons/venus) icon
    Venus,
    ///[venus-and-mars](https://lucide.dev/icons/venus-and-mars) icon
    VenusAndMars,
    ///[vibrate](https://lucide.dev/icons/vibrate) icon
    Vibrate,
    ///[vibrate-off](https://lucide.dev/icons/vibrate-off) icon
    VibrateOff,
    ///[video](https://lucide.dev/icons/video) icon
    Video,
    ///[video-off](https://lucide.dev/icons/video-off) icon
    VideoOff,
    ///[videotape](https://lucide.dev/icons/videotape) icon
    Videotape,
    ///[view](https://lucide.dev/icons/view) icon
    View,
    ///[voicemail](https://lucide.dev/icons/voicemail) icon
    Voicemail,
    ///[volleyball](https://lucide.dev/icons/volleyball) icon
    Volleyball,
    ///[volume](https://lucide.dev/icons/volume) icon
    Volume,
    ///[volume-1](https://lucide.dev/icons/volume-1) icon
    Volume1,
    ///[volume-2](https://lucide.dev/icons/volume-2) icon
    Volume2,
    ///[volume-off](https://lucide.dev/icons/volume-off) icon
    VolumeOff,
    ///[volume-x](https://lucide.dev/icons/volume-x) icon
    VolumeX,
    ///[vote](https://lucide.dev/icons/vote) icon
    Vote,
    ///[wallet](https://lucide.dev/icons/wallet) icon
    Wallet,
    ///[wallet-cards](https://lucide.dev/icons/wallet-cards) icon
    WalletCards,
    ///[wallet-minimal](https://lucide.dev/icons/wallet-minimal) icon
    WalletMinimal,
    ///[wallpaper](https://lucide.dev/icons/wallpaper) icon
    Wallpaper,
    ///[wand](https://lucide.dev/icons/wand) icon
    Wand,
    ///[wand-sparkles](https://lucide.dev/icons/wand-sparkles) icon
    WandSparkles,
    ///[warehouse](https://lucide.dev/icons/warehouse) icon
    Warehouse,
    ///[washing-machine](https://lucide.dev/icons/washing-machine) icon
    WashingMachine,
    ///[watch](https://lucide.dev/icons/watch) icon
    Watch,
    ///[waves](https://lucide.dev/icons/waves) icon
    Waves,
    ///[waves-ladder](https://lucide.dev/icons/waves-ladder) icon
    WavesLadder,
    ///[waypoints](https://lucide.dev/icons/waypoints) icon
    Waypoints,
    ///[webcam](https://lucide.dev/icons/webcam) icon
    Webcam,
    ///[webhook](https://lucide.dev/icons/webhook) icon
    Webhook,
    ///[webhook-off](https://lucide.dev/icons/webhook-off) icon
    WebhookOff,
    ///[weight](https://lucide.dev/icons/weight) icon
    Weight,
    ///[wheat](https://lucide.dev/icons/wheat) icon
    Wheat,
    ///[wheat-off](https://lucide.dev/icons/wheat-off) icon
    WheatOff,
    ///[whole-word](https://lucide.dev/icons/whole-word) icon
    WholeWord,
    ///[wifi](https://lucide.dev/icons/wifi) icon
    Wifi,
    ///[wifi-cog](https://lucide.dev/icons/wifi-cog) icon
    WifiCog,
    ///[wifi-high](https://lucide.dev/icons/wifi-high) icon
    WifiHigh,
    ///[wifi-low](https://lucide.dev/icons/wifi-low) icon
    WifiLow,
    ///[wifi-off](https://lucide.dev/icons/wifi-off) icon
    WifiOff,
    ///[wifi-pen](https://lucide.dev/icons/wifi-pen) icon
    WifiPen,
    ///[wifi-zero](https://lucide.dev/icons/wifi-zero) icon
    WifiZero,
    ///[wind](https://lucide.dev/icons/wind) icon
    Wind,
    ///[wind-arrow-down](https://lucide.dev/icons/wind-arrow-down) icon
    WindArrowDown,
    ///[wine](https://lucide.dev/icons/wine) icon
    Wine,
    ///[wine-off](https://lucide.dev/icons/wine-off) icon
    WineOff,
    ///[workflow](https://lucide.dev/icons/workflow) icon
    Workflow,
    ///[worm](https://lucide.dev/icons/worm) icon
    Worm,
    ///[wrap-text](https://lucide.dev/icons/wrap-text) icon
    WrapText,
    ///[wrench](https://lucide.dev/icons/wrench) icon
    Wrench,
    ///[x](https://lucide.dev/icons/x) icon
    X,
    ///[youtube](https://lucide.dev/icons/youtube) icon
    Youtube,
    ///[zap](https://lucide.dev/icons/zap) icon
    Zap,
    ///[zap-off](https://lucide.dev/icons/zap-off) icon
    ZapOff,
    ///[zoom-in](https://lucide.dev/icons/zoom-in) icon
    ZoomIn,
    ///[zoom-out](https://lucide.dev/icons/zoom-out) icon
    ZoomOut,
}
impl Icon {
    /// Unicode character of an icon
    pub fn unicode(&self) -> char {
        match self {
            Self::AArrowDown => '\u{e58a}',
            Self::AArrowUp => '\u{e58b}',
            Self::ALargeSmall => '\u{e58c}',
            Self::Accessibility => '\u{e297}',
            Self::Activity => '\u{e038}',
            Self::AirVent => '\u{e351}',
            Self::Airplay => '\u{e039}',
            Self::AlarmClock => '\u{e03a}',
            Self::AlarmClockCheck => '\u{e1ec}',
            Self::AlarmClockMinus => '\u{e1ed}',
            Self::AlarmClockOff => '\u{e23b}',
            Self::AlarmClockPlus => '\u{e1ee}',
            Self::AlarmSmoke => '\u{e580}',
            Self::Album => '\u{e03b}',
            Self::AlignCenter => '\u{e03c}',
            Self::AlignCenterHorizontal => '\u{e26c}',
            Self::AlignCenterVertical => '\u{e26d}',
            Self::AlignEndHorizontal => '\u{e26e}',
            Self::AlignEndVertical => '\u{e26f}',
            Self::AlignHorizontalDistributeCenter => '\u{e03d}',
            Self::AlignHorizontalDistributeEnd => '\u{e03e}',
            Self::AlignHorizontalDistributeStart => '\u{e03f}',
            Self::AlignHorizontalJustifyCenter => '\u{e272}',
            Self::AlignHorizontalJustifyEnd => '\u{e273}',
            Self::AlignHorizontalJustifyStart => '\u{e274}',
            Self::AlignHorizontalSpaceAround => '\u{e275}',
            Self::AlignHorizontalSpaceBetween => '\u{e276}',
            Self::AlignJustify => '\u{e040}',
            Self::AlignLeft => '\u{e041}',
            Self::AlignRight => '\u{e042}',
            Self::AlignStartHorizontal => '\u{e270}',
            Self::AlignStartVertical => '\u{e271}',
            Self::AlignVerticalDistributeCenter => '\u{e27e}',
            Self::AlignVerticalDistributeEnd => '\u{e27f}',
            Self::AlignVerticalDistributeStart => '\u{e280}',
            Self::AlignVerticalJustifyCenter => '\u{e277}',
            Self::AlignVerticalJustifyEnd => '\u{e278}',
            Self::AlignVerticalJustifyStart => '\u{e279}',
            Self::AlignVerticalSpaceAround => '\u{e27a}',
            Self::AlignVerticalSpaceBetween => '\u{e27b}',
            Self::Ambulance => '\u{e5c0}',
            Self::Ampersand => '\u{e4a1}',
            Self::Ampersands => '\u{e4a2}',
            Self::Amphora => '\u{e620}',
            Self::Anchor => '\u{e043}',
            Self::Angry => '\u{e2fc}',
            Self::Annoyed => '\u{e2fd}',
            Self::Antenna => '\u{e4e7}',
            Self::Anvil => '\u{e585}',
            Self::Aperture => '\u{e044}',
            Self::AppWindow => '\u{e42b}',
            Self::AppWindowMac => '\u{e5d7}',
            Self::Apple => '\u{e352}',
            Self::Archive => '\u{e045}',
            Self::ArchiveRestore => '\u{e2cd}',
            Self::ArchiveX => '\u{e511}',
            Self::Armchair => '\u{e2c0}',
            Self::ArrowBigDown => '\u{e1e1}',
            Self::ArrowBigDownDash => '\u{e422}',
            Self::ArrowBigLeft => '\u{e1e2}',
            Self::ArrowBigLeftDash => '\u{e423}',
            Self::ArrowBigRight => '\u{e1e3}',
            Self::ArrowBigRightDash => '\u{e424}',
            Self::ArrowBigUp => '\u{e1e4}',
            Self::ArrowBigUpDash => '\u{e425}',
            Self::ArrowDown => '\u{e046}',
            Self::ArrowDown01 => '\u{e418}',
            Self::ArrowDown10 => '\u{e419}',
            Self::ArrowDownAZ => '\u{e41a}',
            Self::ArrowDownFromLine => '\u{e459}',
            Self::ArrowDownLeft => '\u{e047}',
            Self::ArrowDownNarrowWide => '\u{e048}',
            Self::ArrowDownRight => '\u{e049}',
            Self::ArrowDownToDot => '\u{e452}',
            Self::ArrowDownToLine => '\u{e45a}',
            Self::ArrowDownUp => '\u{e04a}',
            Self::ArrowDownWideNarrow => '\u{e04b}',
            Self::ArrowDownZA => '\u{e41b}',
            Self::ArrowLeft => '\u{e04c}',
            Self::ArrowLeftFromLine => '\u{e45b}',
            Self::ArrowLeftRight => '\u{e24a}',
            Self::ArrowLeftToLine => '\u{e45c}',
            Self::ArrowRight => '\u{e04d}',
            Self::ArrowRightFromLine => '\u{e45d}',
            Self::ArrowRightLeft => '\u{e41c}',
            Self::ArrowRightToLine => '\u{e45e}',
            Self::ArrowUp => '\u{e04e}',
            Self::ArrowUp01 => '\u{e41d}',
            Self::ArrowUp10 => '\u{e41e}',
            Self::ArrowUpAZ => '\u{e41f}',
            Self::ArrowUpDown => '\u{e381}',
            Self::ArrowUpFromDot => '\u{e453}',
            Self::ArrowUpFromLine => '\u{e45f}',
            Self::ArrowUpLeft => '\u{e04f}',
            Self::ArrowUpNarrowWide => '\u{e050}',
            Self::ArrowUpRight => '\u{e051}',
            Self::ArrowUpToLine => '\u{e460}',
            Self::ArrowUpWideNarrow => '\u{e420}',
            Self::ArrowUpZA => '\u{e421}',
            Self::ArrowsUpFromLine => '\u{e4d9}',
            Self::Asterisk => '\u{e1ef}',
            Self::AtSign => '\u{e052}',
            Self::Atom => '\u{e3db}',
            Self::AudioLines => '\u{e55f}',
            Self::AudioWaveform => '\u{e560}',
            Self::Award => '\u{e053}',
            Self::Axe => '\u{e054}',
            Self::Axis3d => '\u{e2fe}',
            Self::Baby => '\u{e2ce}',
            Self::Backpack => '\u{e2c8}',
            Self::Badge => '\u{e479}',
            Self::BadgeAlert => '\u{e47a}',
            Self::BadgeCent => '\u{e514}',
            Self::BadgeCheck => '\u{e241}',
            Self::BadgeDollarSign => '\u{e47b}',
            Self::BadgeEuro => '\u{e515}',
            Self::BadgeIndianRupee => '\u{e516}',
            Self::BadgeInfo => '\u{e47c}',
            Self::BadgeJapaneseYen => '\u{e517}',
            Self::BadgeMinus => '\u{e47d}',
            Self::BadgePercent => '\u{e47e}',
            Self::BadgePlus => '\u{e47f}',
            Self::BadgePoundSterling => '\u{e518}',
            Self::BadgeQuestionMark => '\u{e480}',
            Self::BadgeRussianRuble => '\u{e519}',
            Self::BadgeSwissFranc => '\u{e51a}',
            Self::BadgeX => '\u{e481}',
            Self::BaggageClaim => '\u{e2c9}',
            Self::Ban => '\u{e055}',
            Self::Banana => '\u{e353}',
            Self::Bandage => '\u{e622}',
            Self::Banknote => '\u{e056}',
            Self::BanknoteArrowDown => '\u{e651}',
            Self::BanknoteArrowUp => '\u{e652}',
            Self::BanknoteX => '\u{e653}',
            Self::Barcode => '\u{e538}',
            Self::Barrel => '\u{e67a}',
            Self::Baseline => '\u{e285}',
            Self::Bath => '\u{e2ab}',
            Self::Battery => '\u{e057}',
            Self::BatteryCharging => '\u{e058}',
            Self::BatteryFull => '\u{e059}',
            Self::BatteryLow => '\u{e05a}',
            Self::BatteryMedium => '\u{e05b}',
            Self::BatteryPlus => '\u{e643}',
            Self::BatteryWarning => '\u{e3b0}',
            Self::Beaker => '\u{e05c}',
            Self::Bean => '\u{e393}',
            Self::BeanOff => '\u{e394}',
            Self::Bed => '\u{e2c1}',
            Self::BedDouble => '\u{e2c2}',
            Self::BedSingle => '\u{e2c3}',
            Self::Beef => '\u{e3a9}',
            Self::Beer => '\u{e2cf}',
            Self::BeerOff => '\u{e5de}',
            Self::Bell => '\u{e05d}',
            Self::BellDot => '\u{e430}',
            Self::BellElectric => '\u{e581}',
            Self::BellMinus => '\u{e1f0}',
            Self::BellOff => '\u{e05e}',
            Self::BellPlus => '\u{e1f1}',
            Self::BellRing => '\u{e224}',
            Self::BetweenHorizontalEnd => '\u{e596}',
            Self::BetweenHorizontalStart => '\u{e597}',
            Self::BetweenVerticalEnd => '\u{e598}',
            Self::BetweenVerticalStart => '\u{e599}',
            Self::BicepsFlexed => '\u{e5f0}',
            Self::Bike => '\u{e1d2}',
            Self::Binary => '\u{e1f2}',
            Self::Binoculars => '\u{e626}',
            Self::Biohazard => '\u{e446}',
            Self::Bird => '\u{e3c9}',
            Self::Bitcoin => '\u{e05f}',
            Self::Blend => '\u{e5a1}',
            Self::Blinds => '\u{e3c4}',
            Self::Blocks => '\u{e4ff}',
            Self::Bluetooth => '\u{e060}',
            Self::BluetoothConnected => '\u{e1b8}',
            Self::BluetoothOff => '\u{e1b9}',
            Self::BluetoothSearching => '\u{e1ba}',
            Self::Bold => '\u{e061}',
            Self::Bolt => '\u{e591}',
            Self::Bomb => '\u{e2ff}',
            Self::Bone => '\u{e35c}',
            Self::Book => '\u{e062}',
            Self::BookA => '\u{e549}',
            Self::BookAlert => '\u{e677}',
            Self::BookAudio => '\u{e54a}',
            Self::BookCheck => '\u{e54b}',
            Self::BookCopy => '\u{e3f1}',
            Self::BookDashed => '\u{e3f2}',
            Self::BookDown => '\u{e3f3}',
            Self::BookHeadphones => '\u{e54c}',
            Self::BookHeart => '\u{e54d}',
            Self::BookImage => '\u{e54e}',
            Self::BookKey => '\u{e3f4}',
            Self::BookLock => '\u{e3f5}',
            Self::BookMarked => '\u{e3f6}',
            Self::BookMinus => '\u{e3f7}',
            Self::BookOpen => '\u{e063}',
            Self::BookOpenCheck => '\u{e385}',
            Self::BookOpenText => '\u{e54f}',
            Self::BookPlus => '\u{e3f8}',
            Self::BookText => '\u{e550}',
            Self::BookType => '\u{e551}',
            Self::BookUp => '\u{e3f9}',
            Self::BookUp2 => '\u{e4ab}',
            Self::BookUser => '\u{e552}',
            Self::BookX => '\u{e3fa}',
            Self::Bookmark => '\u{e064}',
            Self::BookmarkCheck => '\u{e524}',
            Self::BookmarkMinus => '\u{e23c}',
            Self::BookmarkPlus => '\u{e23d}',
            Self::BookmarkX => '\u{e525}',
            Self::BoomBox => '\u{e4f3}',
            Self::Bot => '\u{e1bb}',
            Self::BotMessageSquare => '\u{e5d3}',
            Self::BotOff => '\u{e5e5}',
            Self::BottleWine => '\u{e680}',
            Self::BowArrow => '\u{e663}',
            Self::Box => '\u{e065}',
            Self::Boxes => '\u{e2d0}',
            Self::Braces => '\u{e36e}',
            Self::Brackets => '\u{e448}',
            Self::Brain => '\u{e3ca}',
            Self::BrainCircuit => '\u{e3cb}',
            Self::BrainCog => '\u{e3cc}',
            Self::BrickWall => '\u{e586}',
            Self::BrickWallFire => '\u{e658}',
            Self::Briefcase => '\u{e066}',
            Self::BriefcaseBusiness => '\u{e5da}',
            Self::BriefcaseConveyorBelt => '\u{e630}',
            Self::BriefcaseMedical => '\u{e5db}',
            Self::BringToFront => '\u{e4f4}',
            Self::Brush => '\u{e1d3}',
            Self::BrushCleaning => '\u{e66b}',
            Self::Bubbles => '\u{e659}',
            Self::Bug => '\u{e20c}',
            Self::BugOff => '\u{e512}',
            Self::BugPlay => '\u{e513}',
            Self::Building => '\u{e1cc}',
            Self::Building2 => '\u{e290}',
            Self::Bus => '\u{e1d4}',
            Self::BusFront => '\u{e500}',
            Self::Cable => '\u{e4e8}',
            Self::CableCar => '\u{e501}',
            Self::Cake => '\u{e348}',
            Self::CakeSlice => '\u{e4be}',
            Self::Calculator => '\u{e1bc}',
            Self::Calendar => '\u{e067}',
            Self::Calendar1 => '\u{e635}',
            Self::CalendarArrowDown => '\u{e603}',
            Self::CalendarArrowUp => '\u{e604}',
            Self::CalendarCheck => '\u{e2b7}',
            Self::CalendarCheck2 => '\u{e2b8}',
            Self::CalendarClock => '\u{e304}',
            Self::CalendarCog => '\u{e5f2}',
            Self::CalendarDays => '\u{e2b9}',
            Self::CalendarFold => '\u{e5b9}',
            Self::CalendarHeart => '\u{e305}',
            Self::CalendarMinus => '\u{e2ba}',
            Self::CalendarMinus2 => '\u{e5ba}',
            Self::CalendarOff => '\u{e2bb}',
            Self::CalendarPlus => '\u{e2bc}',
            Self::CalendarPlus2 => '\u{e5bb}',
            Self::CalendarRange => '\u{e2bd}',
            Self::CalendarSearch => '\u{e306}',
            Self::CalendarSync => '\u{e63b}',
            Self::CalendarX => '\u{e2be}',
            Self::CalendarX2 => '\u{e2bf}',
            Self::Camera => '\u{e068}',
            Self::CameraOff => '\u{e069}',
            Self::Candy => '\u{e395}',
            Self::CandyCane => '\u{e4bf}',
            Self::CandyOff => '\u{e396}',
            Self::Cannabis => '\u{e5d9}',
            Self::Captions => '\u{e3a8}',
            Self::CaptionsOff => '\u{e5c6}',
            Self::Car => '\u{e1d5}',
            Self::CarFront => '\u{e502}',
            Self::CarTaxiFront => '\u{e503}',
            Self::Caravan => '\u{e53e}',
            Self::CardSim => '\u{e676}',
            Self::Carrot => '\u{e25a}',
            Self::CaseLower => '\u{e3dc}',
            Self::CaseSensitive => '\u{e3dd}',
            Self::CaseUpper => '\u{e3de}',
            Self::CassetteTape => '\u{e4cf}',
            Self::Cast => '\u{e06a}',
            Self::Castle => '\u{e3e4}',
            Self::Cat => '\u{e390}',
            Self::Cctv => '\u{e582}',
            Self::ChartArea => '\u{e4d8}',
            Self::ChartBar => '\u{e2a2}',
            Self::ChartBarBig => '\u{e4ac}',
            Self::ChartBarDecreasing => '\u{e60c}',
            Self::ChartBarIncreasing => '\u{e60d}',
            Self::ChartBarStacked => '\u{e60e}',
            Self::ChartCandlestick => '\u{e4ad}',
            Self::ChartColumn => '\u{e2a3}',
            Self::ChartColumnBig => '\u{e4ae}',
            Self::ChartColumnDecreasing => '\u{e06b}',
            Self::ChartColumnIncreasing => '\u{e2a4}',
            Self::ChartColumnStacked => '\u{e60f}',
            Self::ChartGantt => '\u{e629}',
            Self::ChartLine => '\u{e2a5}',
            Self::ChartNetwork => '\u{e610}',
            Self::ChartNoAxesColumn => '\u{e06c}',
            Self::ChartNoAxesColumnDecreasing => '\u{e06d}',
            Self::ChartNoAxesColumnIncreasing => '\u{e06e}',
            Self::ChartNoAxesCombined => '\u{e611}',
            Self::ChartNoAxesGantt => '\u{e4c9}',
            Self::ChartPie => '\u{e06f}',
            Self::ChartScatter => '\u{e48f}',
            Self::ChartSpline => '\u{e612}',
            Self::Check => '\u{e070}',
            Self::CheckCheck => '\u{e392}',
            Self::CheckLine => '\u{e670}',
            Self::ChefHat => '\u{e2ac}',
            Self::Cherry => '\u{e354}',
            Self::ChevronDown => '\u{e071}',
            Self::ChevronFirst => '\u{e243}',
            Self::ChevronLast => '\u{e244}',
            Self::ChevronLeft => '\u{e072}',
            Self::ChevronRight => '\u{e073}',
            Self::ChevronUp => '\u{e074}',
            Self::ChevronsDown => '\u{e075}',
            Self::ChevronsDownUp => '\u{e228}',
            Self::ChevronsLeft => '\u{e076}',
            Self::ChevronsLeftRight => '\u{e293}',
            Self::ChevronsLeftRightEllipsis => '\u{e624}',
            Self::ChevronsRight => '\u{e077}',
            Self::ChevronsRightLeft => '\u{e294}',
            Self::ChevronsUp => '\u{e078}',
            Self::ChevronsUpDown => '\u{e211}',
            Self::Chrome => '\u{e079}',
            Self::Church => '\u{e3e5}',
            Self::Cigarette => '\u{e2c6}',
            Self::CigaretteOff => '\u{e2c7}',
            Self::Circle => '\u{e07a}',
            Self::CircleAlert => '\u{e07b}',
            Self::CircleArrowDown => '\u{e07c}',
            Self::CircleArrowLeft => '\u{e07d}',
            Self::CircleArrowOutDownLeft => '\u{e3fc}',
            Self::CircleArrowOutDownRight => '\u{e3fd}',
            Self::CircleArrowOutUpLeft => '\u{e3fe}',
            Self::CircleArrowOutUpRight => '\u{e3ff}',
            Self::CircleArrowRight => '\u{e07e}',
            Self::CircleArrowUp => '\u{e07f}',
            Self::CircleCheck => '\u{e226}',
            Self::CircleCheckBig => '\u{e080}',
            Self::CircleChevronDown => '\u{e4e2}',
            Self::CircleChevronLeft => '\u{e4e3}',
            Self::CircleChevronRight => '\u{e4e4}',
            Self::CircleChevronUp => '\u{e4e5}',
            Self::CircleDashed => '\u{e4b5}',
            Self::CircleDivide => '\u{e081}',
            Self::CircleDollarSign => '\u{e482}',
            Self::CircleDot => '\u{e349}',
            Self::CircleDotDashed => '\u{e4b6}',
            Self::CircleEllipsis => '\u{e34a}',
            Self::CircleEqual => '\u{e405}',
            Self::CircleFadingArrowUp => '\u{e61d}',
            Self::CircleFadingPlus => '\u{e5c1}',
            Self::CircleGauge => '\u{e4e6}',
            Self::CircleMinus => '\u{e082}',
            Self::CircleOff => '\u{e406}',
            Self::CircleParking => '\u{e3cd}',
            Self::CircleParkingOff => '\u{e3ce}',
            Self::CirclePause => '\u{e083}',
            Self::CirclePercent => '\u{e51f}',
            Self::CirclePlay => '\u{e084}',
            Self::CirclePlus => '\u{e085}',
            Self::CirclePoundSterling => '\u{e672}',
            Self::CirclePower => '\u{e555}',
            Self::CircleQuestionMark => '\u{e086}',
            Self::CircleSlash => '\u{e407}',
            Self::CircleSlash2 => '\u{e213}',
            Self::CircleSmall => '\u{e645}',
            Self::CircleStop => '\u{e087}',
            Self::CircleUser => '\u{e466}',
            Self::CircleUserRound => '\u{e467}',
            Self::CircleX => '\u{e088}',
            Self::CircuitBoard => '\u{e408}',
            Self::Citrus => '\u{e379}',
            Self::Clapperboard => '\u{e29b}',
            Self::Clipboard => '\u{e089}',
            Self::ClipboardCheck => '\u{e219}',
            Self::ClipboardCopy => '\u{e225}',
            Self::ClipboardList => '\u{e08a}',
            Self::ClipboardMinus => '\u{e5c3}',
            Self::ClipboardPaste => '\u{e3ec}',
            Self::ClipboardPen => '\u{e307}',
            Self::ClipboardPenLine => '\u{e308}',
            Self::ClipboardPlus => '\u{e5c4}',
            Self::ClipboardType => '\u{e309}',
            Self::ClipboardX => '\u{e222}',
            Self::Clock => '\u{e08b}',
            Self::Clock1 => '\u{e24b}',
            Self::Clock10 => '\u{e24c}',
            Self::Clock11 => '\u{e24d}',
            Self::Clock12 => '\u{e24e}',
            Self::Clock2 => '\u{e24f}',
            Self::Clock3 => '\u{e250}',
            Self::Clock4 => '\u{e251}',
            Self::Clock5 => '\u{e252}',
            Self::Clock6 => '\u{e253}',
            Self::Clock7 => '\u{e254}',
            Self::Clock8 => '\u{e255}',
            Self::Clock9 => '\u{e256}',
            Self::ClockAlert => '\u{e62f}',
            Self::ClockArrowDown => '\u{e605}',
            Self::ClockArrowUp => '\u{e606}',
            Self::ClockFading => '\u{e64f}',
            Self::ClockPlus => '\u{e66c}',
            Self::Cloud => '\u{e08c}',
            Self::CloudAlert => '\u{e638}',
            Self::CloudCheck => '\u{e673}',
            Self::CloudCog => '\u{e30a}',
            Self::CloudDownload => '\u{e08d}',
            Self::CloudDrizzle => '\u{e08e}',
            Self::CloudFog => '\u{e214}',
            Self::CloudHail => '\u{e08f}',
            Self::CloudLightning => '\u{e090}',
            Self::CloudMoon => '\u{e215}',
            Self::CloudMoonRain => '\u{e2fa}',
            Self::CloudOff => '\u{e091}',
            Self::CloudRain => '\u{e092}',
            Self::CloudRainWind => '\u{e093}',
            Self::CloudSnow => '\u{e094}',
            Self::CloudSun => '\u{e216}',
            Self::CloudSunRain => '\u{e2fb}',
            Self::CloudUpload => '\u{e095}',
            Self::Cloudy => '\u{e217}',
            Self::Clover => '\u{e096}',
            Self::Club => '\u{e49b}',
            Self::Code => '\u{e097}',
            Self::CodeXml => '\u{e206}',
            Self::Codepen => '\u{e098}',
            Self::Codesandbox => '\u{e099}',
            Self::Coffee => '\u{e09a}',
            Self::Cog => '\u{e30b}',
            Self::Coins => '\u{e09b}',
            Self::Columns2 => '\u{e09c}',
            Self::Columns3 => '\u{e09d}',
            Self::Columns3Cog => '\u{e666}',
            Self::Columns4 => '\u{e58e}',
            Self::Combine => '\u{e451}',
            Self::Command => '\u{e09e}',
            Self::Compass => '\u{e09f}',
            Self::Component => '\u{e2ad}',
            Self::Computer => '\u{e4e9}',
            Self::ConciergeBell => '\u{e37c}',
            Self::Cone => '\u{e528}',
            Self::Construction => '\u{e3b8}',
            Self::Contact => '\u{e0a0}',
            Self::ContactRound => '\u{e468}',
            Self::Container => '\u{e4da}',
            Self::Contrast => '\u{e0a1}',
            Self::Cookie => '\u{e26b}',
            Self::CookingPot => '\u{e589}',
            Self::Copy => '\u{e0a2}',
            Self::CopyCheck => '\u{e400}',
            Self::CopyMinus => '\u{e401}',
            Self::CopyPlus => '\u{e402}',
            Self::CopySlash => '\u{e403}',
            Self::CopyX => '\u{e404}',
            Self::Copyleft => '\u{e0a3}',
            Self::Copyright => '\u{e0a4}',
            Self::CornerDownLeft => '\u{e0a5}',
            Self::CornerDownRight => '\u{e0a6}',
            Self::CornerLeftDown => '\u{e0a7}',
            Self::CornerLeftUp => '\u{e0a8}',
            Self::CornerRightDown => '\u{e0a9}',
            Self::CornerRightUp => '\u{e0aa}',
            Self::CornerUpLeft => '\u{e0ab}',
            Self::CornerUpRight => '\u{e0ac}',
            Self::Cpu => '\u{e0ad}',
            Self::CreativeCommons => '\u{e3b6}',
            Self::CreditCard => '\u{e0ae}',
            Self::Croissant => '\u{e2ae}',
            Self::Crop => '\u{e0af}',
            Self::Cross => '\u{e1e5}',
            Self::Crosshair => '\u{e0b0}',
            Self::Crown => '\u{e1d6}',
            Self::Cuboid => '\u{e529}',
            Self::CupSoda => '\u{e2d1}',
            Self::Currency => '\u{e230}',
            Self::Cylinder => '\u{e52a}',
            Self::Dam => '\u{e60b}',
            Self::Database => '\u{e0b1}',
            Self::DatabaseBackup => '\u{e3af}',
            Self::DatabaseZap => '\u{e510}',
            Self::DecimalsArrowLeft => '\u{e661}',
            Self::DecimalsArrowRight => '\u{e662}',
            Self::Delete => '\u{e0b2}',
            Self::Dessert => '\u{e4c0}',
            Self::Diameter => '\u{e52b}',
            Self::Diamond => '\u{e2d2}',
            Self::DiamondMinus => '\u{e5e6}',
            Self::DiamondPercent => '\u{e520}',
            Self::DiamondPlus => '\u{e5e7}',
            Self::Dice1 => '\u{e287}',
            Self::Dice2 => '\u{e288}',
            Self::Dice3 => '\u{e289}',
            Self::Dice4 => '\u{e28a}',
            Self::Dice5 => '\u{e28b}',
            Self::Dice6 => '\u{e28c}',
            Self::Dices => '\u{e2c5}',
            Self::Diff => '\u{e30c}',
            Self::Disc => '\u{e0b3}',
            Self::Disc2 => '\u{e3fb}',
            Self::Disc3 => '\u{e499}',
            Self::DiscAlbum => '\u{e561}',
            Self::Divide => '\u{e0b4}',
            Self::Dna => '\u{e397}',
            Self::DnaOff => '\u{e398}',
            Self::Dock => '\u{e5d8}',
            Self::Dog => '\u{e391}',
            Self::DollarSign => '\u{e0b5}',
            Self::Donut => '\u{e4c1}',
            Self::DoorClosed => '\u{e3d9}',
            Self::DoorClosedLocked => '\u{e669}',
            Self::DoorOpen => '\u{e3da}',
            Self::Dot => '\u{e454}',
            Self::Download => '\u{e0b6}',
            Self::DraftingCompass => '\u{e52c}',
            Self::Drama => '\u{e526}',
            Self::Dribbble => '\u{e0b7}',
            Self::Drill => '\u{e592}',
            Self::Drone => '\u{e67b}',
            Self::Droplet => '\u{e0b8}',
            Self::DropletOff => '\u{e63d}',
            Self::Droplets => '\u{e0b9}',
            Self::Drum => '\u{e562}',
            Self::Drumstick => '\u{e25b}',
            Self::Dumbbell => '\u{e3a5}',
            Self::Ear => '\u{e386}',
            Self::EarOff => '\u{e387}',
            Self::Earth => '\u{e1f3}',
            Self::EarthLock => '\u{e5d1}',
            Self::Eclipse => '\u{e5a2}',
            Self::Egg => '\u{e25d}',
            Self::EggFried => '\u{e355}',
            Self::EggOff => '\u{e399}',
            Self::Ellipsis => '\u{e0ba}',
            Self::EllipsisVertical => '\u{e0bb}',
            Self::Equal => '\u{e1bd}',
            Self::EqualApproximately => '\u{e639}',
            Self::EqualNot => '\u{e1be}',
            Self::Eraser => '\u{e28f}',
            Self::EthernetPort => '\u{e625}',
            Self::Euro => '\u{e0bc}',
            Self::Expand => '\u{e21a}',
            Self::ExternalLink => '\u{e0bd}',
            Self::Eye => '\u{e0be}',
            Self::EyeClosed => '\u{e633}',
            Self::EyeOff => '\u{e0bf}',
            Self::Facebook => '\u{e0c0}',
            Self::Factory => '\u{e29f}',
            Self::Fan => '\u{e37d}',
            Self::FastForward => '\u{e0c1}',
            Self::Feather => '\u{e0c2}',
            Self::Fence => '\u{e587}',
            Self::FerrisWheel => '\u{e484}',
            Self::Figma => '\u{e0c3}',
            Self::File => '\u{e0c4}',
            Self::FileArchive => '\u{e30d}',
            Self::FileAudio => '\u{e30e}',
            Self::FileAudio2 => '\u{e30f}',
            Self::FileAxis3d => '\u{e310}',
            Self::FileBadge => '\u{e311}',
            Self::FileBadge2 => '\u{e312}',
            Self::FileBox => '\u{e313}',
            Self::FileChartColumn => '\u{e314}',
            Self::FileChartColumnIncreasing => '\u{e315}',
            Self::FileChartLine => '\u{e316}',
            Self::FileChartPie => '\u{e317}',
            Self::FileCheck => '\u{e0c5}',
            Self::FileCheck2 => '\u{e0c6}',
            Self::FileClock => '\u{e318}',
            Self::FileCode => '\u{e0c7}',
            Self::FileCode2 => '\u{e463}',
            Self::FileCog => '\u{e319}',
            Self::FileDiff => '\u{e31a}',
            Self::FileDigit => '\u{e0c8}',
            Self::FileDown => '\u{e31b}',
            Self::FileHeart => '\u{e31c}',
            Self::FileImage => '\u{e31d}',
            Self::FileInput => '\u{e0c9}',
            Self::FileJson => '\u{e36f}',
            Self::FileJson2 => '\u{e370}',
            Self::FileKey => '\u{e31e}',
            Self::FileKey2 => '\u{e31f}',
            Self::FileLock => '\u{e320}',
            Self::FileLock2 => '\u{e321}',
            Self::FileMinus => '\u{e0ca}',
            Self::FileMinus2 => '\u{e0cb}',
            Self::FileMusic => '\u{e563}',
            Self::FileOutput => '\u{e0cc}',
            Self::FilePen => '\u{e322}',
            Self::FilePenLine => '\u{e323}',
            Self::FilePlus => '\u{e0cd}',
            Self::FilePlus2 => '\u{e0ce}',
            Self::FileQuestionMark => '\u{e324}',
            Self::FileScan => '\u{e325}',
            Self::FileSearch => '\u{e0cf}',
            Self::FileSearch2 => '\u{e326}',
            Self::FileSliders => '\u{e5a5}',
            Self::FileSpreadsheet => '\u{e327}',
            Self::FileStack => '\u{e4a6}',
            Self::FileSymlink => '\u{e328}',
            Self::FileTerminal => '\u{e329}',
            Self::FileText => '\u{e0d0}',
            Self::FileType => '\u{e32a}',
            Self::FileType2 => '\u{e371}',
            Self::FileUp => '\u{e32b}',
            Self::FileUser => '\u{e632}',
            Self::FileVideo => '\u{e32c}',
            Self::FileVideo2 => '\u{e32d}',
            Self::FileVolume => '\u{e32e}',
            Self::FileVolume2 => '\u{e32f}',
            Self::FileWarning => '\u{e330}',
            Self::FileX => '\u{e0d1}',
            Self::FileX2 => '\u{e0d2}',
            Self::Files => '\u{e0d3}',
            Self::Film => '\u{e0d4}',
            Self::Fingerprint => '\u{e2cb}',
            Self::FireExtinguisher => '\u{e583}',
            Self::Fish => '\u{e3aa}',
            Self::FishOff => '\u{e3b4}',
            Self::FishSymbol => '\u{e4f9}',
            Self::Flag => '\u{e0d5}',
            Self::FlagOff => '\u{e292}',
            Self::FlagTriangleLeft => '\u{e237}',
            Self::FlagTriangleRight => '\u{e238}',
            Self::Flame => '\u{e0d6}',
            Self::FlameKindling => '\u{e53f}',
            Self::Flashlight => '\u{e0d7}',
            Self::FlashlightOff => '\u{e0d8}',
            Self::FlaskConical => '\u{e0d9}',
            Self::FlaskConicalOff => '\u{e39a}',
            Self::FlaskRound => '\u{e0da}',
            Self::FlipHorizontal => '\u{e361}',
            Self::FlipHorizontal2 => '\u{e362}',
            Self::FlipVertical => '\u{e363}',
            Self::FlipVertical2 => '\u{e364}',
            Self::Flower => '\u{e2d3}',
            Self::Flower2 => '\u{e2d4}',
            Self::Focus => '\u{e29e}',
            Self::FoldHorizontal => '\u{e440}',
            Self::FoldVertical => '\u{e441}',
            Self::Folder => '\u{e0db}',
            Self::FolderArchive => '\u{e331}',
            Self::FolderCheck => '\u{e332}',
            Self::FolderClock => '\u{e333}',
            Self::FolderClosed => '\u{e334}',
            Self::FolderCode => '\u{e600}',
            Self::FolderCog => '\u{e335}',
            Self::FolderDot => '\u{e4ca}',
            Self::FolderDown => '\u{e336}',
            Self::FolderGit => '\u{e40e}',
            Self::FolderGit2 => '\u{e40f}',
            Self::FolderHeart => '\u{e337}',
            Self::FolderInput => '\u{e338}',
            Self::FolderKanban => '\u{e4cb}',
            Self::FolderKey => '\u{e339}',
            Self::FolderLock => '\u{e33a}',
            Self::FolderMinus => '\u{e0dc}',
            Self::FolderOpen => '\u{e247}',
            Self::FolderOpenDot => '\u{e4cc}',
            Self::FolderOutput => '\u{e33b}',
            Self::FolderPen => '\u{e33c}',
            Self::FolderPlus => '\u{e0dd}',
            Self::FolderRoot => '\u{e4cd}',
            Self::FolderSearch => '\u{e33d}',
            Self::FolderSearch2 => '\u{e33e}',
            Self::FolderSymlink => '\u{e33f}',
            Self::FolderSync => '\u{e4ce}',
            Self::FolderTree => '\u{e340}',
            Self::FolderUp => '\u{e341}',
            Self::FolderX => '\u{e342}',
            Self::Folders => '\u{e343}',
            Self::Footprints => '\u{e3bd}',
            Self::Forklift => '\u{e3c5}',
            Self::Forward => '\u{e229}',
            Self::Frame => '\u{e291}',
            Self::Framer => '\u{e0de}',
            Self::Frown => '\u{e0df}',
            Self::Fuel => '\u{e2af}',
            Self::Fullscreen => '\u{e539}',
            Self::Funnel => '\u{e0e0}',
            Self::FunnelPlus => '\u{e0e1}',
            Self::FunnelX => '\u{e3b9}',
            Self::GalleryHorizontal => '\u{e4d3}',
            Self::GalleryHorizontalEnd => '\u{e4d4}',
            Self::GalleryThumbnails => '\u{e4d5}',
            Self::GalleryVertical => '\u{e4d6}',
            Self::GalleryVerticalEnd => '\u{e4d7}',
            Self::Gamepad => '\u{e0e2}',
            Self::Gamepad2 => '\u{e0e3}',
            Self::Gauge => '\u{e1bf}',
            Self::Gavel => '\u{e0e4}',
            Self::Gem => '\u{e242}',
            Self::GeorgianLari => '\u{e67d}',
            Self::Ghost => '\u{e20e}',
            Self::Gift => '\u{e0e5}',
            Self::GitBranch => '\u{e0e6}',
            Self::GitBranchPlus => '\u{e1f4}',
            Self::GitCommitHorizontal => '\u{e0e7}',
            Self::GitCommitVertical => '\u{e557}',
            Self::GitCompare => '\u{e35d}',
            Self::GitCompareArrows => '\u{e558}',
            Self::GitFork => '\u{e28d}',
            Self::GitGraph => '\u{e559}',
            Self::GitMerge => '\u{e0e8}',
            Self::GitPullRequest => '\u{e0e9}',
            Self::GitPullRequestArrow => '\u{e55a}',
            Self::GitPullRequestClosed => '\u{e35e}',
            Self::GitPullRequestCreate => '\u{e55b}',
            Self::GitPullRequestCreateArrow => '\u{e55c}',
            Self::GitPullRequestDraft => '\u{e35f}',
            Self::Github => '\u{e0ea}',
            Self::Gitlab => '\u{e0eb}',
            Self::GlassWater => '\u{e2d5}',
            Self::Glasses => '\u{e20d}',
            Self::Globe => '\u{e0ec}',
            Self::GlobeLock => '\u{e5d2}',
            Self::Goal => '\u{e4aa}',
            Self::Gpu => '\u{e66f}',
            Self::Grab => '\u{e1e6}',
            Self::GraduationCap => '\u{e234}',
            Self::Grape => '\u{e356}',
            Self::Grid2x2 => '\u{e504}',
            Self::Grid2x2Check => '\u{e5e9}',
            Self::Grid2x2Plus => '\u{e62d}',
            Self::Grid2x2X => '\u{e5ea}',
            Self::Grid3x2 => '\u{e674}',
            Self::Grid3x3 => '\u{e0ed}',
            Self::Grip => '\u{e3b5}',
            Self::GripHorizontal => '\u{e0ee}',
            Self::GripVertical => '\u{e0ef}',
            Self::Group => '\u{e469}',
            Self::Guitar => '\u{e564}',
            Self::Ham => '\u{e5dc}',
            Self::Hamburger => '\u{e66a}',
            Self::Hammer => '\u{e0f0}',
            Self::Hand => '\u{e1d7}',
            Self::HandCoins => '\u{e5bd}',
            Self::HandHeart => '\u{e5be}',
            Self::HandHelping => '\u{e3bc}',
            Self::HandMetal => '\u{e22c}',
            Self::HandPlatter => '\u{e5bf}',
            Self::Handshake => '\u{e5c5}',
            Self::HardDrive => '\u{e0f1}',
            Self::HardDriveDownload => '\u{e4ea}',
            Self::HardDriveUpload => '\u{e4eb}',
            Self::HardHat => '\u{e0f2}',
            Self::Hash => '\u{e0f3}',
            Self::Haze => '\u{e0f4}',
            Self::HdmiPort => '\u{e4ec}',
            Self::Heading => '\u{e388}',
            Self::Heading1 => '\u{e389}',
            Self::Heading2 => '\u{e38a}',
            Self::Heading3 => '\u{e38b}',
            Self::Heading4 => '\u{e38c}',
            Self::Heading5 => '\u{e38d}',
            Self::Heading6 => '\u{e38e}',
            Self::HeadphoneOff => '\u{e62e}',
            Self::Headphones => '\u{e0f5}',
            Self::Headset => '\u{e5c2}',
            Self::Heart => '\u{e0f6}',
            Self::HeartCrack => '\u{e2d6}',
            Self::HeartHandshake => '\u{e2d7}',
            Self::HeartMinus => '\u{e656}',
            Self::HeartOff => '\u{e295}',
            Self::HeartPlus => '\u{e657}',
            Self::HeartPulse => '\u{e372}',
            Self::Heater => '\u{e593}',
            Self::Hexagon => '\u{e0f7}',
            Self::Highlighter => '\u{e0f8}',
            Self::History => '\u{e1f5}',
            Self::Hop => '\u{e39b}',
            Self::HopOff => '\u{e39c}',
            Self::Hospital => '\u{e5dd}',
            Self::Hotel => '\u{e3e6}',
            Self::Hourglass => '\u{e296}',
            Self::House => '\u{e0f9}',
            Self::HousePlug => '\u{e5f5}',
            Self::HousePlus => '\u{e5f6}',
            Self::HouseWifi => '\u{e641}',
            Self::IceCreamBowl => '\u{e3ab}',
            Self::IceCreamCone => '\u{e357}',
            Self::IdCard => '\u{e61c}',
            Self::IdCardLanyard => '\u{e675}',
            Self::Image => '\u{e0fa}',
            Self::ImageDown => '\u{e541}',
            Self::ImageMinus => '\u{e1f6}',
            Self::ImageOff => '\u{e1c0}',
            Self::ImagePlay => '\u{e5e4}',
            Self::ImagePlus => '\u{e1f7}',
            Self::ImageUp => '\u{e5d0}',
            Self::ImageUpscale => '\u{e63c}',
            Self::Images => '\u{e5c9}',
            Self::Import => '\u{e22f}',
            Self::Inbox => '\u{e0fb}',
            Self::IndentDecrease => '\u{e0fc}',
            Self::IndentIncrease => '\u{e0fd}',
            Self::IndianRupee => '\u{e0fe}',
            Self::Infinity => '\u{e1e7}',
            Self::Info => '\u{e0ff}',
            Self::InspectionPanel => '\u{e588}',
            Self::Instagram => '\u{e100}',
            Self::Italic => '\u{e101}',
            Self::IterationCcw => '\u{e428}',
            Self::IterationCw => '\u{e429}',
            Self::JapaneseYen => '\u{e102}',
            Self::Joystick => '\u{e359}',
            Self::Kanban => '\u{e4e1}',
            Self::Key => '\u{e103}',
            Self::KeyRound => '\u{e4a8}',
            Self::KeySquare => '\u{e4a9}',
            Self::Keyboard => '\u{e284}',
            Self::KeyboardMusic => '\u{e565}',
            Self::KeyboardOff => '\u{e5e3}',
            Self::Lamp => '\u{e2d8}',
            Self::LampCeiling => '\u{e2d9}',
            Self::LampDesk => '\u{e2da}',
            Self::LampFloor => '\u{e2db}',
            Self::LampWallDown => '\u{e2dc}',
            Self::LampWallUp => '\u{e2dd}',
            Self::LandPlot => '\u{e52d}',
            Self::Landmark => '\u{e23a}',
            Self::Languages => '\u{e104}',
            Self::Laptop => '\u{e1cd}',
            Self::LaptopMinimal => '\u{e1d8}',
            Self::LaptopMinimalCheck => '\u{e637}',
            Self::Lasso => '\u{e1ce}',
            Self::LassoSelect => '\u{e1cf}',
            Self::Laugh => '\u{e300}',
            Self::Layers => '\u{e52e}',
            Self::Layers2 => '\u{e52f}',
            Self::LayoutDashboard => '\u{e1c1}',
            Self::LayoutGrid => '\u{e105}',
            Self::LayoutList => '\u{e1d9}',
            Self::LayoutPanelLeft => '\u{e475}',
            Self::LayoutPanelTop => '\u{e476}',
            Self::LayoutTemplate => '\u{e207}',
            Self::Leaf => '\u{e2de}',
            Self::LeafyGreen => '\u{e474}',
            Self::Lectern => '\u{e5ee}',
            Self::LetterText => '\u{e60a}',
            Self::Library => '\u{e106}',
            Self::LibraryBig => '\u{e553}',
            Self::LifeBuoy => '\u{e107}',
            Self::Ligature => '\u{e43f}',
            Self::Lightbulb => '\u{e1c2}',
            Self::LightbulbOff => '\u{e208}',
            Self::LineSquiggle => '\u{e67f}',
            Self::Link => '\u{e108}',
            Self::Link2 => '\u{e109}',
            Self::Link2Off => '\u{e10a}',
            Self::Linkedin => '\u{e10b}',
            Self::List => '\u{e10c}',
            Self::ListCheck => '\u{e5ff}',
            Self::ListChecks => '\u{e1d0}',
            Self::ListCollapse => '\u{e5a0}',
            Self::ListEnd => '\u{e2df}',
            Self::ListFilter => '\u{e465}',
            Self::ListFilterPlus => '\u{e63e}',
            Self::ListMinus => '\u{e23e}',
            Self::ListMusic => '\u{e2e0}',
            Self::ListOrdered => '\u{e1d1}',
            Self::ListPlus => '\u{e23f}',
            Self::ListRestart => '\u{e457}',
            Self::ListStart => '\u{e2e1}',
            Self::ListTodo => '\u{e4c8}',
            Self::ListTree => '\u{e40d}',
            Self::ListVideo => '\u{e2e2}',
            Self::ListX => '\u{e240}',
            Self::Loader => '\u{e10d}',
            Self::LoaderCircle => '\u{e10e}',
            Self::LoaderPinwheel => '\u{e5eb}',
            Self::Locate => '\u{e1da}',
            Self::LocateFixed => '\u{e1db}',
            Self::LocateOff => '\u{e282}',
            Self::LocationEdit => '\u{e65a}',
            Self::Lock => '\u{e10f}',
            Self::LockKeyhole => '\u{e536}',
            Self::LockKeyholeOpen => '\u{e537}',
            Self::LockOpen => '\u{e110}',
            Self::LogIn => '\u{e111}',
            Self::LogOut => '\u{e112}',
            Self::Logs => '\u{e5f9}',
            Self::Lollipop => '\u{e4c2}',
            Self::Luggage => '\u{e2ca}',
            Self::Magnet => '\u{e2b5}',
            Self::Mail => '\u{e113}',
            Self::MailCheck => '\u{e365}',
            Self::MailMinus => '\u{e366}',
            Self::MailOpen => '\u{e367}',
            Self::MailPlus => '\u{e368}',
            Self::MailQuestionMark => '\u{e369}',
            Self::MailSearch => '\u{e36a}',
            Self::MailWarning => '\u{e36b}',
            Self::MailX => '\u{e36c}',
            Self::Mailbox => '\u{e3d8}',
            Self::Mails => '\u{e36d}',
            Self::Map => '\u{e114}',
            Self::MapPin => '\u{e115}',
            Self::MapPinCheck => '\u{e614}',
            Self::MapPinCheckInside => '\u{e615}',
            Self::MapPinHouse => '\u{e621}',
            Self::MapPinMinus => '\u{e616}',
            Self::MapPinMinusInside => '\u{e617}',
            Self::MapPinOff => '\u{e2a6}',
            Self::MapPinPlus => '\u{e618}',
            Self::MapPinPlusInside => '\u{e619}',
            Self::MapPinX => '\u{e61a}',
            Self::MapPinXInside => '\u{e61b}',
            Self::MapPinned => '\u{e542}',
            Self::MapPlus => '\u{e644}',
            Self::Mars => '\u{e646}',
            Self::MarsStroke => '\u{e647}',
            Self::Martini => '\u{e2e3}',
            Self::Maximize => '\u{e116}',
            Self::Maximize2 => '\u{e117}',
            Self::Medal => '\u{e373}',
            Self::Megaphone => '\u{e235}',
            Self::MegaphoneOff => '\u{e374}',
            Self::Meh => '\u{e118}',
            Self::MemoryStick => '\u{e44a}',
            Self::Menu => '\u{e119}',
            Self::Merge => '\u{e444}',
            Self::MessageCircle => '\u{e11a}',
            Self::MessageCircleCode => '\u{e567}',
            Self::MessageCircleDashed => '\u{e568}',
            Self::MessageCircleHeart => '\u{e569}',
            Self::MessageCircleMore => '\u{e56a}',
            Self::MessageCircleOff => '\u{e56b}',
            Self::MessageCirclePlus => '\u{e56c}',
            Self::MessageCircleQuestionMark => '\u{e56d}',
            Self::MessageCircleReply => '\u{e56e}',
            Self::MessageCircleWarning => '\u{e56f}',
            Self::MessageCircleX => '\u{e570}',
            Self::MessageSquare => '\u{e11b}',
            Self::MessageSquareCode => '\u{e571}',
            Self::MessageSquareDashed => '\u{e410}',
            Self::MessageSquareDiff => '\u{e572}',
            Self::MessageSquareDot => '\u{e573}',
            Self::MessageSquareHeart => '\u{e574}',
            Self::MessageSquareLock => '\u{e631}',
            Self::MessageSquareMore => '\u{e575}',
            Self::MessageSquareOff => '\u{e576}',
            Self::MessageSquarePlus => '\u{e411}',
            Self::MessageSquareQuote => '\u{e577}',
            Self::MessageSquareReply => '\u{e578}',
            Self::MessageSquareShare => '\u{e579}',
            Self::MessageSquareText => '\u{e57a}',
            Self::MessageSquareWarning => '\u{e57b}',
            Self::MessageSquareX => '\u{e57c}',
            Self::MessagesSquare => '\u{e412}',
            Self::Mic => '\u{e11c}',
            Self::MicOff => '\u{e11d}',
            Self::MicVocal => '\u{e34d}',
            Self::Microchip => '\u{e61f}',
            Self::Microscope => '\u{e2e4}',
            Self::Microwave => '\u{e37e}',
            Self::Milestone => '\u{e298}',
            Self::Milk => '\u{e39d}',
            Self::MilkOff => '\u{e39e}',
            Self::Minimize => '\u{e11e}',
            Self::Minimize2 => '\u{e11f}',
            Self::Minus => '\u{e120}',
            Self::Monitor => '\u{e121}',
            Self::MonitorCheck => '\u{e487}',
            Self::MonitorCog => '\u{e608}',
            Self::MonitorDot => '\u{e488}',
            Self::MonitorDown => '\u{e426}',
            Self::MonitorOff => '\u{e1dc}',
            Self::MonitorPause => '\u{e489}',
            Self::MonitorPlay => '\u{e48a}',
            Self::MonitorSmartphone => '\u{e3a6}',
            Self::MonitorSpeaker => '\u{e210}',
            Self::MonitorStop => '\u{e48b}',
            Self::MonitorUp => '\u{e427}',
            Self::MonitorX => '\u{e48c}',
            Self::Moon => '\u{e122}',
            Self::MoonStar => '\u{e415}',
            Self::Mountain => '\u{e231}',
            Self::MountainSnow => '\u{e232}',
            Self::Mouse => '\u{e28e}',
            Self::MouseOff => '\u{e5e0}',
            Self::MousePointer => '\u{e123}',
            Self::MousePointer2 => '\u{e1c3}',
            Self::MousePointerBan => '\u{e5ec}',
            Self::MousePointerClick => '\u{e124}',
            Self::Move => '\u{e125}',
            Self::Move3d => '\u{e2e5}',
            Self::MoveDiagonal => '\u{e1c4}',
            Self::MoveDiagonal2 => '\u{e1c5}',
            Self::MoveDown => '\u{e491}',
            Self::MoveDownLeft => '\u{e492}',
            Self::MoveDownRight => '\u{e493}',
            Self::MoveHorizontal => '\u{e1c6}',
            Self::MoveLeft => '\u{e494}',
            Self::MoveRight => '\u{e495}',
            Self::MoveUp => '\u{e496}',
            Self::MoveUpLeft => '\u{e497}',
            Self::MoveUpRight => '\u{e498}',
            Self::MoveVertical => '\u{e1c7}',
            Self::Music => '\u{e126}',
            Self::Music2 => '\u{e34e}',
            Self::Music3 => '\u{e34f}',
            Self::Music4 => '\u{e350}',
            Self::Navigation => '\u{e127}',
            Self::Navigation2 => '\u{e128}',
            Self::Navigation2Off => '\u{e2a7}',
            Self::NavigationOff => '\u{e2a8}',
            Self::Network => '\u{e129}',
            Self::Newspaper => '\u{e34c}',
            Self::Nfc => '\u{e3c7}',
            Self::NonBinary => '\u{e648}',
            Self::Notebook => '\u{e59a}',
            Self::NotebookPen => '\u{e59b}',
            Self::NotebookTabs => '\u{e59c}',
            Self::NotebookText => '\u{e59d}',
            Self::NotepadText => '\u{e59e}',
            Self::NotepadTextDashed => '\u{e59f}',
            Self::Nut => '\u{e39f}',
            Self::NutOff => '\u{e3a0}',
            Self::Octagon => '\u{e12a}',
            Self::OctagonAlert => '\u{e12b}',
            Self::OctagonMinus => '\u{e62c}',
            Self::OctagonPause => '\u{e21b}',
            Self::OctagonX => '\u{e12c}',
            Self::Omega => '\u{e61e}',
            Self::Option => '\u{e1f8}',
            Self::Orbit => '\u{e3eb}',
            Self::Origami => '\u{e5e8}',
            Self::Package => '\u{e12d}',
            Self::Package2 => '\u{e344}',
            Self::PackageCheck => '\u{e266}',
            Self::PackageMinus => '\u{e267}',
            Self::PackageOpen => '\u{e2cc}',
            Self::PackagePlus => '\u{e268}',
            Self::PackageSearch => '\u{e269}',
            Self::PackageX => '\u{e26a}',
            Self::PaintBucket => '\u{e2e6}',
            Self::PaintRoller => '\u{e5a3}',
            Self::Paintbrush => '\u{e2e7}',
            Self::PaintbrushVertical => '\u{e2e8}',
            Self::Palette => '\u{e1dd}',
            Self::Panda => '\u{e66d}',
            Self::PanelBottom => '\u{e431}',
            Self::PanelBottomClose => '\u{e432}',
            Self::PanelBottomDashed => '\u{e433}',
            Self::PanelBottomOpen => '\u{e434}',
            Self::PanelLeft => '\u{e12e}',
            Self::PanelLeftClose => '\u{e21c}',
            Self::PanelLeftDashed => '\u{e435}',
            Self::PanelLeftOpen => '\u{e21d}',
            Self::PanelRight => '\u{e436}',
            Self::PanelRightClose => '\u{e437}',
            Self::PanelRightDashed => '\u{e438}',
            Self::PanelRightOpen => '\u{e439}',
            Self::PanelTop => '\u{e43a}',
            Self::PanelTopClose => '\u{e43b}',
            Self::PanelTopDashed => '\u{e43c}',
            Self::PanelTopOpen => '\u{e43d}',
            Self::PanelsLeftBottom => '\u{e12f}',
            Self::PanelsRightBottom => '\u{e58d}',
            Self::PanelsTopLeft => '\u{e130}',
            Self::Paperclip => '\u{e131}',
            Self::Parentheses => '\u{e449}',
            Self::ParkingMeter => '\u{e505}',
            Self::PartyPopper => '\u{e347}',
            Self::Pause => '\u{e132}',
            Self::PawPrint => '\u{e4fa}',
            Self::PcCase => '\u{e44b}',
            Self::Pen => '\u{e133}',
            Self::PenLine => '\u{e134}',
            Self::PenOff => '\u{e5f3}',
            Self::PenTool => '\u{e135}',
            Self::Pencil => '\u{e1f9}',
            Self::PencilLine => '\u{e4f5}',
            Self::PencilOff => '\u{e5f4}',
            Self::PencilRuler => '\u{e4f6}',
            Self::Pentagon => '\u{e530}',
            Self::Percent => '\u{e136}',
            Self::PersonStanding => '\u{e21e}',
            Self::PhilippinePeso => '\u{e609}',
            Self::Phone => '\u{e137}',
            Self::PhoneCall => '\u{e138}',
            Self::PhoneForwarded => '\u{e139}',
            Self::PhoneIncoming => '\u{e13a}',
            Self::PhoneMissed => '\u{e13b}',
            Self::PhoneOff => '\u{e13c}',
            Self::PhoneOutgoing => '\u{e13d}',
            Self::Pi => '\u{e477}',
            Self::Piano => '\u{e566}',
            Self::Pickaxe => '\u{e5cb}',
            Self::PictureInPicture => '\u{e3b2}',
            Self::PictureInPicture2 => '\u{e3b3}',
            Self::PiggyBank => '\u{e13e}',
            Self::Pilcrow => '\u{e3a7}',
            Self::PilcrowLeft => '\u{e5e1}',
            Self::PilcrowRight => '\u{e5e2}',
            Self::Pill => '\u{e3c1}',
            Self::PillBottle => '\u{e5ef}',
            Self::Pin => '\u{e259}',
            Self::PinOff => '\u{e2b6}',
            Self::Pipette => '\u{e13f}',
            Self::Pizza => '\u{e358}',
            Self::Plane => '\u{e1de}',
            Self::PlaneLanding => '\u{e3d1}',
            Self::PlaneTakeoff => '\u{e3d2}',
            Self::Play => '\u{e140}',
            Self::Plug => '\u{e383}',
            Self::Plug2 => '\u{e384}',
            Self::PlugZap => '\u{e461}',
            Self::Plus => '\u{e141}',
            Self::Pocket => '\u{e142}',
            Self::PocketKnife => '\u{e4a5}',
            Self::Podcast => '\u{e1fa}',
            Self::Pointer => '\u{e1e8}',
            Self::PointerOff => '\u{e584}',
            Self::Popcorn => '\u{e4c3}',
            Self::Popsicle => '\u{e4c4}',
            Self::PoundSterling => '\u{e143}',
            Self::Power => '\u{e144}',
            Self::PowerOff => '\u{e209}',
            Self::Presentation => '\u{e4b3}',
            Self::Printer => '\u{e145}',
            Self::PrinterCheck => '\u{e5fa}',
            Self::Projector => '\u{e4b4}',
            Self::Proportions => '\u{e5d4}',
            Self::Puzzle => '\u{e29c}',
            Self::Pyramid => '\u{e531}',
            Self::QrCode => '\u{e1df}',
            Self::Quote => '\u{e239}',
            Self::Rabbit => '\u{e4fb}',
            Self::Radar => '\u{e49c}',
            Self::Radiation => '\u{e447}',
            Self::Radical => '\u{e5c7}',
            Self::Radio => '\u{e146}',
            Self::RadioReceiver => '\u{e1fb}',
            Self::RadioTower => '\u{e409}',
            Self::Radius => '\u{e532}',
            Self::RailSymbol => '\u{e506}',
            Self::Rainbow => '\u{e4c7}',
            Self::Rat => '\u{e3f0}',
            Self::Ratio => '\u{e4ed}',
            Self::Receipt => '\u{e3d7}',
            Self::ReceiptCent => '\u{e5aa}',
            Self::ReceiptEuro => '\u{e5ab}',
            Self::ReceiptIndianRupee => '\u{e5ac}',
            Self::ReceiptJapaneseYen => '\u{e5ad}',
            Self::ReceiptPoundSterling => '\u{e5ae}',
            Self::ReceiptRussianRuble => '\u{e5af}',
            Self::ReceiptSwissFranc => '\u{e5b0}',
            Self::ReceiptText => '\u{e5b1}',
            Self::RectangleCircle => '\u{e678}',
            Self::RectangleEllipsis => '\u{e21f}',
            Self::RectangleGoggles => '\u{e65b}',
            Self::RectangleHorizontal => '\u{e37a}',
            Self::RectangleVertical => '\u{e37b}',
            Self::Recycle => '\u{e2e9}',
            Self::Redo => '\u{e147}',
            Self::Redo2 => '\u{e2a0}',
            Self::RedoDot => '\u{e455}',
            Self::RefreshCcw => '\u{e148}',
            Self::RefreshCcwDot => '\u{e4b7}',
            Self::RefreshCw => '\u{e149}',
            Self::RefreshCwOff => '\u{e49d}',
            Self::Refrigerator => '\u{e37f}',
            Self::Regex => '\u{e1fc}',
            Self::RemoveFormatting => '\u{e3b7}',
            Self::Repeat => '\u{e14a}',
            Self::Repeat1 => '\u{e1fd}',
            Self::Repeat2 => '\u{e416}',
            Self::Replace => '\u{e3df}',
            Self::ReplaceAll => '\u{e3e0}',
            Self::Reply => '\u{e22a}',
            Self::ReplyAll => '\u{e22b}',
            Self::Rewind => '\u{e14b}',
            Self::Ribbon => '\u{e55d}',
            Self::Rocket => '\u{e286}',
            Self::RockingChair => '\u{e233}',
            Self::RollerCoaster => '\u{e485}',
            Self::Rotate3d => '\u{e2ea}',
            Self::RotateCcw => '\u{e14c}',
            Self::RotateCcwKey => '\u{e655}',
            Self::RotateCcwSquare => '\u{e5d5}',
            Self::RotateCw => '\u{e14d}',
            Self::RotateCwSquare => '\u{e5d6}',
            Self::Route => '\u{e543}',
            Self::RouteOff => '\u{e544}',
            Self::Router => '\u{e3c3}',
            Self::Rows2 => '\u{e43e}',
            Self::Rows3 => '\u{e58f}',
            Self::Rows4 => '\u{e590}',
            Self::Rss => '\u{e14e}',
            Self::Ruler => '\u{e14f}',
            Self::RulerDimensionLine => '\u{e667}',
            Self::RussianRuble => '\u{e150}',
            Self::Sailboat => '\u{e382}',
            Self::Salad => '\u{e3ac}',
            Self::Sandwich => '\u{e3ad}',
            Self::Satellite => '\u{e44c}',
            Self::SatelliteDish => '\u{e44d}',
            Self::SaudiRiyal => '\u{e650}',
            Self::Save => '\u{e151}',
            Self::SaveAll => '\u{e414}',
            Self::SaveOff => '\u{e5f8}',
            Self::Scale => '\u{e212}',
            Self::Scale3d => '\u{e2eb}',
            Self::Scaling => '\u{e2ec}',
            Self::Scan => '\u{e257}',
            Self::ScanBarcode => '\u{e53a}',
            Self::ScanEye => '\u{e53b}',
            Self::ScanFace => '\u{e375}',
            Self::ScanHeart => '\u{e63f}',
            Self::ScanLine => '\u{e258}',
            Self::ScanQrCode => '\u{e5fb}',
            Self::ScanSearch => '\u{e53c}',
            Self::ScanText => '\u{e53d}',
            Self::School => '\u{e3e7}',
            Self::Scissors => '\u{e152}',
            Self::ScissorsLineDashed => '\u{e4ee}',
            Self::ScreenShare => '\u{e153}',
            Self::ScreenShareOff => '\u{e154}',
            Self::Scroll => '\u{e2ed}',
            Self::ScrollText => '\u{e464}',
            Self::Search => '\u{e155}',
            Self::SearchCheck => '\u{e4af}',
            Self::SearchCode => '\u{e4b0}',
            Self::SearchSlash => '\u{e4b1}',
            Self::SearchX => '\u{e4b2}',
            Self::Section => '\u{e5ed}',
            Self::Send => '\u{e156}',
            Self::SendHorizontal => '\u{e4f7}',
            Self::SendToBack => '\u{e4f8}',
            Self::SeparatorHorizontal => '\u{e1c8}',
            Self::SeparatorVertical => '\u{e1c9}',
            Self::Server => '\u{e157}',
            Self::ServerCog => '\u{e345}',
            Self::ServerCrash => '\u{e1e9}',
            Self::ServerOff => '\u{e1ea}',
            Self::Settings => '\u{e158}',
            Self::Settings2 => '\u{e245}',
            Self::Shapes => '\u{e4b8}',
            Self::Share => '\u{e159}',
            Self::Share2 => '\u{e15a}',
            Self::Sheet => '\u{e15b}',
            Self::Shell => '\u{e4fc}',
            Self::Shield => '\u{e15c}',
            Self::ShieldAlert => '\u{e1fe}',
            Self::ShieldBan => '\u{e15d}',
            Self::ShieldCheck => '\u{e1ff}',
            Self::ShieldEllipsis => '\u{e51b}',
            Self::ShieldHalf => '\u{e51c}',
            Self::ShieldMinus => '\u{e51d}',
            Self::ShieldOff => '\u{e15e}',
            Self::ShieldPlus => '\u{e51e}',
            Self::ShieldQuestionMark => '\u{e413}',
            Self::ShieldUser => '\u{e64c}',
            Self::ShieldX => '\u{e200}',
            Self::Ship => '\u{e3be}',
            Self::ShipWheel => '\u{e507}',
            Self::Shirt => '\u{e1ca}',
            Self::ShoppingBag => '\u{e15f}',
            Self::ShoppingBasket => '\u{e4ef}',
            Self::ShoppingCart => '\u{e160}',
            Self::Shovel => '\u{e161}',
            Self::ShowerHead => '\u{e380}',
            Self::Shredder => '\u{e660}',
            Self::Shrimp => '\u{e64e}',
            Self::Shrink => '\u{e220}',
            Self::Shrub => '\u{e2ee}',
            Self::Shuffle => '\u{e162}',
            Self::Sigma => '\u{e201}',
            Self::Signal => '\u{e25f}',
            Self::SignalHigh => '\u{e260}',
            Self::SignalLow => '\u{e261}',
            Self::SignalMedium => '\u{e262}',
            Self::SignalZero => '\u{e263}',
            Self::Signature => '\u{e5f7}',
            Self::Signpost => '\u{e545}',
            Self::SignpostBig => '\u{e546}',
            Self::Siren => '\u{e2ef}',
            Self::SkipBack => '\u{e163}',
            Self::SkipForward => '\u{e164}',
            Self::Skull => '\u{e221}',
            Self::Slack => '\u{e165}',
            Self::Slash => '\u{e522}',
            Self::Slice => '\u{e2f0}',
            Self::SlidersHorizontal => '\u{e29a}',
            Self::SlidersVertical => '\u{e166}',
            Self::Smartphone => '\u{e167}',
            Self::SmartphoneCharging => '\u{e22e}',
            Self::SmartphoneNfc => '\u{e3c8}',
            Self::Smile => '\u{e168}',
            Self::SmilePlus => '\u{e301}',
            Self::Snail => '\u{e4fd}',
            Self::Snowflake => '\u{e169}',
            Self::SoapDispenserDroplet => '\u{e66e}',
            Self::Sofa => '\u{e2c4}',
            Self::Soup => '\u{e3ae}',
            Self::Space => '\u{e3e1}',
            Self::Spade => '\u{e49e}',
            Self::Sparkle => '\u{e483}',
            Self::Sparkles => '\u{e417}',
            Self::Speaker => '\u{e16a}',
            Self::Speech => '\u{e523}',
            Self::SpellCheck => '\u{e49f}',
            Self::SpellCheck2 => '\u{e4a0}',
            Self::Spline => '\u{e38f}',
            Self::SplinePointer => '\u{e654}',
            Self::Split => '\u{e445}',
            Self::Spool => '\u{e67c}',
            Self::SprayCan => '\u{e49a}',
            Self::Sprout => '\u{e1eb}',
            Self::Square => '\u{e16b}',
            Self::SquareActivity => '\u{e4b9}',
            Self::SquareArrowDown => '\u{e42c}',
            Self::SquareArrowDownLeft => '\u{e4ba}',
            Self::SquareArrowDownRight => '\u{e4bb}',
            Self::SquareArrowLeft => '\u{e42d}',
            Self::SquareArrowOutDownLeft => '\u{e5a6}',
            Self::SquareArrowOutDownRight => '\u{e5a7}',
            Self::SquareArrowOutUpLeft => '\u{e5a8}',
            Self::SquareArrowOutUpRight => '\u{e5a9}',
            Self::SquareArrowRight => '\u{e42e}',
            Self::SquareArrowUp => '\u{e42f}',
            Self::SquareArrowUpLeft => '\u{e4bc}',
            Self::SquareArrowUpRight => '\u{e4bd}',
            Self::SquareAsterisk => '\u{e16c}',
            Self::SquareBottomDashedScissors => '\u{e4f0}',
            Self::SquareChartGantt => '\u{e16d}',
            Self::SquareCheck => '\u{e55e}',
            Self::SquareCheckBig => '\u{e16e}',
            Self::SquareChevronDown => '\u{e3d3}',
            Self::SquareChevronLeft => '\u{e3d4}',
            Self::SquareChevronRight => '\u{e3d5}',
            Self::SquareChevronUp => '\u{e3d6}',
            Self::SquareCode => '\u{e16f}',
            Self::SquareDashed => '\u{e1cb}',
            Self::SquareDashedBottom => '\u{e4c5}',
            Self::SquareDashedBottomCode => '\u{e4c6}',
            Self::SquareDashedKanban => '\u{e170}',
            Self::SquareDashedMousePointer => '\u{e50e}',
            Self::SquareDashedTopSolid => '\u{e671}',
            Self::SquareDivide => '\u{e171}',
            Self::SquareDot => '\u{e172}',
            Self::SquareEqual => '\u{e173}',
            Self::SquareFunction => '\u{e22d}',
            Self::SquareKanban => '\u{e174}',
            Self::SquareLibrary => '\u{e554}',
            Self::SquareM => '\u{e508}',
            Self::SquareMenu => '\u{e458}',
            Self::SquareMinus => '\u{e175}',
            Self::SquareMousePointer => '\u{e202}',
            Self::SquareParking => '\u{e3cf}',
            Self::SquareParkingOff => '\u{e3d0}',
            Self::SquarePen => '\u{e176}',
            Self::SquarePercent => '\u{e521}',
            Self::SquarePi => '\u{e48d}',
            Self::SquarePilcrow => '\u{e490}',
            Self::SquarePlay => '\u{e486}',
            Self::SquarePlus => '\u{e177}',
            Self::SquarePower => '\u{e556}',
            Self::SquareRadical => '\u{e5c8}',
            Self::SquareRoundCorner => '\u{e64d}',
            Self::SquareScissors => '\u{e4f1}',
            Self::SquareSigma => '\u{e48e}',
            Self::SquareSlash => '\u{e178}',
            Self::SquareSplitHorizontal => '\u{e3ba}',
            Self::SquareSplitVertical => '\u{e3bb}',
            Self::SquareSquare => '\u{e613}',
            Self::SquareStack => '\u{e4a7}',
            Self::SquareTerminal => '\u{e20a}',
            Self::SquareUser => '\u{e46a}',
            Self::SquareUserRound => '\u{e46b}',
            Self::SquareX => '\u{e179}',
            Self::SquaresExclude => '\u{e65c}',
            Self::SquaresIntersect => '\u{e65d}',
            Self::SquaresSubtract => '\u{e65e}',
            Self::SquaresUnite => '\u{e65f}',
            Self::Squircle => '\u{e57f}',
            Self::SquircleDashed => '\u{e67e}',
            Self::Squirrel => '\u{e4a4}',
            Self::Stamp => '\u{e3bf}',
            Self::Star => '\u{e17a}',
            Self::StarHalf => '\u{e20b}',
            Self::StarOff => '\u{e2b0}',
            Self::StepBack => '\u{e3ed}',
            Self::StepForward => '\u{e3ee}',
            Self::Stethoscope => '\u{e2f1}',
            Self::Sticker => '\u{e302}',
            Self::StickyNote => '\u{e303}',
            Self::Store => '\u{e3e8}',
            Self::StretchHorizontal => '\u{e27c}',
            Self::StretchVertical => '\u{e27d}',
            Self::Strikethrough => '\u{e17b}',
            Self::Subscript => '\u{e25c}',
            Self::Sun => '\u{e17c}',
            Self::SunDim => '\u{e299}',
            Self::SunMedium => '\u{e2b1}',
            Self::SunMoon => '\u{e2b2}',
            Self::SunSnow => '\u{e376}',
            Self::Sunrise => '\u{e17d}',
            Self::Sunset => '\u{e17e}',
            Self::Superscript => '\u{e25e}',
            Self::SwatchBook => '\u{e5a4}',
            Self::SwissFranc => '\u{e17f}',
            Self::SwitchCamera => '\u{e180}',
            Self::Sword => '\u{e2b3}',
            Self::Swords => '\u{e2b4}',
            Self::Syringe => '\u{e2f2}',
            Self::Table => '\u{e181}',
            Self::Table2 => '\u{e2f9}',
            Self::TableCellsMerge => '\u{e5cc}',
            Self::TableCellsSplit => '\u{e5cd}',
            Self::TableColumnsSplit => '\u{e5ce}',
            Self::TableOfContents => '\u{e623}',
            Self::TableProperties => '\u{e4e0}',
            Self::TableRowsSplit => '\u{e5cf}',
            Self::Tablet => '\u{e182}',
            Self::TabletSmartphone => '\u{e50f}',
            Self::Tablets => '\u{e3c2}',
            Self::Tag => '\u{e183}',
            Self::Tags => '\u{e360}',
            Self::Tally1 => '\u{e4db}',
            Self::Tally2 => '\u{e4dc}',
            Self::Tally3 => '\u{e4dd}',
            Self::Tally4 => '\u{e4de}',
            Self::Tally5 => '\u{e4df}',
            Self::Tangent => '\u{e533}',
            Self::Target => '\u{e184}',
            Self::Telescope => '\u{e5ca}',
            Self::Tent => '\u{e227}',
            Self::TentTree => '\u{e540}',
            Self::Terminal => '\u{e185}',
            Self::TestTube => '\u{e40a}',
            Self::TestTubeDiagonal => '\u{e40b}',
            Self::TestTubes => '\u{e40c}',
            Self::Text => '\u{e3ef}',
            Self::TextCursor => '\u{e264}',
            Self::TextCursorInput => '\u{e265}',
            Self::TextQuote => '\u{e4a3}',
            Self::TextSearch => '\u{e5b2}',
            Self::TextSelect => '\u{e3e2}',
            Self::Theater => '\u{e527}',
            Self::Thermometer => '\u{e186}',
            Self::ThermometerSnowflake => '\u{e187}',
            Self::ThermometerSun => '\u{e188}',
            Self::ThumbsDown => '\u{e189}',
            Self::ThumbsUp => '\u{e18a}',
            Self::Ticket => '\u{e20f}',
            Self::TicketCheck => '\u{e5b3}',
            Self::TicketMinus => '\u{e5b4}',
            Self::TicketPercent => '\u{e5b5}',
            Self::TicketPlus => '\u{e5b6}',
            Self::TicketSlash => '\u{e5b7}',
            Self::TicketX => '\u{e5b8}',
            Self::Tickets => '\u{e627}',
            Self::TicketsPlane => '\u{e628}',
            Self::Timer => '\u{e1e0}',
            Self::TimerOff => '\u{e249}',
            Self::TimerReset => '\u{e236}',
            Self::ToggleLeft => '\u{e18b}',
            Self::ToggleRight => '\u{e18c}',
            Self::Toilet => '\u{e63a}',
            Self::ToolCase => '\u{e682}',
            Self::Tornado => '\u{e218}',
            Self::Torus => '\u{e534}',
            Self::Touchpad => '\u{e44e}',
            Self::TouchpadOff => '\u{e44f}',
            Self::TowerControl => '\u{e3c0}',
            Self::ToyBrick => '\u{e34b}',
            Self::Tractor => '\u{e509}',
            Self::TrafficCone => '\u{e50a}',
            Self::TrainFront => '\u{e50b}',
            Self::TrainFrontTunnel => '\u{e50c}',
            Self::TrainTrack => '\u{e50d}',
            Self::TramFront => '\u{e2a9}',
            Self::Transgender => '\u{e649}',
            Self::Trash => '\u{e18d}',
            Self::Trash2 => '\u{e18e}',
            Self::TreeDeciduous => '\u{e2f3}',
            Self::TreePalm => '\u{e281}',
            Self::TreePine => '\u{e2f4}',
            Self::Trees => '\u{e2f5}',
            Self::Trello => '\u{e18f}',
            Self::TrendingDown => '\u{e190}',
            Self::TrendingUp => '\u{e191}',
            Self::TrendingUpDown => '\u{e62a}',
            Self::Triangle => '\u{e192}',
            Self::TriangleAlert => '\u{e193}',
            Self::TriangleDashed => '\u{e642}',
            Self::TriangleRight => '\u{e4f2}',
            Self::Trophy => '\u{e377}',
            Self::Truck => '\u{e194}',
            Self::TruckElectric => '\u{e664}',
            Self::Turtle => '\u{e4fe}',
            Self::Tv => '\u{e195}',
            Self::TvMinimal => '\u{e203}',
            Self::TvMinimalPlay => '\u{e5f1}',
            Self::Twitch => '\u{e196}',
            Self::Twitter => '\u{e197}',
            Self::Type => '\u{e198}',
            Self::TypeOutline => '\u{e607}',
            Self::Umbrella => '\u{e199}',
            Self::UmbrellaOff => '\u{e548}',
            Self::Underline => '\u{e19a}',
            Self::Undo => '\u{e19b}',
            Self::Undo2 => '\u{e2a1}',
            Self::UndoDot => '\u{e456}',
            Self::UnfoldHorizontal => '\u{e442}',
            Self::UnfoldVertical => '\u{e443}',
            Self::Ungroup => '\u{e46c}',
            Self::University => '\u{e3e9}',
            Self::Unlink => '\u{e19c}',
            Self::Unlink2 => '\u{e19d}',
            Self::Unplug => '\u{e462}',
            Self::Upload => '\u{e19e}',
            Self::Usb => '\u{e35a}',
            Self::User => '\u{e19f}',
            Self::UserCheck => '\u{e1a0}',
            Self::UserCog => '\u{e346}',
            Self::UserLock => '\u{e665}',
            Self::UserMinus => '\u{e1a1}',
            Self::UserPen => '\u{e601}',
            Self::UserPlus => '\u{e1a2}',
            Self::UserRound => '\u{e46d}',
            Self::UserRoundCheck => '\u{e46e}',
            Self::UserRoundCog => '\u{e46f}',
            Self::UserRoundMinus => '\u{e470}',
            Self::UserRoundPen => '\u{e602}',
            Self::UserRoundPlus => '\u{e471}',
            Self::UserRoundSearch => '\u{e57d}',
            Self::UserRoundX => '\u{e472}',
            Self::UserSearch => '\u{e57e}',
            Self::UserX => '\u{e1a3}',
            Self::Users => '\u{e1a4}',
            Self::UsersRound => '\u{e473}',
            Self::Utensils => '\u{e2f6}',
            Self::UtensilsCrossed => '\u{e2f7}',
            Self::UtilityPole => '\u{e3c6}',
            Self::Variable => '\u{e478}',
            Self::Vault => '\u{e594}',
            Self::VectorSquare => '\u{e681}',
            Self::Vegan => '\u{e3a1}',
            Self::VenetianMask => '\u{e2aa}',
            Self::Venus => '\u{e64a}',
            Self::VenusAndMars => '\u{e64b}',
            Self::Vibrate => '\u{e223}',
            Self::VibrateOff => '\u{e29d}',
            Self::Video => '\u{e1a5}',
            Self::VideoOff => '\u{e1a6}',
            Self::Videotape => '\u{e4d0}',
            Self::View => '\u{e1a7}',
            Self::Voicemail => '\u{e1a8}',
            Self::Volleyball => '\u{e634}',
            Self::Volume => '\u{e1a9}',
            Self::Volume1 => '\u{e1aa}',
            Self::Volume2 => '\u{e1ab}',
            Self::VolumeOff => '\u{e62b}',
            Self::VolumeX => '\u{e1ac}',
            Self::Vote => '\u{e3b1}',
            Self::Wallet => '\u{e204}',
            Self::WalletCards => '\u{e4d1}',
            Self::WalletMinimal => '\u{e4d2}',
            Self::Wallpaper => '\u{e450}',
            Self::Wand => '\u{e246}',
            Self::WandSparkles => '\u{e35b}',
            Self::Warehouse => '\u{e3ea}',
            Self::WashingMachine => '\u{e595}',
            Self::Watch => '\u{e1ad}',
            Self::Waves => '\u{e283}',
            Self::WavesLadder => '\u{e640}',
            Self::Waypoints => '\u{e547}',
            Self::Webcam => '\u{e205}',
            Self::Webhook => '\u{e378}',
            Self::WebhookOff => '\u{e5bc}',
            Self::Weight => '\u{e535}',
            Self::Wheat => '\u{e3a2}',
            Self::WheatOff => '\u{e3a3}',
            Self::WholeWord => '\u{e3e3}',
            Self::Wifi => '\u{e1ae}',
            Self::WifiCog => '\u{e679}',
            Self::WifiHigh => '\u{e5fc}',
            Self::WifiLow => '\u{e5fd}',
            Self::WifiOff => '\u{e1af}',
            Self::WifiPen => '\u{e668}',
            Self::WifiZero => '\u{e5fe}',
            Self::Wind => '\u{e1b0}',
            Self::WindArrowDown => '\u{e636}',
            Self::Wine => '\u{e2f8}',
            Self::WineOff => '\u{e3a4}',
            Self::Workflow => '\u{e42a}',
            Self::Worm => '\u{e5df}',
            Self::WrapText => '\u{e248}',
            Self::Wrench => '\u{e1b1}',
            Self::X => '\u{e1b2}',
            Self::Youtube => '\u{e1b3}',
            Self::Zap => '\u{e1b4}',
            Self::ZapOff => '\u{e1b5}',
            Self::ZoomIn => '\u{e1b6}',
            Self::ZoomOut => '\u{e1b7}',
        }
    }
    /// Get an icon from it's name
    ///
    /// The names need to be all-lowercase-dashed (e.g. `app-window`)
    pub fn from_name(icon_name: &str) -> Option<Self> {
        match icon_name {
            "a-arrow-down" => Some(Icon::AArrowDown),
            "a-arrow-up" => Some(Icon::AArrowUp),
            "a-large-small" => Some(Icon::ALargeSmall),
            "accessibility" => Some(Icon::Accessibility),
            "activity" => Some(Icon::Activity),
            "air-vent" => Some(Icon::AirVent),
            "airplay" => Some(Icon::Airplay),
            "alarm-clock" => Some(Icon::AlarmClock),
            "alarm-clock-check" => Some(Icon::AlarmClockCheck),
            "alarm-clock-minus" => Some(Icon::AlarmClockMinus),
            "alarm-clock-off" => Some(Icon::AlarmClockOff),
            "alarm-clock-plus" => Some(Icon::AlarmClockPlus),
            "alarm-smoke" => Some(Icon::AlarmSmoke),
            "album" => Some(Icon::Album),
            "align-center" => Some(Icon::AlignCenter),
            "align-center-horizontal" => Some(Icon::AlignCenterHorizontal),
            "align-center-vertical" => Some(Icon::AlignCenterVertical),
            "align-end-horizontal" => Some(Icon::AlignEndHorizontal),
            "align-end-vertical" => Some(Icon::AlignEndVertical),
            "align-horizontal-distribute-center" => Some(Icon::AlignHorizontalDistributeCenter),
            "align-horizontal-distribute-end" => Some(Icon::AlignHorizontalDistributeEnd),
            "align-horizontal-distribute-start" => Some(Icon::AlignHorizontalDistributeStart),
            "align-horizontal-justify-center" => Some(Icon::AlignHorizontalJustifyCenter),
            "align-horizontal-justify-end" => Some(Icon::AlignHorizontalJustifyEnd),
            "align-horizontal-justify-start" => Some(Icon::AlignHorizontalJustifyStart),
            "align-horizontal-space-around" => Some(Icon::AlignHorizontalSpaceAround),
            "align-horizontal-space-between" => Some(Icon::AlignHorizontalSpaceBetween),
            "align-justify" => Some(Icon::AlignJustify),
            "align-left" => Some(Icon::AlignLeft),
            "align-right" => Some(Icon::AlignRight),
            "align-start-horizontal" => Some(Icon::AlignStartHorizontal),
            "align-start-vertical" => Some(Icon::AlignStartVertical),
            "align-vertical-distribute-center" => Some(Icon::AlignVerticalDistributeCenter),
            "align-vertical-distribute-end" => Some(Icon::AlignVerticalDistributeEnd),
            "align-vertical-distribute-start" => Some(Icon::AlignVerticalDistributeStart),
            "align-vertical-justify-center" => Some(Icon::AlignVerticalJustifyCenter),
            "align-vertical-justify-end" => Some(Icon::AlignVerticalJustifyEnd),
            "align-vertical-justify-start" => Some(Icon::AlignVerticalJustifyStart),
            "align-vertical-space-around" => Some(Icon::AlignVerticalSpaceAround),
            "align-vertical-space-between" => Some(Icon::AlignVerticalSpaceBetween),
            "ambulance" => Some(Icon::Ambulance),
            "ampersand" => Some(Icon::Ampersand),
            "ampersands" => Some(Icon::Ampersands),
            "amphora" => Some(Icon::Amphora),
            "anchor" => Some(Icon::Anchor),
            "angry" => Some(Icon::Angry),
            "annoyed" => Some(Icon::Annoyed),
            "antenna" => Some(Icon::Antenna),
            "anvil" => Some(Icon::Anvil),
            "aperture" => Some(Icon::Aperture),
            "app-window" => Some(Icon::AppWindow),
            "app-window-mac" => Some(Icon::AppWindowMac),
            "apple" => Some(Icon::Apple),
            "archive" => Some(Icon::Archive),
            "archive-restore" => Some(Icon::ArchiveRestore),
            "archive-x" => Some(Icon::ArchiveX),
            "armchair" => Some(Icon::Armchair),
            "arrow-big-down" => Some(Icon::ArrowBigDown),
            "arrow-big-down-dash" => Some(Icon::ArrowBigDownDash),
            "arrow-big-left" => Some(Icon::ArrowBigLeft),
            "arrow-big-left-dash" => Some(Icon::ArrowBigLeftDash),
            "arrow-big-right" => Some(Icon::ArrowBigRight),
            "arrow-big-right-dash" => Some(Icon::ArrowBigRightDash),
            "arrow-big-up" => Some(Icon::ArrowBigUp),
            "arrow-big-up-dash" => Some(Icon::ArrowBigUpDash),
            "arrow-down" => Some(Icon::ArrowDown),
            "arrow-down-0-1" => Some(Icon::ArrowDown01),
            "arrow-down-1-0" => Some(Icon::ArrowDown10),
            "arrow-down-a-z" => Some(Icon::ArrowDownAZ),
            "arrow-down-from-line" => Some(Icon::ArrowDownFromLine),
            "arrow-down-left" => Some(Icon::ArrowDownLeft),
            "arrow-down-narrow-wide" => Some(Icon::ArrowDownNarrowWide),
            "arrow-down-right" => Some(Icon::ArrowDownRight),
            "arrow-down-to-dot" => Some(Icon::ArrowDownToDot),
            "arrow-down-to-line" => Some(Icon::ArrowDownToLine),
            "arrow-down-up" => Some(Icon::ArrowDownUp),
            "arrow-down-wide-narrow" => Some(Icon::ArrowDownWideNarrow),
            "arrow-down-z-a" => Some(Icon::ArrowDownZA),
            "arrow-left" => Some(Icon::ArrowLeft),
            "arrow-left-from-line" => Some(Icon::ArrowLeftFromLine),
            "arrow-left-right" => Some(Icon::ArrowLeftRight),
            "arrow-left-to-line" => Some(Icon::ArrowLeftToLine),
            "arrow-right" => Some(Icon::ArrowRight),
            "arrow-right-from-line" => Some(Icon::ArrowRightFromLine),
            "arrow-right-left" => Some(Icon::ArrowRightLeft),
            "arrow-right-to-line" => Some(Icon::ArrowRightToLine),
            "arrow-up" => Some(Icon::ArrowUp),
            "arrow-up-0-1" => Some(Icon::ArrowUp01),
            "arrow-up-1-0" => Some(Icon::ArrowUp10),
            "arrow-up-a-z" => Some(Icon::ArrowUpAZ),
            "arrow-up-down" => Some(Icon::ArrowUpDown),
            "arrow-up-from-dot" => Some(Icon::ArrowUpFromDot),
            "arrow-up-from-line" => Some(Icon::ArrowUpFromLine),
            "arrow-up-left" => Some(Icon::ArrowUpLeft),
            "arrow-up-narrow-wide" => Some(Icon::ArrowUpNarrowWide),
            "arrow-up-right" => Some(Icon::ArrowUpRight),
            "arrow-up-to-line" => Some(Icon::ArrowUpToLine),
            "arrow-up-wide-narrow" => Some(Icon::ArrowUpWideNarrow),
            "arrow-up-z-a" => Some(Icon::ArrowUpZA),
            "arrows-up-from-line" => Some(Icon::ArrowsUpFromLine),
            "asterisk" => Some(Icon::Asterisk),
            "at-sign" => Some(Icon::AtSign),
            "atom" => Some(Icon::Atom),
            "audio-lines" => Some(Icon::AudioLines),
            "audio-waveform" => Some(Icon::AudioWaveform),
            "award" => Some(Icon::Award),
            "axe" => Some(Icon::Axe),
            "axis-3d" => Some(Icon::Axis3d),
            "baby" => Some(Icon::Baby),
            "backpack" => Some(Icon::Backpack),
            "badge" => Some(Icon::Badge),
            "badge-alert" => Some(Icon::BadgeAlert),
            "badge-cent" => Some(Icon::BadgeCent),
            "badge-check" => Some(Icon::BadgeCheck),
            "badge-dollar-sign" => Some(Icon::BadgeDollarSign),
            "badge-euro" => Some(Icon::BadgeEuro),
            "badge-indian-rupee" => Some(Icon::BadgeIndianRupee),
            "badge-info" => Some(Icon::BadgeInfo),
            "badge-japanese-yen" => Some(Icon::BadgeJapaneseYen),
            "badge-minus" => Some(Icon::BadgeMinus),
            "badge-percent" => Some(Icon::BadgePercent),
            "badge-plus" => Some(Icon::BadgePlus),
            "badge-pound-sterling" => Some(Icon::BadgePoundSterling),
            "badge-question-mark" => Some(Icon::BadgeQuestionMark),
            "badge-russian-ruble" => Some(Icon::BadgeRussianRuble),
            "badge-swiss-franc" => Some(Icon::BadgeSwissFranc),
            "badge-x" => Some(Icon::BadgeX),
            "baggage-claim" => Some(Icon::BaggageClaim),
            "ban" => Some(Icon::Ban),
            "banana" => Some(Icon::Banana),
            "bandage" => Some(Icon::Bandage),
            "banknote" => Some(Icon::Banknote),
            "banknote-arrow-down" => Some(Icon::BanknoteArrowDown),
            "banknote-arrow-up" => Some(Icon::BanknoteArrowUp),
            "banknote-x" => Some(Icon::BanknoteX),
            "barcode" => Some(Icon::Barcode),
            "barrel" => Some(Icon::Barrel),
            "baseline" => Some(Icon::Baseline),
            "bath" => Some(Icon::Bath),
            "battery" => Some(Icon::Battery),
            "battery-charging" => Some(Icon::BatteryCharging),
            "battery-full" => Some(Icon::BatteryFull),
            "battery-low" => Some(Icon::BatteryLow),
            "battery-medium" => Some(Icon::BatteryMedium),
            "battery-plus" => Some(Icon::BatteryPlus),
            "battery-warning" => Some(Icon::BatteryWarning),
            "beaker" => Some(Icon::Beaker),
            "bean" => Some(Icon::Bean),
            "bean-off" => Some(Icon::BeanOff),
            "bed" => Some(Icon::Bed),
            "bed-double" => Some(Icon::BedDouble),
            "bed-single" => Some(Icon::BedSingle),
            "beef" => Some(Icon::Beef),
            "beer" => Some(Icon::Beer),
            "beer-off" => Some(Icon::BeerOff),
            "bell" => Some(Icon::Bell),
            "bell-dot" => Some(Icon::BellDot),
            "bell-electric" => Some(Icon::BellElectric),
            "bell-minus" => Some(Icon::BellMinus),
            "bell-off" => Some(Icon::BellOff),
            "bell-plus" => Some(Icon::BellPlus),
            "bell-ring" => Some(Icon::BellRing),
            "between-horizontal-end" => Some(Icon::BetweenHorizontalEnd),
            "between-horizontal-start" => Some(Icon::BetweenHorizontalStart),
            "between-vertical-end" => Some(Icon::BetweenVerticalEnd),
            "between-vertical-start" => Some(Icon::BetweenVerticalStart),
            "biceps-flexed" => Some(Icon::BicepsFlexed),
            "bike" => Some(Icon::Bike),
            "binary" => Some(Icon::Binary),
            "binoculars" => Some(Icon::Binoculars),
            "biohazard" => Some(Icon::Biohazard),
            "bird" => Some(Icon::Bird),
            "bitcoin" => Some(Icon::Bitcoin),
            "blend" => Some(Icon::Blend),
            "blinds" => Some(Icon::Blinds),
            "blocks" => Some(Icon::Blocks),
            "bluetooth" => Some(Icon::Bluetooth),
            "bluetooth-connected" => Some(Icon::BluetoothConnected),
            "bluetooth-off" => Some(Icon::BluetoothOff),
            "bluetooth-searching" => Some(Icon::BluetoothSearching),
            "bold" => Some(Icon::Bold),
            "bolt" => Some(Icon::Bolt),
            "bomb" => Some(Icon::Bomb),
            "bone" => Some(Icon::Bone),
            "book" => Some(Icon::Book),
            "book-a" => Some(Icon::BookA),
            "book-alert" => Some(Icon::BookAlert),
            "book-audio" => Some(Icon::BookAudio),
            "book-check" => Some(Icon::BookCheck),
            "book-copy" => Some(Icon::BookCopy),
            "book-dashed" => Some(Icon::BookDashed),
            "book-down" => Some(Icon::BookDown),
            "book-headphones" => Some(Icon::BookHeadphones),
            "book-heart" => Some(Icon::BookHeart),
            "book-image" => Some(Icon::BookImage),
            "book-key" => Some(Icon::BookKey),
            "book-lock" => Some(Icon::BookLock),
            "book-marked" => Some(Icon::BookMarked),
            "book-minus" => Some(Icon::BookMinus),
            "book-open" => Some(Icon::BookOpen),
            "book-open-check" => Some(Icon::BookOpenCheck),
            "book-open-text" => Some(Icon::BookOpenText),
            "book-plus" => Some(Icon::BookPlus),
            "book-text" => Some(Icon::BookText),
            "book-type" => Some(Icon::BookType),
            "book-up" => Some(Icon::BookUp),
            "book-up-2" => Some(Icon::BookUp2),
            "book-user" => Some(Icon::BookUser),
            "book-x" => Some(Icon::BookX),
            "bookmark" => Some(Icon::Bookmark),
            "bookmark-check" => Some(Icon::BookmarkCheck),
            "bookmark-minus" => Some(Icon::BookmarkMinus),
            "bookmark-plus" => Some(Icon::BookmarkPlus),
            "bookmark-x" => Some(Icon::BookmarkX),
            "boom-box" => Some(Icon::BoomBox),
            "bot" => Some(Icon::Bot),
            "bot-message-square" => Some(Icon::BotMessageSquare),
            "bot-off" => Some(Icon::BotOff),
            "bottle-wine" => Some(Icon::BottleWine),
            "bow-arrow" => Some(Icon::BowArrow),
            "box" => Some(Icon::Box),
            "boxes" => Some(Icon::Boxes),
            "braces" => Some(Icon::Braces),
            "brackets" => Some(Icon::Brackets),
            "brain" => Some(Icon::Brain),
            "brain-circuit" => Some(Icon::BrainCircuit),
            "brain-cog" => Some(Icon::BrainCog),
            "brick-wall" => Some(Icon::BrickWall),
            "brick-wall-fire" => Some(Icon::BrickWallFire),
            "briefcase" => Some(Icon::Briefcase),
            "briefcase-business" => Some(Icon::BriefcaseBusiness),
            "briefcase-conveyor-belt" => Some(Icon::BriefcaseConveyorBelt),
            "briefcase-medical" => Some(Icon::BriefcaseMedical),
            "bring-to-front" => Some(Icon::BringToFront),
            "brush" => Some(Icon::Brush),
            "brush-cleaning" => Some(Icon::BrushCleaning),
            "bubbles" => Some(Icon::Bubbles),
            "bug" => Some(Icon::Bug),
            "bug-off" => Some(Icon::BugOff),
            "bug-play" => Some(Icon::BugPlay),
            "building" => Some(Icon::Building),
            "building-2" => Some(Icon::Building2),
            "bus" => Some(Icon::Bus),
            "bus-front" => Some(Icon::BusFront),
            "cable" => Some(Icon::Cable),
            "cable-car" => Some(Icon::CableCar),
            "cake" => Some(Icon::Cake),
            "cake-slice" => Some(Icon::CakeSlice),
            "calculator" => Some(Icon::Calculator),
            "calendar" => Some(Icon::Calendar),
            "calendar-1" => Some(Icon::Calendar1),
            "calendar-arrow-down" => Some(Icon::CalendarArrowDown),
            "calendar-arrow-up" => Some(Icon::CalendarArrowUp),
            "calendar-check" => Some(Icon::CalendarCheck),
            "calendar-check-2" => Some(Icon::CalendarCheck2),
            "calendar-clock" => Some(Icon::CalendarClock),
            "calendar-cog" => Some(Icon::CalendarCog),
            "calendar-days" => Some(Icon::CalendarDays),
            "calendar-fold" => Some(Icon::CalendarFold),
            "calendar-heart" => Some(Icon::CalendarHeart),
            "calendar-minus" => Some(Icon::CalendarMinus),
            "calendar-minus-2" => Some(Icon::CalendarMinus2),
            "calendar-off" => Some(Icon::CalendarOff),
            "calendar-plus" => Some(Icon::CalendarPlus),
            "calendar-plus-2" => Some(Icon::CalendarPlus2),
            "calendar-range" => Some(Icon::CalendarRange),
            "calendar-search" => Some(Icon::CalendarSearch),
            "calendar-sync" => Some(Icon::CalendarSync),
            "calendar-x" => Some(Icon::CalendarX),
            "calendar-x-2" => Some(Icon::CalendarX2),
            "camera" => Some(Icon::Camera),
            "camera-off" => Some(Icon::CameraOff),
            "candy" => Some(Icon::Candy),
            "candy-cane" => Some(Icon::CandyCane),
            "candy-off" => Some(Icon::CandyOff),
            "cannabis" => Some(Icon::Cannabis),
            "captions" => Some(Icon::Captions),
            "captions-off" => Some(Icon::CaptionsOff),
            "car" => Some(Icon::Car),
            "car-front" => Some(Icon::CarFront),
            "car-taxi-front" => Some(Icon::CarTaxiFront),
            "caravan" => Some(Icon::Caravan),
            "card-sim" => Some(Icon::CardSim),
            "carrot" => Some(Icon::Carrot),
            "case-lower" => Some(Icon::CaseLower),
            "case-sensitive" => Some(Icon::CaseSensitive),
            "case-upper" => Some(Icon::CaseUpper),
            "cassette-tape" => Some(Icon::CassetteTape),
            "cast" => Some(Icon::Cast),
            "castle" => Some(Icon::Castle),
            "cat" => Some(Icon::Cat),
            "cctv" => Some(Icon::Cctv),
            "chart-area" => Some(Icon::ChartArea),
            "chart-bar" => Some(Icon::ChartBar),
            "chart-bar-big" => Some(Icon::ChartBarBig),
            "chart-bar-decreasing" => Some(Icon::ChartBarDecreasing),
            "chart-bar-increasing" => Some(Icon::ChartBarIncreasing),
            "chart-bar-stacked" => Some(Icon::ChartBarStacked),
            "chart-candlestick" => Some(Icon::ChartCandlestick),
            "chart-column" => Some(Icon::ChartColumn),
            "chart-column-big" => Some(Icon::ChartColumnBig),
            "chart-column-decreasing" => Some(Icon::ChartColumnDecreasing),
            "chart-column-increasing" => Some(Icon::ChartColumnIncreasing),
            "chart-column-stacked" => Some(Icon::ChartColumnStacked),
            "chart-gantt" => Some(Icon::ChartGantt),
            "chart-line" => Some(Icon::ChartLine),
            "chart-network" => Some(Icon::ChartNetwork),
            "chart-no-axes-column" => Some(Icon::ChartNoAxesColumn),
            "chart-no-axes-column-decreasing" => Some(Icon::ChartNoAxesColumnDecreasing),
            "chart-no-axes-column-increasing" => Some(Icon::ChartNoAxesColumnIncreasing),
            "chart-no-axes-combined" => Some(Icon::ChartNoAxesCombined),
            "chart-no-axes-gantt" => Some(Icon::ChartNoAxesGantt),
            "chart-pie" => Some(Icon::ChartPie),
            "chart-scatter" => Some(Icon::ChartScatter),
            "chart-spline" => Some(Icon::ChartSpline),
            "check" => Some(Icon::Check),
            "check-check" => Some(Icon::CheckCheck),
            "check-line" => Some(Icon::CheckLine),
            "chef-hat" => Some(Icon::ChefHat),
            "cherry" => Some(Icon::Cherry),
            "chevron-down" => Some(Icon::ChevronDown),
            "chevron-first" => Some(Icon::ChevronFirst),
            "chevron-last" => Some(Icon::ChevronLast),
            "chevron-left" => Some(Icon::ChevronLeft),
            "chevron-right" => Some(Icon::ChevronRight),
            "chevron-up" => Some(Icon::ChevronUp),
            "chevrons-down" => Some(Icon::ChevronsDown),
            "chevrons-down-up" => Some(Icon::ChevronsDownUp),
            "chevrons-left" => Some(Icon::ChevronsLeft),
            "chevrons-left-right" => Some(Icon::ChevronsLeftRight),
            "chevrons-left-right-ellipsis" => Some(Icon::ChevronsLeftRightEllipsis),
            "chevrons-right" => Some(Icon::ChevronsRight),
            "chevrons-right-left" => Some(Icon::ChevronsRightLeft),
            "chevrons-up" => Some(Icon::ChevronsUp),
            "chevrons-up-down" => Some(Icon::ChevronsUpDown),
            "chrome" => Some(Icon::Chrome),
            "church" => Some(Icon::Church),
            "cigarette" => Some(Icon::Cigarette),
            "cigarette-off" => Some(Icon::CigaretteOff),
            "circle" => Some(Icon::Circle),
            "circle-alert" => Some(Icon::CircleAlert),
            "circle-arrow-down" => Some(Icon::CircleArrowDown),
            "circle-arrow-left" => Some(Icon::CircleArrowLeft),
            "circle-arrow-out-down-left" => Some(Icon::CircleArrowOutDownLeft),
            "circle-arrow-out-down-right" => Some(Icon::CircleArrowOutDownRight),
            "circle-arrow-out-up-left" => Some(Icon::CircleArrowOutUpLeft),
            "circle-arrow-out-up-right" => Some(Icon::CircleArrowOutUpRight),
            "circle-arrow-right" => Some(Icon::CircleArrowRight),
            "circle-arrow-up" => Some(Icon::CircleArrowUp),
            "circle-check" => Some(Icon::CircleCheck),
            "circle-check-big" => Some(Icon::CircleCheckBig),
            "circle-chevron-down" => Some(Icon::CircleChevronDown),
            "circle-chevron-left" => Some(Icon::CircleChevronLeft),
            "circle-chevron-right" => Some(Icon::CircleChevronRight),
            "circle-chevron-up" => Some(Icon::CircleChevronUp),
            "circle-dashed" => Some(Icon::CircleDashed),
            "circle-divide" => Some(Icon::CircleDivide),
            "circle-dollar-sign" => Some(Icon::CircleDollarSign),
            "circle-dot" => Some(Icon::CircleDot),
            "circle-dot-dashed" => Some(Icon::CircleDotDashed),
            "circle-ellipsis" => Some(Icon::CircleEllipsis),
            "circle-equal" => Some(Icon::CircleEqual),
            "circle-fading-arrow-up" => Some(Icon::CircleFadingArrowUp),
            "circle-fading-plus" => Some(Icon::CircleFadingPlus),
            "circle-gauge" => Some(Icon::CircleGauge),
            "circle-minus" => Some(Icon::CircleMinus),
            "circle-off" => Some(Icon::CircleOff),
            "circle-parking" => Some(Icon::CircleParking),
            "circle-parking-off" => Some(Icon::CircleParkingOff),
            "circle-pause" => Some(Icon::CirclePause),
            "circle-percent" => Some(Icon::CirclePercent),
            "circle-play" => Some(Icon::CirclePlay),
            "circle-plus" => Some(Icon::CirclePlus),
            "circle-pound-sterling" => Some(Icon::CirclePoundSterling),
            "circle-power" => Some(Icon::CirclePower),
            "circle-question-mark" => Some(Icon::CircleQuestionMark),
            "circle-slash" => Some(Icon::CircleSlash),
            "circle-slash-2" => Some(Icon::CircleSlash2),
            "circle-small" => Some(Icon::CircleSmall),
            "circle-stop" => Some(Icon::CircleStop),
            "circle-user" => Some(Icon::CircleUser),
            "circle-user-round" => Some(Icon::CircleUserRound),
            "circle-x" => Some(Icon::CircleX),
            "circuit-board" => Some(Icon::CircuitBoard),
            "citrus" => Some(Icon::Citrus),
            "clapperboard" => Some(Icon::Clapperboard),
            "clipboard" => Some(Icon::Clipboard),
            "clipboard-check" => Some(Icon::ClipboardCheck),
            "clipboard-copy" => Some(Icon::ClipboardCopy),
            "clipboard-list" => Some(Icon::ClipboardList),
            "clipboard-minus" => Some(Icon::ClipboardMinus),
            "clipboard-paste" => Some(Icon::ClipboardPaste),
            "clipboard-pen" => Some(Icon::ClipboardPen),
            "clipboard-pen-line" => Some(Icon::ClipboardPenLine),
            "clipboard-plus" => Some(Icon::ClipboardPlus),
            "clipboard-type" => Some(Icon::ClipboardType),
            "clipboard-x" => Some(Icon::ClipboardX),
            "clock" => Some(Icon::Clock),
            "clock-1" => Some(Icon::Clock1),
            "clock-10" => Some(Icon::Clock10),
            "clock-11" => Some(Icon::Clock11),
            "clock-12" => Some(Icon::Clock12),
            "clock-2" => Some(Icon::Clock2),
            "clock-3" => Some(Icon::Clock3),
            "clock-4" => Some(Icon::Clock4),
            "clock-5" => Some(Icon::Clock5),
            "clock-6" => Some(Icon::Clock6),
            "clock-7" => Some(Icon::Clock7),
            "clock-8" => Some(Icon::Clock8),
            "clock-9" => Some(Icon::Clock9),
            "clock-alert" => Some(Icon::ClockAlert),
            "clock-arrow-down" => Some(Icon::ClockArrowDown),
            "clock-arrow-up" => Some(Icon::ClockArrowUp),
            "clock-fading" => Some(Icon::ClockFading),
            "clock-plus" => Some(Icon::ClockPlus),
            "cloud" => Some(Icon::Cloud),
            "cloud-alert" => Some(Icon::CloudAlert),
            "cloud-check" => Some(Icon::CloudCheck),
            "cloud-cog" => Some(Icon::CloudCog),
            "cloud-download" => Some(Icon::CloudDownload),
            "cloud-drizzle" => Some(Icon::CloudDrizzle),
            "cloud-fog" => Some(Icon::CloudFog),
            "cloud-hail" => Some(Icon::CloudHail),
            "cloud-lightning" => Some(Icon::CloudLightning),
            "cloud-moon" => Some(Icon::CloudMoon),
            "cloud-moon-rain" => Some(Icon::CloudMoonRain),
            "cloud-off" => Some(Icon::CloudOff),
            "cloud-rain" => Some(Icon::CloudRain),
            "cloud-rain-wind" => Some(Icon::CloudRainWind),
            "cloud-snow" => Some(Icon::CloudSnow),
            "cloud-sun" => Some(Icon::CloudSun),
            "cloud-sun-rain" => Some(Icon::CloudSunRain),
            "cloud-upload" => Some(Icon::CloudUpload),
            "cloudy" => Some(Icon::Cloudy),
            "clover" => Some(Icon::Clover),
            "club" => Some(Icon::Club),
            "code" => Some(Icon::Code),
            "code-xml" => Some(Icon::CodeXml),
            "codepen" => Some(Icon::Codepen),
            "codesandbox" => Some(Icon::Codesandbox),
            "coffee" => Some(Icon::Coffee),
            "cog" => Some(Icon::Cog),
            "coins" => Some(Icon::Coins),
            "columns-2" => Some(Icon::Columns2),
            "columns-3" => Some(Icon::Columns3),
            "columns-3-cog" => Some(Icon::Columns3Cog),
            "columns-4" => Some(Icon::Columns4),
            "combine" => Some(Icon::Combine),
            "command" => Some(Icon::Command),
            "compass" => Some(Icon::Compass),
            "component" => Some(Icon::Component),
            "computer" => Some(Icon::Computer),
            "concierge-bell" => Some(Icon::ConciergeBell),
            "cone" => Some(Icon::Cone),
            "construction" => Some(Icon::Construction),
            "contact" => Some(Icon::Contact),
            "contact-round" => Some(Icon::ContactRound),
            "container" => Some(Icon::Container),
            "contrast" => Some(Icon::Contrast),
            "cookie" => Some(Icon::Cookie),
            "cooking-pot" => Some(Icon::CookingPot),
            "copy" => Some(Icon::Copy),
            "copy-check" => Some(Icon::CopyCheck),
            "copy-minus" => Some(Icon::CopyMinus),
            "copy-plus" => Some(Icon::CopyPlus),
            "copy-slash" => Some(Icon::CopySlash),
            "copy-x" => Some(Icon::CopyX),
            "copyleft" => Some(Icon::Copyleft),
            "copyright" => Some(Icon::Copyright),
            "corner-down-left" => Some(Icon::CornerDownLeft),
            "corner-down-right" => Some(Icon::CornerDownRight),
            "corner-left-down" => Some(Icon::CornerLeftDown),
            "corner-left-up" => Some(Icon::CornerLeftUp),
            "corner-right-down" => Some(Icon::CornerRightDown),
            "corner-right-up" => Some(Icon::CornerRightUp),
            "corner-up-left" => Some(Icon::CornerUpLeft),
            "corner-up-right" => Some(Icon::CornerUpRight),
            "cpu" => Some(Icon::Cpu),
            "creative-commons" => Some(Icon::CreativeCommons),
            "credit-card" => Some(Icon::CreditCard),
            "croissant" => Some(Icon::Croissant),
            "crop" => Some(Icon::Crop),
            "cross" => Some(Icon::Cross),
            "crosshair" => Some(Icon::Crosshair),
            "crown" => Some(Icon::Crown),
            "cuboid" => Some(Icon::Cuboid),
            "cup-soda" => Some(Icon::CupSoda),
            "currency" => Some(Icon::Currency),
            "cylinder" => Some(Icon::Cylinder),
            "dam" => Some(Icon::Dam),
            "database" => Some(Icon::Database),
            "database-backup" => Some(Icon::DatabaseBackup),
            "database-zap" => Some(Icon::DatabaseZap),
            "decimals-arrow-left" => Some(Icon::DecimalsArrowLeft),
            "decimals-arrow-right" => Some(Icon::DecimalsArrowRight),
            "delete" => Some(Icon::Delete),
            "dessert" => Some(Icon::Dessert),
            "diameter" => Some(Icon::Diameter),
            "diamond" => Some(Icon::Diamond),
            "diamond-minus" => Some(Icon::DiamondMinus),
            "diamond-percent" => Some(Icon::DiamondPercent),
            "diamond-plus" => Some(Icon::DiamondPlus),
            "dice-1" => Some(Icon::Dice1),
            "dice-2" => Some(Icon::Dice2),
            "dice-3" => Some(Icon::Dice3),
            "dice-4" => Some(Icon::Dice4),
            "dice-5" => Some(Icon::Dice5),
            "dice-6" => Some(Icon::Dice6),
            "dices" => Some(Icon::Dices),
            "diff" => Some(Icon::Diff),
            "disc" => Some(Icon::Disc),
            "disc-2" => Some(Icon::Disc2),
            "disc-3" => Some(Icon::Disc3),
            "disc-album" => Some(Icon::DiscAlbum),
            "divide" => Some(Icon::Divide),
            "dna" => Some(Icon::Dna),
            "dna-off" => Some(Icon::DnaOff),
            "dock" => Some(Icon::Dock),
            "dog" => Some(Icon::Dog),
            "dollar-sign" => Some(Icon::DollarSign),
            "donut" => Some(Icon::Donut),
            "door-closed" => Some(Icon::DoorClosed),
            "door-closed-locked" => Some(Icon::DoorClosedLocked),
            "door-open" => Some(Icon::DoorOpen),
            "dot" => Some(Icon::Dot),
            "download" => Some(Icon::Download),
            "drafting-compass" => Some(Icon::DraftingCompass),
            "drama" => Some(Icon::Drama),
            "dribbble" => Some(Icon::Dribbble),
            "drill" => Some(Icon::Drill),
            "drone" => Some(Icon::Drone),
            "droplet" => Some(Icon::Droplet),
            "droplet-off" => Some(Icon::DropletOff),
            "droplets" => Some(Icon::Droplets),
            "drum" => Some(Icon::Drum),
            "drumstick" => Some(Icon::Drumstick),
            "dumbbell" => Some(Icon::Dumbbell),
            "ear" => Some(Icon::Ear),
            "ear-off" => Some(Icon::EarOff),
            "earth" => Some(Icon::Earth),
            "earth-lock" => Some(Icon::EarthLock),
            "eclipse" => Some(Icon::Eclipse),
            "egg" => Some(Icon::Egg),
            "egg-fried" => Some(Icon::EggFried),
            "egg-off" => Some(Icon::EggOff),
            "ellipsis" => Some(Icon::Ellipsis),
            "ellipsis-vertical" => Some(Icon::EllipsisVertical),
            "equal" => Some(Icon::Equal),
            "equal-approximately" => Some(Icon::EqualApproximately),
            "equal-not" => Some(Icon::EqualNot),
            "eraser" => Some(Icon::Eraser),
            "ethernet-port" => Some(Icon::EthernetPort),
            "euro" => Some(Icon::Euro),
            "expand" => Some(Icon::Expand),
            "external-link" => Some(Icon::ExternalLink),
            "eye" => Some(Icon::Eye),
            "eye-closed" => Some(Icon::EyeClosed),
            "eye-off" => Some(Icon::EyeOff),
            "facebook" => Some(Icon::Facebook),
            "factory" => Some(Icon::Factory),
            "fan" => Some(Icon::Fan),
            "fast-forward" => Some(Icon::FastForward),
            "feather" => Some(Icon::Feather),
            "fence" => Some(Icon::Fence),
            "ferris-wheel" => Some(Icon::FerrisWheel),
            "figma" => Some(Icon::Figma),
            "file" => Some(Icon::File),
            "file-archive" => Some(Icon::FileArchive),
            "file-audio" => Some(Icon::FileAudio),
            "file-audio-2" => Some(Icon::FileAudio2),
            "file-axis-3d" => Some(Icon::FileAxis3d),
            "file-badge" => Some(Icon::FileBadge),
            "file-badge-2" => Some(Icon::FileBadge2),
            "file-box" => Some(Icon::FileBox),
            "file-chart-column" => Some(Icon::FileChartColumn),
            "file-chart-column-increasing" => Some(Icon::FileChartColumnIncreasing),
            "file-chart-line" => Some(Icon::FileChartLine),
            "file-chart-pie" => Some(Icon::FileChartPie),
            "file-check" => Some(Icon::FileCheck),
            "file-check-2" => Some(Icon::FileCheck2),
            "file-clock" => Some(Icon::FileClock),
            "file-code" => Some(Icon::FileCode),
            "file-code-2" => Some(Icon::FileCode2),
            "file-cog" => Some(Icon::FileCog),
            "file-diff" => Some(Icon::FileDiff),
            "file-digit" => Some(Icon::FileDigit),
            "file-down" => Some(Icon::FileDown),
            "file-heart" => Some(Icon::FileHeart),
            "file-image" => Some(Icon::FileImage),
            "file-input" => Some(Icon::FileInput),
            "file-json" => Some(Icon::FileJson),
            "file-json-2" => Some(Icon::FileJson2),
            "file-key" => Some(Icon::FileKey),
            "file-key-2" => Some(Icon::FileKey2),
            "file-lock" => Some(Icon::FileLock),
            "file-lock-2" => Some(Icon::FileLock2),
            "file-minus" => Some(Icon::FileMinus),
            "file-minus-2" => Some(Icon::FileMinus2),
            "file-music" => Some(Icon::FileMusic),
            "file-output" => Some(Icon::FileOutput),
            "file-pen" => Some(Icon::FilePen),
            "file-pen-line" => Some(Icon::FilePenLine),
            "file-plus" => Some(Icon::FilePlus),
            "file-plus-2" => Some(Icon::FilePlus2),
            "file-question-mark" => Some(Icon::FileQuestionMark),
            "file-scan" => Some(Icon::FileScan),
            "file-search" => Some(Icon::FileSearch),
            "file-search-2" => Some(Icon::FileSearch2),
            "file-sliders" => Some(Icon::FileSliders),
            "file-spreadsheet" => Some(Icon::FileSpreadsheet),
            "file-stack" => Some(Icon::FileStack),
            "file-symlink" => Some(Icon::FileSymlink),
            "file-terminal" => Some(Icon::FileTerminal),
            "file-text" => Some(Icon::FileText),
            "file-type" => Some(Icon::FileType),
            "file-type-2" => Some(Icon::FileType2),
            "file-up" => Some(Icon::FileUp),
            "file-user" => Some(Icon::FileUser),
            "file-video" => Some(Icon::FileVideo),
            "file-video-2" => Some(Icon::FileVideo2),
            "file-volume" => Some(Icon::FileVolume),
            "file-volume-2" => Some(Icon::FileVolume2),
            "file-warning" => Some(Icon::FileWarning),
            "file-x" => Some(Icon::FileX),
            "file-x-2" => Some(Icon::FileX2),
            "files" => Some(Icon::Files),
            "film" => Some(Icon::Film),
            "fingerprint" => Some(Icon::Fingerprint),
            "fire-extinguisher" => Some(Icon::FireExtinguisher),
            "fish" => Some(Icon::Fish),
            "fish-off" => Some(Icon::FishOff),
            "fish-symbol" => Some(Icon::FishSymbol),
            "flag" => Some(Icon::Flag),
            "flag-off" => Some(Icon::FlagOff),
            "flag-triangle-left" => Some(Icon::FlagTriangleLeft),
            "flag-triangle-right" => Some(Icon::FlagTriangleRight),
            "flame" => Some(Icon::Flame),
            "flame-kindling" => Some(Icon::FlameKindling),
            "flashlight" => Some(Icon::Flashlight),
            "flashlight-off" => Some(Icon::FlashlightOff),
            "flask-conical" => Some(Icon::FlaskConical),
            "flask-conical-off" => Some(Icon::FlaskConicalOff),
            "flask-round" => Some(Icon::FlaskRound),
            "flip-horizontal" => Some(Icon::FlipHorizontal),
            "flip-horizontal-2" => Some(Icon::FlipHorizontal2),
            "flip-vertical" => Some(Icon::FlipVertical),
            "flip-vertical-2" => Some(Icon::FlipVertical2),
            "flower" => Some(Icon::Flower),
            "flower-2" => Some(Icon::Flower2),
            "focus" => Some(Icon::Focus),
            "fold-horizontal" => Some(Icon::FoldHorizontal),
            "fold-vertical" => Some(Icon::FoldVertical),
            "folder" => Some(Icon::Folder),
            "folder-archive" => Some(Icon::FolderArchive),
            "folder-check" => Some(Icon::FolderCheck),
            "folder-clock" => Some(Icon::FolderClock),
            "folder-closed" => Some(Icon::FolderClosed),
            "folder-code" => Some(Icon::FolderCode),
            "folder-cog" => Some(Icon::FolderCog),
            "folder-dot" => Some(Icon::FolderDot),
            "folder-down" => Some(Icon::FolderDown),
            "folder-git" => Some(Icon::FolderGit),
            "folder-git-2" => Some(Icon::FolderGit2),
            "folder-heart" => Some(Icon::FolderHeart),
            "folder-input" => Some(Icon::FolderInput),
            "folder-kanban" => Some(Icon::FolderKanban),
            "folder-key" => Some(Icon::FolderKey),
            "folder-lock" => Some(Icon::FolderLock),
            "folder-minus" => Some(Icon::FolderMinus),
            "folder-open" => Some(Icon::FolderOpen),
            "folder-open-dot" => Some(Icon::FolderOpenDot),
            "folder-output" => Some(Icon::FolderOutput),
            "folder-pen" => Some(Icon::FolderPen),
            "folder-plus" => Some(Icon::FolderPlus),
            "folder-root" => Some(Icon::FolderRoot),
            "folder-search" => Some(Icon::FolderSearch),
            "folder-search-2" => Some(Icon::FolderSearch2),
            "folder-symlink" => Some(Icon::FolderSymlink),
            "folder-sync" => Some(Icon::FolderSync),
            "folder-tree" => Some(Icon::FolderTree),
            "folder-up" => Some(Icon::FolderUp),
            "folder-x" => Some(Icon::FolderX),
            "folders" => Some(Icon::Folders),
            "footprints" => Some(Icon::Footprints),
            "forklift" => Some(Icon::Forklift),
            "forward" => Some(Icon::Forward),
            "frame" => Some(Icon::Frame),
            "framer" => Some(Icon::Framer),
            "frown" => Some(Icon::Frown),
            "fuel" => Some(Icon::Fuel),
            "fullscreen" => Some(Icon::Fullscreen),
            "funnel" => Some(Icon::Funnel),
            "funnel-plus" => Some(Icon::FunnelPlus),
            "funnel-x" => Some(Icon::FunnelX),
            "gallery-horizontal" => Some(Icon::GalleryHorizontal),
            "gallery-horizontal-end" => Some(Icon::GalleryHorizontalEnd),
            "gallery-thumbnails" => Some(Icon::GalleryThumbnails),
            "gallery-vertical" => Some(Icon::GalleryVertical),
            "gallery-vertical-end" => Some(Icon::GalleryVerticalEnd),
            "gamepad" => Some(Icon::Gamepad),
            "gamepad-2" => Some(Icon::Gamepad2),
            "gauge" => Some(Icon::Gauge),
            "gavel" => Some(Icon::Gavel),
            "gem" => Some(Icon::Gem),
            "georgian-lari" => Some(Icon::GeorgianLari),
            "ghost" => Some(Icon::Ghost),
            "gift" => Some(Icon::Gift),
            "git-branch" => Some(Icon::GitBranch),
            "git-branch-plus" => Some(Icon::GitBranchPlus),
            "git-commit-horizontal" => Some(Icon::GitCommitHorizontal),
            "git-commit-vertical" => Some(Icon::GitCommitVertical),
            "git-compare" => Some(Icon::GitCompare),
            "git-compare-arrows" => Some(Icon::GitCompareArrows),
            "git-fork" => Some(Icon::GitFork),
            "git-graph" => Some(Icon::GitGraph),
            "git-merge" => Some(Icon::GitMerge),
            "git-pull-request" => Some(Icon::GitPullRequest),
            "git-pull-request-arrow" => Some(Icon::GitPullRequestArrow),
            "git-pull-request-closed" => Some(Icon::GitPullRequestClosed),
            "git-pull-request-create" => Some(Icon::GitPullRequestCreate),
            "git-pull-request-create-arrow" => Some(Icon::GitPullRequestCreateArrow),
            "git-pull-request-draft" => Some(Icon::GitPullRequestDraft),
            "github" => Some(Icon::Github),
            "gitlab" => Some(Icon::Gitlab),
            "glass-water" => Some(Icon::GlassWater),
            "glasses" => Some(Icon::Glasses),
            "globe" => Some(Icon::Globe),
            "globe-lock" => Some(Icon::GlobeLock),
            "goal" => Some(Icon::Goal),
            "gpu" => Some(Icon::Gpu),
            "grab" => Some(Icon::Grab),
            "graduation-cap" => Some(Icon::GraduationCap),
            "grape" => Some(Icon::Grape),
            "grid-2x2" => Some(Icon::Grid2x2),
            "grid-2x2-check" => Some(Icon::Grid2x2Check),
            "grid-2x2-plus" => Some(Icon::Grid2x2Plus),
            "grid-2x2-x" => Some(Icon::Grid2x2X),
            "grid-3x2" => Some(Icon::Grid3x2),
            "grid-3x3" => Some(Icon::Grid3x3),
            "grip" => Some(Icon::Grip),
            "grip-horizontal" => Some(Icon::GripHorizontal),
            "grip-vertical" => Some(Icon::GripVertical),
            "group" => Some(Icon::Group),
            "guitar" => Some(Icon::Guitar),
            "ham" => Some(Icon::Ham),
            "hamburger" => Some(Icon::Hamburger),
            "hammer" => Some(Icon::Hammer),
            "hand" => Some(Icon::Hand),
            "hand-coins" => Some(Icon::HandCoins),
            "hand-heart" => Some(Icon::HandHeart),
            "hand-helping" => Some(Icon::HandHelping),
            "hand-metal" => Some(Icon::HandMetal),
            "hand-platter" => Some(Icon::HandPlatter),
            "handshake" => Some(Icon::Handshake),
            "hard-drive" => Some(Icon::HardDrive),
            "hard-drive-download" => Some(Icon::HardDriveDownload),
            "hard-drive-upload" => Some(Icon::HardDriveUpload),
            "hard-hat" => Some(Icon::HardHat),
            "hash" => Some(Icon::Hash),
            "haze" => Some(Icon::Haze),
            "hdmi-port" => Some(Icon::HdmiPort),
            "heading" => Some(Icon::Heading),
            "heading-1" => Some(Icon::Heading1),
            "heading-2" => Some(Icon::Heading2),
            "heading-3" => Some(Icon::Heading3),
            "heading-4" => Some(Icon::Heading4),
            "heading-5" => Some(Icon::Heading5),
            "heading-6" => Some(Icon::Heading6),
            "headphone-off" => Some(Icon::HeadphoneOff),
            "headphones" => Some(Icon::Headphones),
            "headset" => Some(Icon::Headset),
            "heart" => Some(Icon::Heart),
            "heart-crack" => Some(Icon::HeartCrack),
            "heart-handshake" => Some(Icon::HeartHandshake),
            "heart-minus" => Some(Icon::HeartMinus),
            "heart-off" => Some(Icon::HeartOff),
            "heart-plus" => Some(Icon::HeartPlus),
            "heart-pulse" => Some(Icon::HeartPulse),
            "heater" => Some(Icon::Heater),
            "hexagon" => Some(Icon::Hexagon),
            "highlighter" => Some(Icon::Highlighter),
            "history" => Some(Icon::History),
            "hop" => Some(Icon::Hop),
            "hop-off" => Some(Icon::HopOff),
            "hospital" => Some(Icon::Hospital),
            "hotel" => Some(Icon::Hotel),
            "hourglass" => Some(Icon::Hourglass),
            "house" => Some(Icon::House),
            "house-plug" => Some(Icon::HousePlug),
            "house-plus" => Some(Icon::HousePlus),
            "house-wifi" => Some(Icon::HouseWifi),
            "ice-cream-bowl" => Some(Icon::IceCreamBowl),
            "ice-cream-cone" => Some(Icon::IceCreamCone),
            "id-card" => Some(Icon::IdCard),
            "id-card-lanyard" => Some(Icon::IdCardLanyard),
            "image" => Some(Icon::Image),
            "image-down" => Some(Icon::ImageDown),
            "image-minus" => Some(Icon::ImageMinus),
            "image-off" => Some(Icon::ImageOff),
            "image-play" => Some(Icon::ImagePlay),
            "image-plus" => Some(Icon::ImagePlus),
            "image-up" => Some(Icon::ImageUp),
            "image-upscale" => Some(Icon::ImageUpscale),
            "images" => Some(Icon::Images),
            "import" => Some(Icon::Import),
            "inbox" => Some(Icon::Inbox),
            "indent-decrease" => Some(Icon::IndentDecrease),
            "indent-increase" => Some(Icon::IndentIncrease),
            "indian-rupee" => Some(Icon::IndianRupee),
            "infinity" => Some(Icon::Infinity),
            "info" => Some(Icon::Info),
            "inspection-panel" => Some(Icon::InspectionPanel),
            "instagram" => Some(Icon::Instagram),
            "italic" => Some(Icon::Italic),
            "iteration-ccw" => Some(Icon::IterationCcw),
            "iteration-cw" => Some(Icon::IterationCw),
            "japanese-yen" => Some(Icon::JapaneseYen),
            "joystick" => Some(Icon::Joystick),
            "kanban" => Some(Icon::Kanban),
            "key" => Some(Icon::Key),
            "key-round" => Some(Icon::KeyRound),
            "key-square" => Some(Icon::KeySquare),
            "keyboard" => Some(Icon::Keyboard),
            "keyboard-music" => Some(Icon::KeyboardMusic),
            "keyboard-off" => Some(Icon::KeyboardOff),
            "lamp" => Some(Icon::Lamp),
            "lamp-ceiling" => Some(Icon::LampCeiling),
            "lamp-desk" => Some(Icon::LampDesk),
            "lamp-floor" => Some(Icon::LampFloor),
            "lamp-wall-down" => Some(Icon::LampWallDown),
            "lamp-wall-up" => Some(Icon::LampWallUp),
            "land-plot" => Some(Icon::LandPlot),
            "landmark" => Some(Icon::Landmark),
            "languages" => Some(Icon::Languages),
            "laptop" => Some(Icon::Laptop),
            "laptop-minimal" => Some(Icon::LaptopMinimal),
            "laptop-minimal-check" => Some(Icon::LaptopMinimalCheck),
            "lasso" => Some(Icon::Lasso),
            "lasso-select" => Some(Icon::LassoSelect),
            "laugh" => Some(Icon::Laugh),
            "layers" => Some(Icon::Layers),
            "layers-2" => Some(Icon::Layers2),
            "layout-dashboard" => Some(Icon::LayoutDashboard),
            "layout-grid" => Some(Icon::LayoutGrid),
            "layout-list" => Some(Icon::LayoutList),
            "layout-panel-left" => Some(Icon::LayoutPanelLeft),
            "layout-panel-top" => Some(Icon::LayoutPanelTop),
            "layout-template" => Some(Icon::LayoutTemplate),
            "leaf" => Some(Icon::Leaf),
            "leafy-green" => Some(Icon::LeafyGreen),
            "lectern" => Some(Icon::Lectern),
            "letter-text" => Some(Icon::LetterText),
            "library" => Some(Icon::Library),
            "library-big" => Some(Icon::LibraryBig),
            "life-buoy" => Some(Icon::LifeBuoy),
            "ligature" => Some(Icon::Ligature),
            "lightbulb" => Some(Icon::Lightbulb),
            "lightbulb-off" => Some(Icon::LightbulbOff),
            "line-squiggle" => Some(Icon::LineSquiggle),
            "link" => Some(Icon::Link),
            "link-2" => Some(Icon::Link2),
            "link-2-off" => Some(Icon::Link2Off),
            "linkedin" => Some(Icon::Linkedin),
            "list" => Some(Icon::List),
            "list-check" => Some(Icon::ListCheck),
            "list-checks" => Some(Icon::ListChecks),
            "list-collapse" => Some(Icon::ListCollapse),
            "list-end" => Some(Icon::ListEnd),
            "list-filter" => Some(Icon::ListFilter),
            "list-filter-plus" => Some(Icon::ListFilterPlus),
            "list-minus" => Some(Icon::ListMinus),
            "list-music" => Some(Icon::ListMusic),
            "list-ordered" => Some(Icon::ListOrdered),
            "list-plus" => Some(Icon::ListPlus),
            "list-restart" => Some(Icon::ListRestart),
            "list-start" => Some(Icon::ListStart),
            "list-todo" => Some(Icon::ListTodo),
            "list-tree" => Some(Icon::ListTree),
            "list-video" => Some(Icon::ListVideo),
            "list-x" => Some(Icon::ListX),
            "loader" => Some(Icon::Loader),
            "loader-circle" => Some(Icon::LoaderCircle),
            "loader-pinwheel" => Some(Icon::LoaderPinwheel),
            "locate" => Some(Icon::Locate),
            "locate-fixed" => Some(Icon::LocateFixed),
            "locate-off" => Some(Icon::LocateOff),
            "location-edit" => Some(Icon::LocationEdit),
            "lock" => Some(Icon::Lock),
            "lock-keyhole" => Some(Icon::LockKeyhole),
            "lock-keyhole-open" => Some(Icon::LockKeyholeOpen),
            "lock-open" => Some(Icon::LockOpen),
            "log-in" => Some(Icon::LogIn),
            "log-out" => Some(Icon::LogOut),
            "logs" => Some(Icon::Logs),
            "lollipop" => Some(Icon::Lollipop),
            "luggage" => Some(Icon::Luggage),
            "magnet" => Some(Icon::Magnet),
            "mail" => Some(Icon::Mail),
            "mail-check" => Some(Icon::MailCheck),
            "mail-minus" => Some(Icon::MailMinus),
            "mail-open" => Some(Icon::MailOpen),
            "mail-plus" => Some(Icon::MailPlus),
            "mail-question-mark" => Some(Icon::MailQuestionMark),
            "mail-search" => Some(Icon::MailSearch),
            "mail-warning" => Some(Icon::MailWarning),
            "mail-x" => Some(Icon::MailX),
            "mailbox" => Some(Icon::Mailbox),
            "mails" => Some(Icon::Mails),
            "map" => Some(Icon::Map),
            "map-pin" => Some(Icon::MapPin),
            "map-pin-check" => Some(Icon::MapPinCheck),
            "map-pin-check-inside" => Some(Icon::MapPinCheckInside),
            "map-pin-house" => Some(Icon::MapPinHouse),
            "map-pin-minus" => Some(Icon::MapPinMinus),
            "map-pin-minus-inside" => Some(Icon::MapPinMinusInside),
            "map-pin-off" => Some(Icon::MapPinOff),
            "map-pin-plus" => Some(Icon::MapPinPlus),
            "map-pin-plus-inside" => Some(Icon::MapPinPlusInside),
            "map-pin-x" => Some(Icon::MapPinX),
            "map-pin-x-inside" => Some(Icon::MapPinXInside),
            "map-pinned" => Some(Icon::MapPinned),
            "map-plus" => Some(Icon::MapPlus),
            "mars" => Some(Icon::Mars),
            "mars-stroke" => Some(Icon::MarsStroke),
            "martini" => Some(Icon::Martini),
            "maximize" => Some(Icon::Maximize),
            "maximize-2" => Some(Icon::Maximize2),
            "medal" => Some(Icon::Medal),
            "megaphone" => Some(Icon::Megaphone),
            "megaphone-off" => Some(Icon::MegaphoneOff),
            "meh" => Some(Icon::Meh),
            "memory-stick" => Some(Icon::MemoryStick),
            "menu" => Some(Icon::Menu),
            "merge" => Some(Icon::Merge),
            "message-circle" => Some(Icon::MessageCircle),
            "message-circle-code" => Some(Icon::MessageCircleCode),
            "message-circle-dashed" => Some(Icon::MessageCircleDashed),
            "message-circle-heart" => Some(Icon::MessageCircleHeart),
            "message-circle-more" => Some(Icon::MessageCircleMore),
            "message-circle-off" => Some(Icon::MessageCircleOff),
            "message-circle-plus" => Some(Icon::MessageCirclePlus),
            "message-circle-question-mark" => Some(Icon::MessageCircleQuestionMark),
            "message-circle-reply" => Some(Icon::MessageCircleReply),
            "message-circle-warning" => Some(Icon::MessageCircleWarning),
            "message-circle-x" => Some(Icon::MessageCircleX),
            "message-square" => Some(Icon::MessageSquare),
            "message-square-code" => Some(Icon::MessageSquareCode),
            "message-square-dashed" => Some(Icon::MessageSquareDashed),
            "message-square-diff" => Some(Icon::MessageSquareDiff),
            "message-square-dot" => Some(Icon::MessageSquareDot),
            "message-square-heart" => Some(Icon::MessageSquareHeart),
            "message-square-lock" => Some(Icon::MessageSquareLock),
            "message-square-more" => Some(Icon::MessageSquareMore),
            "message-square-off" => Some(Icon::MessageSquareOff),
            "message-square-plus" => Some(Icon::MessageSquarePlus),
            "message-square-quote" => Some(Icon::MessageSquareQuote),
            "message-square-reply" => Some(Icon::MessageSquareReply),
            "message-square-share" => Some(Icon::MessageSquareShare),
            "message-square-text" => Some(Icon::MessageSquareText),
            "message-square-warning" => Some(Icon::MessageSquareWarning),
            "message-square-x" => Some(Icon::MessageSquareX),
            "messages-square" => Some(Icon::MessagesSquare),
            "mic" => Some(Icon::Mic),
            "mic-off" => Some(Icon::MicOff),
            "mic-vocal" => Some(Icon::MicVocal),
            "microchip" => Some(Icon::Microchip),
            "microscope" => Some(Icon::Microscope),
            "microwave" => Some(Icon::Microwave),
            "milestone" => Some(Icon::Milestone),
            "milk" => Some(Icon::Milk),
            "milk-off" => Some(Icon::MilkOff),
            "minimize" => Some(Icon::Minimize),
            "minimize-2" => Some(Icon::Minimize2),
            "minus" => Some(Icon::Minus),
            "monitor" => Some(Icon::Monitor),
            "monitor-check" => Some(Icon::MonitorCheck),
            "monitor-cog" => Some(Icon::MonitorCog),
            "monitor-dot" => Some(Icon::MonitorDot),
            "monitor-down" => Some(Icon::MonitorDown),
            "monitor-off" => Some(Icon::MonitorOff),
            "monitor-pause" => Some(Icon::MonitorPause),
            "monitor-play" => Some(Icon::MonitorPlay),
            "monitor-smartphone" => Some(Icon::MonitorSmartphone),
            "monitor-speaker" => Some(Icon::MonitorSpeaker),
            "monitor-stop" => Some(Icon::MonitorStop),
            "monitor-up" => Some(Icon::MonitorUp),
            "monitor-x" => Some(Icon::MonitorX),
            "moon" => Some(Icon::Moon),
            "moon-star" => Some(Icon::MoonStar),
            "mountain" => Some(Icon::Mountain),
            "mountain-snow" => Some(Icon::MountainSnow),
            "mouse" => Some(Icon::Mouse),
            "mouse-off" => Some(Icon::MouseOff),
            "mouse-pointer" => Some(Icon::MousePointer),
            "mouse-pointer-2" => Some(Icon::MousePointer2),
            "mouse-pointer-ban" => Some(Icon::MousePointerBan),
            "mouse-pointer-click" => Some(Icon::MousePointerClick),
            "move" => Some(Icon::Move),
            "move-3d" => Some(Icon::Move3d),
            "move-diagonal" => Some(Icon::MoveDiagonal),
            "move-diagonal-2" => Some(Icon::MoveDiagonal2),
            "move-down" => Some(Icon::MoveDown),
            "move-down-left" => Some(Icon::MoveDownLeft),
            "move-down-right" => Some(Icon::MoveDownRight),
            "move-horizontal" => Some(Icon::MoveHorizontal),
            "move-left" => Some(Icon::MoveLeft),
            "move-right" => Some(Icon::MoveRight),
            "move-up" => Some(Icon::MoveUp),
            "move-up-left" => Some(Icon::MoveUpLeft),
            "move-up-right" => Some(Icon::MoveUpRight),
            "move-vertical" => Some(Icon::MoveVertical),
            "music" => Some(Icon::Music),
            "music-2" => Some(Icon::Music2),
            "music-3" => Some(Icon::Music3),
            "music-4" => Some(Icon::Music4),
            "navigation" => Some(Icon::Navigation),
            "navigation-2" => Some(Icon::Navigation2),
            "navigation-2-off" => Some(Icon::Navigation2Off),
            "navigation-off" => Some(Icon::NavigationOff),
            "network" => Some(Icon::Network),
            "newspaper" => Some(Icon::Newspaper),
            "nfc" => Some(Icon::Nfc),
            "non-binary" => Some(Icon::NonBinary),
            "notebook" => Some(Icon::Notebook),
            "notebook-pen" => Some(Icon::NotebookPen),
            "notebook-tabs" => Some(Icon::NotebookTabs),
            "notebook-text" => Some(Icon::NotebookText),
            "notepad-text" => Some(Icon::NotepadText),
            "notepad-text-dashed" => Some(Icon::NotepadTextDashed),
            "nut" => Some(Icon::Nut),
            "nut-off" => Some(Icon::NutOff),
            "octagon" => Some(Icon::Octagon),
            "octagon-alert" => Some(Icon::OctagonAlert),
            "octagon-minus" => Some(Icon::OctagonMinus),
            "octagon-pause" => Some(Icon::OctagonPause),
            "octagon-x" => Some(Icon::OctagonX),
            "omega" => Some(Icon::Omega),
            "option" => Some(Icon::Option),
            "orbit" => Some(Icon::Orbit),
            "origami" => Some(Icon::Origami),
            "package" => Some(Icon::Package),
            "package-2" => Some(Icon::Package2),
            "package-check" => Some(Icon::PackageCheck),
            "package-minus" => Some(Icon::PackageMinus),
            "package-open" => Some(Icon::PackageOpen),
            "package-plus" => Some(Icon::PackagePlus),
            "package-search" => Some(Icon::PackageSearch),
            "package-x" => Some(Icon::PackageX),
            "paint-bucket" => Some(Icon::PaintBucket),
            "paint-roller" => Some(Icon::PaintRoller),
            "paintbrush" => Some(Icon::Paintbrush),
            "paintbrush-vertical" => Some(Icon::PaintbrushVertical),
            "palette" => Some(Icon::Palette),
            "panda" => Some(Icon::Panda),
            "panel-bottom" => Some(Icon::PanelBottom),
            "panel-bottom-close" => Some(Icon::PanelBottomClose),
            "panel-bottom-dashed" => Some(Icon::PanelBottomDashed),
            "panel-bottom-open" => Some(Icon::PanelBottomOpen),
            "panel-left" => Some(Icon::PanelLeft),
            "panel-left-close" => Some(Icon::PanelLeftClose),
            "panel-left-dashed" => Some(Icon::PanelLeftDashed),
            "panel-left-open" => Some(Icon::PanelLeftOpen),
            "panel-right" => Some(Icon::PanelRight),
            "panel-right-close" => Some(Icon::PanelRightClose),
            "panel-right-dashed" => Some(Icon::PanelRightDashed),
            "panel-right-open" => Some(Icon::PanelRightOpen),
            "panel-top" => Some(Icon::PanelTop),
            "panel-top-close" => Some(Icon::PanelTopClose),
            "panel-top-dashed" => Some(Icon::PanelTopDashed),
            "panel-top-open" => Some(Icon::PanelTopOpen),
            "panels-left-bottom" => Some(Icon::PanelsLeftBottom),
            "panels-right-bottom" => Some(Icon::PanelsRightBottom),
            "panels-top-left" => Some(Icon::PanelsTopLeft),
            "paperclip" => Some(Icon::Paperclip),
            "parentheses" => Some(Icon::Parentheses),
            "parking-meter" => Some(Icon::ParkingMeter),
            "party-popper" => Some(Icon::PartyPopper),
            "pause" => Some(Icon::Pause),
            "paw-print" => Some(Icon::PawPrint),
            "pc-case" => Some(Icon::PcCase),
            "pen" => Some(Icon::Pen),
            "pen-line" => Some(Icon::PenLine),
            "pen-off" => Some(Icon::PenOff),
            "pen-tool" => Some(Icon::PenTool),
            "pencil" => Some(Icon::Pencil),
            "pencil-line" => Some(Icon::PencilLine),
            "pencil-off" => Some(Icon::PencilOff),
            "pencil-ruler" => Some(Icon::PencilRuler),
            "pentagon" => Some(Icon::Pentagon),
            "percent" => Some(Icon::Percent),
            "person-standing" => Some(Icon::PersonStanding),
            "philippine-peso" => Some(Icon::PhilippinePeso),
            "phone" => Some(Icon::Phone),
            "phone-call" => Some(Icon::PhoneCall),
            "phone-forwarded" => Some(Icon::PhoneForwarded),
            "phone-incoming" => Some(Icon::PhoneIncoming),
            "phone-missed" => Some(Icon::PhoneMissed),
            "phone-off" => Some(Icon::PhoneOff),
            "phone-outgoing" => Some(Icon::PhoneOutgoing),
            "pi" => Some(Icon::Pi),
            "piano" => Some(Icon::Piano),
            "pickaxe" => Some(Icon::Pickaxe),
            "picture-in-picture" => Some(Icon::PictureInPicture),
            "picture-in-picture-2" => Some(Icon::PictureInPicture2),
            "piggy-bank" => Some(Icon::PiggyBank),
            "pilcrow" => Some(Icon::Pilcrow),
            "pilcrow-left" => Some(Icon::PilcrowLeft),
            "pilcrow-right" => Some(Icon::PilcrowRight),
            "pill" => Some(Icon::Pill),
            "pill-bottle" => Some(Icon::PillBottle),
            "pin" => Some(Icon::Pin),
            "pin-off" => Some(Icon::PinOff),
            "pipette" => Some(Icon::Pipette),
            "pizza" => Some(Icon::Pizza),
            "plane" => Some(Icon::Plane),
            "plane-landing" => Some(Icon::PlaneLanding),
            "plane-takeoff" => Some(Icon::PlaneTakeoff),
            "play" => Some(Icon::Play),
            "plug" => Some(Icon::Plug),
            "plug-2" => Some(Icon::Plug2),
            "plug-zap" => Some(Icon::PlugZap),
            "plus" => Some(Icon::Plus),
            "pocket" => Some(Icon::Pocket),
            "pocket-knife" => Some(Icon::PocketKnife),
            "podcast" => Some(Icon::Podcast),
            "pointer" => Some(Icon::Pointer),
            "pointer-off" => Some(Icon::PointerOff),
            "popcorn" => Some(Icon::Popcorn),
            "popsicle" => Some(Icon::Popsicle),
            "pound-sterling" => Some(Icon::PoundSterling),
            "power" => Some(Icon::Power),
            "power-off" => Some(Icon::PowerOff),
            "presentation" => Some(Icon::Presentation),
            "printer" => Some(Icon::Printer),
            "printer-check" => Some(Icon::PrinterCheck),
            "projector" => Some(Icon::Projector),
            "proportions" => Some(Icon::Proportions),
            "puzzle" => Some(Icon::Puzzle),
            "pyramid" => Some(Icon::Pyramid),
            "qr-code" => Some(Icon::QrCode),
            "quote" => Some(Icon::Quote),
            "rabbit" => Some(Icon::Rabbit),
            "radar" => Some(Icon::Radar),
            "radiation" => Some(Icon::Radiation),
            "radical" => Some(Icon::Radical),
            "radio" => Some(Icon::Radio),
            "radio-receiver" => Some(Icon::RadioReceiver),
            "radio-tower" => Some(Icon::RadioTower),
            "radius" => Some(Icon::Radius),
            "rail-symbol" => Some(Icon::RailSymbol),
            "rainbow" => Some(Icon::Rainbow),
            "rat" => Some(Icon::Rat),
            "ratio" => Some(Icon::Ratio),
            "receipt" => Some(Icon::Receipt),
            "receipt-cent" => Some(Icon::ReceiptCent),
            "receipt-euro" => Some(Icon::ReceiptEuro),
            "receipt-indian-rupee" => Some(Icon::ReceiptIndianRupee),
            "receipt-japanese-yen" => Some(Icon::ReceiptJapaneseYen),
            "receipt-pound-sterling" => Some(Icon::ReceiptPoundSterling),
            "receipt-russian-ruble" => Some(Icon::ReceiptRussianRuble),
            "receipt-swiss-franc" => Some(Icon::ReceiptSwissFranc),
            "receipt-text" => Some(Icon::ReceiptText),
            "rectangle-circle" => Some(Icon::RectangleCircle),
            "rectangle-ellipsis" => Some(Icon::RectangleEllipsis),
            "rectangle-goggles" => Some(Icon::RectangleGoggles),
            "rectangle-horizontal" => Some(Icon::RectangleHorizontal),
            "rectangle-vertical" => Some(Icon::RectangleVertical),
            "recycle" => Some(Icon::Recycle),
            "redo" => Some(Icon::Redo),
            "redo-2" => Some(Icon::Redo2),
            "redo-dot" => Some(Icon::RedoDot),
            "refresh-ccw" => Some(Icon::RefreshCcw),
            "refresh-ccw-dot" => Some(Icon::RefreshCcwDot),
            "refresh-cw" => Some(Icon::RefreshCw),
            "refresh-cw-off" => Some(Icon::RefreshCwOff),
            "refrigerator" => Some(Icon::Refrigerator),
            "regex" => Some(Icon::Regex),
            "remove-formatting" => Some(Icon::RemoveFormatting),
            "repeat" => Some(Icon::Repeat),
            "repeat-1" => Some(Icon::Repeat1),
            "repeat-2" => Some(Icon::Repeat2),
            "replace" => Some(Icon::Replace),
            "replace-all" => Some(Icon::ReplaceAll),
            "reply" => Some(Icon::Reply),
            "reply-all" => Some(Icon::ReplyAll),
            "rewind" => Some(Icon::Rewind),
            "ribbon" => Some(Icon::Ribbon),
            "rocket" => Some(Icon::Rocket),
            "rocking-chair" => Some(Icon::RockingChair),
            "roller-coaster" => Some(Icon::RollerCoaster),
            "rotate-3d" => Some(Icon::Rotate3d),
            "rotate-ccw" => Some(Icon::RotateCcw),
            "rotate-ccw-key" => Some(Icon::RotateCcwKey),
            "rotate-ccw-square" => Some(Icon::RotateCcwSquare),
            "rotate-cw" => Some(Icon::RotateCw),
            "rotate-cw-square" => Some(Icon::RotateCwSquare),
            "route" => Some(Icon::Route),
            "route-off" => Some(Icon::RouteOff),
            "router" => Some(Icon::Router),
            "rows-2" => Some(Icon::Rows2),
            "rows-3" => Some(Icon::Rows3),
            "rows-4" => Some(Icon::Rows4),
            "rss" => Some(Icon::Rss),
            "ruler" => Some(Icon::Ruler),
            "ruler-dimension-line" => Some(Icon::RulerDimensionLine),
            "russian-ruble" => Some(Icon::RussianRuble),
            "sailboat" => Some(Icon::Sailboat),
            "salad" => Some(Icon::Salad),
            "sandwich" => Some(Icon::Sandwich),
            "satellite" => Some(Icon::Satellite),
            "satellite-dish" => Some(Icon::SatelliteDish),
            "saudi-riyal" => Some(Icon::SaudiRiyal),
            "save" => Some(Icon::Save),
            "save-all" => Some(Icon::SaveAll),
            "save-off" => Some(Icon::SaveOff),
            "scale" => Some(Icon::Scale),
            "scale-3d" => Some(Icon::Scale3d),
            "scaling" => Some(Icon::Scaling),
            "scan" => Some(Icon::Scan),
            "scan-barcode" => Some(Icon::ScanBarcode),
            "scan-eye" => Some(Icon::ScanEye),
            "scan-face" => Some(Icon::ScanFace),
            "scan-heart" => Some(Icon::ScanHeart),
            "scan-line" => Some(Icon::ScanLine),
            "scan-qr-code" => Some(Icon::ScanQrCode),
            "scan-search" => Some(Icon::ScanSearch),
            "scan-text" => Some(Icon::ScanText),
            "school" => Some(Icon::School),
            "scissors" => Some(Icon::Scissors),
            "scissors-line-dashed" => Some(Icon::ScissorsLineDashed),
            "screen-share" => Some(Icon::ScreenShare),
            "screen-share-off" => Some(Icon::ScreenShareOff),
            "scroll" => Some(Icon::Scroll),
            "scroll-text" => Some(Icon::ScrollText),
            "search" => Some(Icon::Search),
            "search-check" => Some(Icon::SearchCheck),
            "search-code" => Some(Icon::SearchCode),
            "search-slash" => Some(Icon::SearchSlash),
            "search-x" => Some(Icon::SearchX),
            "section" => Some(Icon::Section),
            "send" => Some(Icon::Send),
            "send-horizontal" => Some(Icon::SendHorizontal),
            "send-to-back" => Some(Icon::SendToBack),
            "separator-horizontal" => Some(Icon::SeparatorHorizontal),
            "separator-vertical" => Some(Icon::SeparatorVertical),
            "server" => Some(Icon::Server),
            "server-cog" => Some(Icon::ServerCog),
            "server-crash" => Some(Icon::ServerCrash),
            "server-off" => Some(Icon::ServerOff),
            "settings" => Some(Icon::Settings),
            "settings-2" => Some(Icon::Settings2),
            "shapes" => Some(Icon::Shapes),
            "share" => Some(Icon::Share),
            "share-2" => Some(Icon::Share2),
            "sheet" => Some(Icon::Sheet),
            "shell" => Some(Icon::Shell),
            "shield" => Some(Icon::Shield),
            "shield-alert" => Some(Icon::ShieldAlert),
            "shield-ban" => Some(Icon::ShieldBan),
            "shield-check" => Some(Icon::ShieldCheck),
            "shield-ellipsis" => Some(Icon::ShieldEllipsis),
            "shield-half" => Some(Icon::ShieldHalf),
            "shield-minus" => Some(Icon::ShieldMinus),
            "shield-off" => Some(Icon::ShieldOff),
            "shield-plus" => Some(Icon::ShieldPlus),
            "shield-question-mark" => Some(Icon::ShieldQuestionMark),
            "shield-user" => Some(Icon::ShieldUser),
            "shield-x" => Some(Icon::ShieldX),
            "ship" => Some(Icon::Ship),
            "ship-wheel" => Some(Icon::ShipWheel),
            "shirt" => Some(Icon::Shirt),
            "shopping-bag" => Some(Icon::ShoppingBag),
            "shopping-basket" => Some(Icon::ShoppingBasket),
            "shopping-cart" => Some(Icon::ShoppingCart),
            "shovel" => Some(Icon::Shovel),
            "shower-head" => Some(Icon::ShowerHead),
            "shredder" => Some(Icon::Shredder),
            "shrimp" => Some(Icon::Shrimp),
            "shrink" => Some(Icon::Shrink),
            "shrub" => Some(Icon::Shrub),
            "shuffle" => Some(Icon::Shuffle),
            "sigma" => Some(Icon::Sigma),
            "signal" => Some(Icon::Signal),
            "signal-high" => Some(Icon::SignalHigh),
            "signal-low" => Some(Icon::SignalLow),
            "signal-medium" => Some(Icon::SignalMedium),
            "signal-zero" => Some(Icon::SignalZero),
            "signature" => Some(Icon::Signature),
            "signpost" => Some(Icon::Signpost),
            "signpost-big" => Some(Icon::SignpostBig),
            "siren" => Some(Icon::Siren),
            "skip-back" => Some(Icon::SkipBack),
            "skip-forward" => Some(Icon::SkipForward),
            "skull" => Some(Icon::Skull),
            "slack" => Some(Icon::Slack),
            "slash" => Some(Icon::Slash),
            "slice" => Some(Icon::Slice),
            "sliders-horizontal" => Some(Icon::SlidersHorizontal),
            "sliders-vertical" => Some(Icon::SlidersVertical),
            "smartphone" => Some(Icon::Smartphone),
            "smartphone-charging" => Some(Icon::SmartphoneCharging),
            "smartphone-nfc" => Some(Icon::SmartphoneNfc),
            "smile" => Some(Icon::Smile),
            "smile-plus" => Some(Icon::SmilePlus),
            "snail" => Some(Icon::Snail),
            "snowflake" => Some(Icon::Snowflake),
            "soap-dispenser-droplet" => Some(Icon::SoapDispenserDroplet),
            "sofa" => Some(Icon::Sofa),
            "soup" => Some(Icon::Soup),
            "space" => Some(Icon::Space),
            "spade" => Some(Icon::Spade),
            "sparkle" => Some(Icon::Sparkle),
            "sparkles" => Some(Icon::Sparkles),
            "speaker" => Some(Icon::Speaker),
            "speech" => Some(Icon::Speech),
            "spell-check" => Some(Icon::SpellCheck),
            "spell-check-2" => Some(Icon::SpellCheck2),
            "spline" => Some(Icon::Spline),
            "spline-pointer" => Some(Icon::SplinePointer),
            "split" => Some(Icon::Split),
            "spool" => Some(Icon::Spool),
            "spray-can" => Some(Icon::SprayCan),
            "sprout" => Some(Icon::Sprout),
            "square" => Some(Icon::Square),
            "square-activity" => Some(Icon::SquareActivity),
            "square-arrow-down" => Some(Icon::SquareArrowDown),
            "square-arrow-down-left" => Some(Icon::SquareArrowDownLeft),
            "square-arrow-down-right" => Some(Icon::SquareArrowDownRight),
            "square-arrow-left" => Some(Icon::SquareArrowLeft),
            "square-arrow-out-down-left" => Some(Icon::SquareArrowOutDownLeft),
            "square-arrow-out-down-right" => Some(Icon::SquareArrowOutDownRight),
            "square-arrow-out-up-left" => Some(Icon::SquareArrowOutUpLeft),
            "square-arrow-out-up-right" => Some(Icon::SquareArrowOutUpRight),
            "square-arrow-right" => Some(Icon::SquareArrowRight),
            "square-arrow-up" => Some(Icon::SquareArrowUp),
            "square-arrow-up-left" => Some(Icon::SquareArrowUpLeft),
            "square-arrow-up-right" => Some(Icon::SquareArrowUpRight),
            "square-asterisk" => Some(Icon::SquareAsterisk),
            "square-bottom-dashed-scissors" => Some(Icon::SquareBottomDashedScissors),
            "square-chart-gantt" => Some(Icon::SquareChartGantt),
            "square-check" => Some(Icon::SquareCheck),
            "square-check-big" => Some(Icon::SquareCheckBig),
            "square-chevron-down" => Some(Icon::SquareChevronDown),
            "square-chevron-left" => Some(Icon::SquareChevronLeft),
            "square-chevron-right" => Some(Icon::SquareChevronRight),
            "square-chevron-up" => Some(Icon::SquareChevronUp),
            "square-code" => Some(Icon::SquareCode),
            "square-dashed" => Some(Icon::SquareDashed),
            "square-dashed-bottom" => Some(Icon::SquareDashedBottom),
            "square-dashed-bottom-code" => Some(Icon::SquareDashedBottomCode),
            "square-dashed-kanban" => Some(Icon::SquareDashedKanban),
            "square-dashed-mouse-pointer" => Some(Icon::SquareDashedMousePointer),
            "square-dashed-top-solid" => Some(Icon::SquareDashedTopSolid),
            "square-divide" => Some(Icon::SquareDivide),
            "square-dot" => Some(Icon::SquareDot),
            "square-equal" => Some(Icon::SquareEqual),
            "square-function" => Some(Icon::SquareFunction),
            "square-kanban" => Some(Icon::SquareKanban),
            "square-library" => Some(Icon::SquareLibrary),
            "square-m" => Some(Icon::SquareM),
            "square-menu" => Some(Icon::SquareMenu),
            "square-minus" => Some(Icon::SquareMinus),
            "square-mouse-pointer" => Some(Icon::SquareMousePointer),
            "square-parking" => Some(Icon::SquareParking),
            "square-parking-off" => Some(Icon::SquareParkingOff),
            "square-pen" => Some(Icon::SquarePen),
            "square-percent" => Some(Icon::SquarePercent),
            "square-pi" => Some(Icon::SquarePi),
            "square-pilcrow" => Some(Icon::SquarePilcrow),
            "square-play" => Some(Icon::SquarePlay),
            "square-plus" => Some(Icon::SquarePlus),
            "square-power" => Some(Icon::SquarePower),
            "square-radical" => Some(Icon::SquareRadical),
            "square-round-corner" => Some(Icon::SquareRoundCorner),
            "square-scissors" => Some(Icon::SquareScissors),
            "square-sigma" => Some(Icon::SquareSigma),
            "square-slash" => Some(Icon::SquareSlash),
            "square-split-horizontal" => Some(Icon::SquareSplitHorizontal),
            "square-split-vertical" => Some(Icon::SquareSplitVertical),
            "square-square" => Some(Icon::SquareSquare),
            "square-stack" => Some(Icon::SquareStack),
            "square-terminal" => Some(Icon::SquareTerminal),
            "square-user" => Some(Icon::SquareUser),
            "square-user-round" => Some(Icon::SquareUserRound),
            "square-x" => Some(Icon::SquareX),
            "squares-exclude" => Some(Icon::SquaresExclude),
            "squares-intersect" => Some(Icon::SquaresIntersect),
            "squares-subtract" => Some(Icon::SquaresSubtract),
            "squares-unite" => Some(Icon::SquaresUnite),
            "squircle" => Some(Icon::Squircle),
            "squircle-dashed" => Some(Icon::SquircleDashed),
            "squirrel" => Some(Icon::Squirrel),
            "stamp" => Some(Icon::Stamp),
            "star" => Some(Icon::Star),
            "star-half" => Some(Icon::StarHalf),
            "star-off" => Some(Icon::StarOff),
            "step-back" => Some(Icon::StepBack),
            "step-forward" => Some(Icon::StepForward),
            "stethoscope" => Some(Icon::Stethoscope),
            "sticker" => Some(Icon::Sticker),
            "sticky-note" => Some(Icon::StickyNote),
            "store" => Some(Icon::Store),
            "stretch-horizontal" => Some(Icon::StretchHorizontal),
            "stretch-vertical" => Some(Icon::StretchVertical),
            "strikethrough" => Some(Icon::Strikethrough),
            "subscript" => Some(Icon::Subscript),
            "sun" => Some(Icon::Sun),
            "sun-dim" => Some(Icon::SunDim),
            "sun-medium" => Some(Icon::SunMedium),
            "sun-moon" => Some(Icon::SunMoon),
            "sun-snow" => Some(Icon::SunSnow),
            "sunrise" => Some(Icon::Sunrise),
            "sunset" => Some(Icon::Sunset),
            "superscript" => Some(Icon::Superscript),
            "swatch-book" => Some(Icon::SwatchBook),
            "swiss-franc" => Some(Icon::SwissFranc),
            "switch-camera" => Some(Icon::SwitchCamera),
            "sword" => Some(Icon::Sword),
            "swords" => Some(Icon::Swords),
            "syringe" => Some(Icon::Syringe),
            "table" => Some(Icon::Table),
            "table-2" => Some(Icon::Table2),
            "table-cells-merge" => Some(Icon::TableCellsMerge),
            "table-cells-split" => Some(Icon::TableCellsSplit),
            "table-columns-split" => Some(Icon::TableColumnsSplit),
            "table-of-contents" => Some(Icon::TableOfContents),
            "table-properties" => Some(Icon::TableProperties),
            "table-rows-split" => Some(Icon::TableRowsSplit),
            "tablet" => Some(Icon::Tablet),
            "tablet-smartphone" => Some(Icon::TabletSmartphone),
            "tablets" => Some(Icon::Tablets),
            "tag" => Some(Icon::Tag),
            "tags" => Some(Icon::Tags),
            "tally-1" => Some(Icon::Tally1),
            "tally-2" => Some(Icon::Tally2),
            "tally-3" => Some(Icon::Tally3),
            "tally-4" => Some(Icon::Tally4),
            "tally-5" => Some(Icon::Tally5),
            "tangent" => Some(Icon::Tangent),
            "target" => Some(Icon::Target),
            "telescope" => Some(Icon::Telescope),
            "tent" => Some(Icon::Tent),
            "tent-tree" => Some(Icon::TentTree),
            "terminal" => Some(Icon::Terminal),
            "test-tube" => Some(Icon::TestTube),
            "test-tube-diagonal" => Some(Icon::TestTubeDiagonal),
            "test-tubes" => Some(Icon::TestTubes),
            "text" => Some(Icon::Text),
            "text-cursor" => Some(Icon::TextCursor),
            "text-cursor-input" => Some(Icon::TextCursorInput),
            "text-quote" => Some(Icon::TextQuote),
            "text-search" => Some(Icon::TextSearch),
            "text-select" => Some(Icon::TextSelect),
            "theater" => Some(Icon::Theater),
            "thermometer" => Some(Icon::Thermometer),
            "thermometer-snowflake" => Some(Icon::ThermometerSnowflake),
            "thermometer-sun" => Some(Icon::ThermometerSun),
            "thumbs-down" => Some(Icon::ThumbsDown),
            "thumbs-up" => Some(Icon::ThumbsUp),
            "ticket" => Some(Icon::Ticket),
            "ticket-check" => Some(Icon::TicketCheck),
            "ticket-minus" => Some(Icon::TicketMinus),
            "ticket-percent" => Some(Icon::TicketPercent),
            "ticket-plus" => Some(Icon::TicketPlus),
            "ticket-slash" => Some(Icon::TicketSlash),
            "ticket-x" => Some(Icon::TicketX),
            "tickets" => Some(Icon::Tickets),
            "tickets-plane" => Some(Icon::TicketsPlane),
            "timer" => Some(Icon::Timer),
            "timer-off" => Some(Icon::TimerOff),
            "timer-reset" => Some(Icon::TimerReset),
            "toggle-left" => Some(Icon::ToggleLeft),
            "toggle-right" => Some(Icon::ToggleRight),
            "toilet" => Some(Icon::Toilet),
            "tool-case" => Some(Icon::ToolCase),
            "tornado" => Some(Icon::Tornado),
            "torus" => Some(Icon::Torus),
            "touchpad" => Some(Icon::Touchpad),
            "touchpad-off" => Some(Icon::TouchpadOff),
            "tower-control" => Some(Icon::TowerControl),
            "toy-brick" => Some(Icon::ToyBrick),
            "tractor" => Some(Icon::Tractor),
            "traffic-cone" => Some(Icon::TrafficCone),
            "train-front" => Some(Icon::TrainFront),
            "train-front-tunnel" => Some(Icon::TrainFrontTunnel),
            "train-track" => Some(Icon::TrainTrack),
            "tram-front" => Some(Icon::TramFront),
            "transgender" => Some(Icon::Transgender),
            "trash" => Some(Icon::Trash),
            "trash-2" => Some(Icon::Trash2),
            "tree-deciduous" => Some(Icon::TreeDeciduous),
            "tree-palm" => Some(Icon::TreePalm),
            "tree-pine" => Some(Icon::TreePine),
            "trees" => Some(Icon::Trees),
            "trello" => Some(Icon::Trello),
            "trending-down" => Some(Icon::TrendingDown),
            "trending-up" => Some(Icon::TrendingUp),
            "trending-up-down" => Some(Icon::TrendingUpDown),
            "triangle" => Some(Icon::Triangle),
            "triangle-alert" => Some(Icon::TriangleAlert),
            "triangle-dashed" => Some(Icon::TriangleDashed),
            "triangle-right" => Some(Icon::TriangleRight),
            "trophy" => Some(Icon::Trophy),
            "truck" => Some(Icon::Truck),
            "truck-electric" => Some(Icon::TruckElectric),
            "turtle" => Some(Icon::Turtle),
            "tv" => Some(Icon::Tv),
            "tv-minimal" => Some(Icon::TvMinimal),
            "tv-minimal-play" => Some(Icon::TvMinimalPlay),
            "twitch" => Some(Icon::Twitch),
            "twitter" => Some(Icon::Twitter),
            "type" => Some(Icon::Type),
            "type-outline" => Some(Icon::TypeOutline),
            "umbrella" => Some(Icon::Umbrella),
            "umbrella-off" => Some(Icon::UmbrellaOff),
            "underline" => Some(Icon::Underline),
            "undo" => Some(Icon::Undo),
            "undo-2" => Some(Icon::Undo2),
            "undo-dot" => Some(Icon::UndoDot),
            "unfold-horizontal" => Some(Icon::UnfoldHorizontal),
            "unfold-vertical" => Some(Icon::UnfoldVertical),
            "ungroup" => Some(Icon::Ungroup),
            "university" => Some(Icon::University),
            "unlink" => Some(Icon::Unlink),
            "unlink-2" => Some(Icon::Unlink2),
            "unplug" => Some(Icon::Unplug),
            "upload" => Some(Icon::Upload),
            "usb" => Some(Icon::Usb),
            "user" => Some(Icon::User),
            "user-check" => Some(Icon::UserCheck),
            "user-cog" => Some(Icon::UserCog),
            "user-lock" => Some(Icon::UserLock),
            "user-minus" => Some(Icon::UserMinus),
            "user-pen" => Some(Icon::UserPen),
            "user-plus" => Some(Icon::UserPlus),
            "user-round" => Some(Icon::UserRound),
            "user-round-check" => Some(Icon::UserRoundCheck),
            "user-round-cog" => Some(Icon::UserRoundCog),
            "user-round-minus" => Some(Icon::UserRoundMinus),
            "user-round-pen" => Some(Icon::UserRoundPen),
            "user-round-plus" => Some(Icon::UserRoundPlus),
            "user-round-search" => Some(Icon::UserRoundSearch),
            "user-round-x" => Some(Icon::UserRoundX),
            "user-search" => Some(Icon::UserSearch),
            "user-x" => Some(Icon::UserX),
            "users" => Some(Icon::Users),
            "users-round" => Some(Icon::UsersRound),
            "utensils" => Some(Icon::Utensils),
            "utensils-crossed" => Some(Icon::UtensilsCrossed),
            "utility-pole" => Some(Icon::UtilityPole),
            "variable" => Some(Icon::Variable),
            "vault" => Some(Icon::Vault),
            "vector-square" => Some(Icon::VectorSquare),
            "vegan" => Some(Icon::Vegan),
            "venetian-mask" => Some(Icon::VenetianMask),
            "venus" => Some(Icon::Venus),
            "venus-and-mars" => Some(Icon::VenusAndMars),
            "vibrate" => Some(Icon::Vibrate),
            "vibrate-off" => Some(Icon::VibrateOff),
            "video" => Some(Icon::Video),
            "video-off" => Some(Icon::VideoOff),
            "videotape" => Some(Icon::Videotape),
            "view" => Some(Icon::View),
            "voicemail" => Some(Icon::Voicemail),
            "volleyball" => Some(Icon::Volleyball),
            "volume" => Some(Icon::Volume),
            "volume-1" => Some(Icon::Volume1),
            "volume-2" => Some(Icon::Volume2),
            "volume-off" => Some(Icon::VolumeOff),
            "volume-x" => Some(Icon::VolumeX),
            "vote" => Some(Icon::Vote),
            "wallet" => Some(Icon::Wallet),
            "wallet-cards" => Some(Icon::WalletCards),
            "wallet-minimal" => Some(Icon::WalletMinimal),
            "wallpaper" => Some(Icon::Wallpaper),
            "wand" => Some(Icon::Wand),
            "wand-sparkles" => Some(Icon::WandSparkles),
            "warehouse" => Some(Icon::Warehouse),
            "washing-machine" => Some(Icon::WashingMachine),
            "watch" => Some(Icon::Watch),
            "waves" => Some(Icon::Waves),
            "waves-ladder" => Some(Icon::WavesLadder),
            "waypoints" => Some(Icon::Waypoints),
            "webcam" => Some(Icon::Webcam),
            "webhook" => Some(Icon::Webhook),
            "webhook-off" => Some(Icon::WebhookOff),
            "weight" => Some(Icon::Weight),
            "wheat" => Some(Icon::Wheat),
            "wheat-off" => Some(Icon::WheatOff),
            "whole-word" => Some(Icon::WholeWord),
            "wifi" => Some(Icon::Wifi),
            "wifi-cog" => Some(Icon::WifiCog),
            "wifi-high" => Some(Icon::WifiHigh),
            "wifi-low" => Some(Icon::WifiLow),
            "wifi-off" => Some(Icon::WifiOff),
            "wifi-pen" => Some(Icon::WifiPen),
            "wifi-zero" => Some(Icon::WifiZero),
            "wind" => Some(Icon::Wind),
            "wind-arrow-down" => Some(Icon::WindArrowDown),
            "wine" => Some(Icon::Wine),
            "wine-off" => Some(Icon::WineOff),
            "workflow" => Some(Icon::Workflow),
            "worm" => Some(Icon::Worm),
            "wrap-text" => Some(Icon::WrapText),
            "wrench" => Some(Icon::Wrench),
            "x" => Some(Icon::X),
            "youtube" => Some(Icon::Youtube),
            "zap" => Some(Icon::Zap),
            "zap-off" => Some(Icon::ZapOff),
            "zoom-in" => Some(Icon::ZoomIn),
            "zoom-out" => Some(Icon::ZoomOut),
            &_ => None,
        }
    }
}
impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::AArrowDown => "a-arrow-down",
            Self::AArrowUp => "a-arrow-up",
            Self::ALargeSmall => "a-large-small",
            Self::Accessibility => "accessibility",
            Self::Activity => "activity",
            Self::AirVent => "air-vent",
            Self::Airplay => "airplay",
            Self::AlarmClock => "alarm-clock",
            Self::AlarmClockCheck => "alarm-clock-check",
            Self::AlarmClockMinus => "alarm-clock-minus",
            Self::AlarmClockOff => "alarm-clock-off",
            Self::AlarmClockPlus => "alarm-clock-plus",
            Self::AlarmSmoke => "alarm-smoke",
            Self::Album => "album",
            Self::AlignCenter => "align-center",
            Self::AlignCenterHorizontal => "align-center-horizontal",
            Self::AlignCenterVertical => "align-center-vertical",
            Self::AlignEndHorizontal => "align-end-horizontal",
            Self::AlignEndVertical => "align-end-vertical",
            Self::AlignHorizontalDistributeCenter => "align-horizontal-distribute-center",
            Self::AlignHorizontalDistributeEnd => "align-horizontal-distribute-end",
            Self::AlignHorizontalDistributeStart => "align-horizontal-distribute-start",
            Self::AlignHorizontalJustifyCenter => "align-horizontal-justify-center",
            Self::AlignHorizontalJustifyEnd => "align-horizontal-justify-end",
            Self::AlignHorizontalJustifyStart => "align-horizontal-justify-start",
            Self::AlignHorizontalSpaceAround => "align-horizontal-space-around",
            Self::AlignHorizontalSpaceBetween => "align-horizontal-space-between",
            Self::AlignJustify => "align-justify",
            Self::AlignLeft => "align-left",
            Self::AlignRight => "align-right",
            Self::AlignStartHorizontal => "align-start-horizontal",
            Self::AlignStartVertical => "align-start-vertical",
            Self::AlignVerticalDistributeCenter => "align-vertical-distribute-center",
            Self::AlignVerticalDistributeEnd => "align-vertical-distribute-end",
            Self::AlignVerticalDistributeStart => "align-vertical-distribute-start",
            Self::AlignVerticalJustifyCenter => "align-vertical-justify-center",
            Self::AlignVerticalJustifyEnd => "align-vertical-justify-end",
            Self::AlignVerticalJustifyStart => "align-vertical-justify-start",
            Self::AlignVerticalSpaceAround => "align-vertical-space-around",
            Self::AlignVerticalSpaceBetween => "align-vertical-space-between",
            Self::Ambulance => "ambulance",
            Self::Ampersand => "ampersand",
            Self::Ampersands => "ampersands",
            Self::Amphora => "amphora",
            Self::Anchor => "anchor",
            Self::Angry => "angry",
            Self::Annoyed => "annoyed",
            Self::Antenna => "antenna",
            Self::Anvil => "anvil",
            Self::Aperture => "aperture",
            Self::AppWindow => "app-window",
            Self::AppWindowMac => "app-window-mac",
            Self::Apple => "apple",
            Self::Archive => "archive",
            Self::ArchiveRestore => "archive-restore",
            Self::ArchiveX => "archive-x",
            Self::Armchair => "armchair",
            Self::ArrowBigDown => "arrow-big-down",
            Self::ArrowBigDownDash => "arrow-big-down-dash",
            Self::ArrowBigLeft => "arrow-big-left",
            Self::ArrowBigLeftDash => "arrow-big-left-dash",
            Self::ArrowBigRight => "arrow-big-right",
            Self::ArrowBigRightDash => "arrow-big-right-dash",
            Self::ArrowBigUp => "arrow-big-up",
            Self::ArrowBigUpDash => "arrow-big-up-dash",
            Self::ArrowDown => "arrow-down",
            Self::ArrowDown01 => "arrow-down-0-1",
            Self::ArrowDown10 => "arrow-down-1-0",
            Self::ArrowDownAZ => "arrow-down-a-z",
            Self::ArrowDownFromLine => "arrow-down-from-line",
            Self::ArrowDownLeft => "arrow-down-left",
            Self::ArrowDownNarrowWide => "arrow-down-narrow-wide",
            Self::ArrowDownRight => "arrow-down-right",
            Self::ArrowDownToDot => "arrow-down-to-dot",
            Self::ArrowDownToLine => "arrow-down-to-line",
            Self::ArrowDownUp => "arrow-down-up",
            Self::ArrowDownWideNarrow => "arrow-down-wide-narrow",
            Self::ArrowDownZA => "arrow-down-z-a",
            Self::ArrowLeft => "arrow-left",
            Self::ArrowLeftFromLine => "arrow-left-from-line",
            Self::ArrowLeftRight => "arrow-left-right",
            Self::ArrowLeftToLine => "arrow-left-to-line",
            Self::ArrowRight => "arrow-right",
            Self::ArrowRightFromLine => "arrow-right-from-line",
            Self::ArrowRightLeft => "arrow-right-left",
            Self::ArrowRightToLine => "arrow-right-to-line",
            Self::ArrowUp => "arrow-up",
            Self::ArrowUp01 => "arrow-up-0-1",
            Self::ArrowUp10 => "arrow-up-1-0",
            Self::ArrowUpAZ => "arrow-up-a-z",
            Self::ArrowUpDown => "arrow-up-down",
            Self::ArrowUpFromDot => "arrow-up-from-dot",
            Self::ArrowUpFromLine => "arrow-up-from-line",
            Self::ArrowUpLeft => "arrow-up-left",
            Self::ArrowUpNarrowWide => "arrow-up-narrow-wide",
            Self::ArrowUpRight => "arrow-up-right",
            Self::ArrowUpToLine => "arrow-up-to-line",
            Self::ArrowUpWideNarrow => "arrow-up-wide-narrow",
            Self::ArrowUpZA => "arrow-up-z-a",
            Self::ArrowsUpFromLine => "arrows-up-from-line",
            Self::Asterisk => "asterisk",
            Self::AtSign => "at-sign",
            Self::Atom => "atom",
            Self::AudioLines => "audio-lines",
            Self::AudioWaveform => "audio-waveform",
            Self::Award => "award",
            Self::Axe => "axe",
            Self::Axis3d => "axis-3d",
            Self::Baby => "baby",
            Self::Backpack => "backpack",
            Self::Badge => "badge",
            Self::BadgeAlert => "badge-alert",
            Self::BadgeCent => "badge-cent",
            Self::BadgeCheck => "badge-check",
            Self::BadgeDollarSign => "badge-dollar-sign",
            Self::BadgeEuro => "badge-euro",
            Self::BadgeIndianRupee => "badge-indian-rupee",
            Self::BadgeInfo => "badge-info",
            Self::BadgeJapaneseYen => "badge-japanese-yen",
            Self::BadgeMinus => "badge-minus",
            Self::BadgePercent => "badge-percent",
            Self::BadgePlus => "badge-plus",
            Self::BadgePoundSterling => "badge-pound-sterling",
            Self::BadgeQuestionMark => "badge-question-mark",
            Self::BadgeRussianRuble => "badge-russian-ruble",
            Self::BadgeSwissFranc => "badge-swiss-franc",
            Self::BadgeX => "badge-x",
            Self::BaggageClaim => "baggage-claim",
            Self::Ban => "ban",
            Self::Banana => "banana",
            Self::Bandage => "bandage",
            Self::Banknote => "banknote",
            Self::BanknoteArrowDown => "banknote-arrow-down",
            Self::BanknoteArrowUp => "banknote-arrow-up",
            Self::BanknoteX => "banknote-x",
            Self::Barcode => "barcode",
            Self::Barrel => "barrel",
            Self::Baseline => "baseline",
            Self::Bath => "bath",
            Self::Battery => "battery",
            Self::BatteryCharging => "battery-charging",
            Self::BatteryFull => "battery-full",
            Self::BatteryLow => "battery-low",
            Self::BatteryMedium => "battery-medium",
            Self::BatteryPlus => "battery-plus",
            Self::BatteryWarning => "battery-warning",
            Self::Beaker => "beaker",
            Self::Bean => "bean",
            Self::BeanOff => "bean-off",
            Self::Bed => "bed",
            Self::BedDouble => "bed-double",
            Self::BedSingle => "bed-single",
            Self::Beef => "beef",
            Self::Beer => "beer",
            Self::BeerOff => "beer-off",
            Self::Bell => "bell",
            Self::BellDot => "bell-dot",
            Self::BellElectric => "bell-electric",
            Self::BellMinus => "bell-minus",
            Self::BellOff => "bell-off",
            Self::BellPlus => "bell-plus",
            Self::BellRing => "bell-ring",
            Self::BetweenHorizontalEnd => "between-horizontal-end",
            Self::BetweenHorizontalStart => "between-horizontal-start",
            Self::BetweenVerticalEnd => "between-vertical-end",
            Self::BetweenVerticalStart => "between-vertical-start",
            Self::BicepsFlexed => "biceps-flexed",
            Self::Bike => "bike",
            Self::Binary => "binary",
            Self::Binoculars => "binoculars",
            Self::Biohazard => "biohazard",
            Self::Bird => "bird",
            Self::Bitcoin => "bitcoin",
            Self::Blend => "blend",
            Self::Blinds => "blinds",
            Self::Blocks => "blocks",
            Self::Bluetooth => "bluetooth",
            Self::BluetoothConnected => "bluetooth-connected",
            Self::BluetoothOff => "bluetooth-off",
            Self::BluetoothSearching => "bluetooth-searching",
            Self::Bold => "bold",
            Self::Bolt => "bolt",
            Self::Bomb => "bomb",
            Self::Bone => "bone",
            Self::Book => "book",
            Self::BookA => "book-a",
            Self::BookAlert => "book-alert",
            Self::BookAudio => "book-audio",
            Self::BookCheck => "book-check",
            Self::BookCopy => "book-copy",
            Self::BookDashed => "book-dashed",
            Self::BookDown => "book-down",
            Self::BookHeadphones => "book-headphones",
            Self::BookHeart => "book-heart",
            Self::BookImage => "book-image",
            Self::BookKey => "book-key",
            Self::BookLock => "book-lock",
            Self::BookMarked => "book-marked",
            Self::BookMinus => "book-minus",
            Self::BookOpen => "book-open",
            Self::BookOpenCheck => "book-open-check",
            Self::BookOpenText => "book-open-text",
            Self::BookPlus => "book-plus",
            Self::BookText => "book-text",
            Self::BookType => "book-type",
            Self::BookUp => "book-up",
            Self::BookUp2 => "book-up-2",
            Self::BookUser => "book-user",
            Self::BookX => "book-x",
            Self::Bookmark => "bookmark",
            Self::BookmarkCheck => "bookmark-check",
            Self::BookmarkMinus => "bookmark-minus",
            Self::BookmarkPlus => "bookmark-plus",
            Self::BookmarkX => "bookmark-x",
            Self::BoomBox => "boom-box",
            Self::Bot => "bot",
            Self::BotMessageSquare => "bot-message-square",
            Self::BotOff => "bot-off",
            Self::BottleWine => "bottle-wine",
            Self::BowArrow => "bow-arrow",
            Self::Box => "box",
            Self::Boxes => "boxes",
            Self::Braces => "braces",
            Self::Brackets => "brackets",
            Self::Brain => "brain",
            Self::BrainCircuit => "brain-circuit",
            Self::BrainCog => "brain-cog",
            Self::BrickWall => "brick-wall",
            Self::BrickWallFire => "brick-wall-fire",
            Self::Briefcase => "briefcase",
            Self::BriefcaseBusiness => "briefcase-business",
            Self::BriefcaseConveyorBelt => "briefcase-conveyor-belt",
            Self::BriefcaseMedical => "briefcase-medical",
            Self::BringToFront => "bring-to-front",
            Self::Brush => "brush",
            Self::BrushCleaning => "brush-cleaning",
            Self::Bubbles => "bubbles",
            Self::Bug => "bug",
            Self::BugOff => "bug-off",
            Self::BugPlay => "bug-play",
            Self::Building => "building",
            Self::Building2 => "building-2",
            Self::Bus => "bus",
            Self::BusFront => "bus-front",
            Self::Cable => "cable",
            Self::CableCar => "cable-car",
            Self::Cake => "cake",
            Self::CakeSlice => "cake-slice",
            Self::Calculator => "calculator",
            Self::Calendar => "calendar",
            Self::Calendar1 => "calendar-1",
            Self::CalendarArrowDown => "calendar-arrow-down",
            Self::CalendarArrowUp => "calendar-arrow-up",
            Self::CalendarCheck => "calendar-check",
            Self::CalendarCheck2 => "calendar-check-2",
            Self::CalendarClock => "calendar-clock",
            Self::CalendarCog => "calendar-cog",
            Self::CalendarDays => "calendar-days",
            Self::CalendarFold => "calendar-fold",
            Self::CalendarHeart => "calendar-heart",
            Self::CalendarMinus => "calendar-minus",
            Self::CalendarMinus2 => "calendar-minus-2",
            Self::CalendarOff => "calendar-off",
            Self::CalendarPlus => "calendar-plus",
            Self::CalendarPlus2 => "calendar-plus-2",
            Self::CalendarRange => "calendar-range",
            Self::CalendarSearch => "calendar-search",
            Self::CalendarSync => "calendar-sync",
            Self::CalendarX => "calendar-x",
            Self::CalendarX2 => "calendar-x-2",
            Self::Camera => "camera",
            Self::CameraOff => "camera-off",
            Self::Candy => "candy",
            Self::CandyCane => "candy-cane",
            Self::CandyOff => "candy-off",
            Self::Cannabis => "cannabis",
            Self::Captions => "captions",
            Self::CaptionsOff => "captions-off",
            Self::Car => "car",
            Self::CarFront => "car-front",
            Self::CarTaxiFront => "car-taxi-front",
            Self::Caravan => "caravan",
            Self::CardSim => "card-sim",
            Self::Carrot => "carrot",
            Self::CaseLower => "case-lower",
            Self::CaseSensitive => "case-sensitive",
            Self::CaseUpper => "case-upper",
            Self::CassetteTape => "cassette-tape",
            Self::Cast => "cast",
            Self::Castle => "castle",
            Self::Cat => "cat",
            Self::Cctv => "cctv",
            Self::ChartArea => "chart-area",
            Self::ChartBar => "chart-bar",
            Self::ChartBarBig => "chart-bar-big",
            Self::ChartBarDecreasing => "chart-bar-decreasing",
            Self::ChartBarIncreasing => "chart-bar-increasing",
            Self::ChartBarStacked => "chart-bar-stacked",
            Self::ChartCandlestick => "chart-candlestick",
            Self::ChartColumn => "chart-column",
            Self::ChartColumnBig => "chart-column-big",
            Self::ChartColumnDecreasing => "chart-column-decreasing",
            Self::ChartColumnIncreasing => "chart-column-increasing",
            Self::ChartColumnStacked => "chart-column-stacked",
            Self::ChartGantt => "chart-gantt",
            Self::ChartLine => "chart-line",
            Self::ChartNetwork => "chart-network",
            Self::ChartNoAxesColumn => "chart-no-axes-column",
            Self::ChartNoAxesColumnDecreasing => "chart-no-axes-column-decreasing",
            Self::ChartNoAxesColumnIncreasing => "chart-no-axes-column-increasing",
            Self::ChartNoAxesCombined => "chart-no-axes-combined",
            Self::ChartNoAxesGantt => "chart-no-axes-gantt",
            Self::ChartPie => "chart-pie",
            Self::ChartScatter => "chart-scatter",
            Self::ChartSpline => "chart-spline",
            Self::Check => "check",
            Self::CheckCheck => "check-check",
            Self::CheckLine => "check-line",
            Self::ChefHat => "chef-hat",
            Self::Cherry => "cherry",
            Self::ChevronDown => "chevron-down",
            Self::ChevronFirst => "chevron-first",
            Self::ChevronLast => "chevron-last",
            Self::ChevronLeft => "chevron-left",
            Self::ChevronRight => "chevron-right",
            Self::ChevronUp => "chevron-up",
            Self::ChevronsDown => "chevrons-down",
            Self::ChevronsDownUp => "chevrons-down-up",
            Self::ChevronsLeft => "chevrons-left",
            Self::ChevronsLeftRight => "chevrons-left-right",
            Self::ChevronsLeftRightEllipsis => "chevrons-left-right-ellipsis",
            Self::ChevronsRight => "chevrons-right",
            Self::ChevronsRightLeft => "chevrons-right-left",
            Self::ChevronsUp => "chevrons-up",
            Self::ChevronsUpDown => "chevrons-up-down",
            Self::Chrome => "chrome",
            Self::Church => "church",
            Self::Cigarette => "cigarette",
            Self::CigaretteOff => "cigarette-off",
            Self::Circle => "circle",
            Self::CircleAlert => "circle-alert",
            Self::CircleArrowDown => "circle-arrow-down",
            Self::CircleArrowLeft => "circle-arrow-left",
            Self::CircleArrowOutDownLeft => "circle-arrow-out-down-left",
            Self::CircleArrowOutDownRight => "circle-arrow-out-down-right",
            Self::CircleArrowOutUpLeft => "circle-arrow-out-up-left",
            Self::CircleArrowOutUpRight => "circle-arrow-out-up-right",
            Self::CircleArrowRight => "circle-arrow-right",
            Self::CircleArrowUp => "circle-arrow-up",
            Self::CircleCheck => "circle-check",
            Self::CircleCheckBig => "circle-check-big",
            Self::CircleChevronDown => "circle-chevron-down",
            Self::CircleChevronLeft => "circle-chevron-left",
            Self::CircleChevronRight => "circle-chevron-right",
            Self::CircleChevronUp => "circle-chevron-up",
            Self::CircleDashed => "circle-dashed",
            Self::CircleDivide => "circle-divide",
            Self::CircleDollarSign => "circle-dollar-sign",
            Self::CircleDot => "circle-dot",
            Self::CircleDotDashed => "circle-dot-dashed",
            Self::CircleEllipsis => "circle-ellipsis",
            Self::CircleEqual => "circle-equal",
            Self::CircleFadingArrowUp => "circle-fading-arrow-up",
            Self::CircleFadingPlus => "circle-fading-plus",
            Self::CircleGauge => "circle-gauge",
            Self::CircleMinus => "circle-minus",
            Self::CircleOff => "circle-off",
            Self::CircleParking => "circle-parking",
            Self::CircleParkingOff => "circle-parking-off",
            Self::CirclePause => "circle-pause",
            Self::CirclePercent => "circle-percent",
            Self::CirclePlay => "circle-play",
            Self::CirclePlus => "circle-plus",
            Self::CirclePoundSterling => "circle-pound-sterling",
            Self::CirclePower => "circle-power",
            Self::CircleQuestionMark => "circle-question-mark",
            Self::CircleSlash => "circle-slash",
            Self::CircleSlash2 => "circle-slash-2",
            Self::CircleSmall => "circle-small",
            Self::CircleStop => "circle-stop",
            Self::CircleUser => "circle-user",
            Self::CircleUserRound => "circle-user-round",
            Self::CircleX => "circle-x",
            Self::CircuitBoard => "circuit-board",
            Self::Citrus => "citrus",
            Self::Clapperboard => "clapperboard",
            Self::Clipboard => "clipboard",
            Self::ClipboardCheck => "clipboard-check",
            Self::ClipboardCopy => "clipboard-copy",
            Self::ClipboardList => "clipboard-list",
            Self::ClipboardMinus => "clipboard-minus",
            Self::ClipboardPaste => "clipboard-paste",
            Self::ClipboardPen => "clipboard-pen",
            Self::ClipboardPenLine => "clipboard-pen-line",
            Self::ClipboardPlus => "clipboard-plus",
            Self::ClipboardType => "clipboard-type",
            Self::ClipboardX => "clipboard-x",
            Self::Clock => "clock",
            Self::Clock1 => "clock-1",
            Self::Clock10 => "clock-10",
            Self::Clock11 => "clock-11",
            Self::Clock12 => "clock-12",
            Self::Clock2 => "clock-2",
            Self::Clock3 => "clock-3",
            Self::Clock4 => "clock-4",
            Self::Clock5 => "clock-5",
            Self::Clock6 => "clock-6",
            Self::Clock7 => "clock-7",
            Self::Clock8 => "clock-8",
            Self::Clock9 => "clock-9",
            Self::ClockAlert => "clock-alert",
            Self::ClockArrowDown => "clock-arrow-down",
            Self::ClockArrowUp => "clock-arrow-up",
            Self::ClockFading => "clock-fading",
            Self::ClockPlus => "clock-plus",
            Self::Cloud => "cloud",
            Self::CloudAlert => "cloud-alert",
            Self::CloudCheck => "cloud-check",
            Self::CloudCog => "cloud-cog",
            Self::CloudDownload => "cloud-download",
            Self::CloudDrizzle => "cloud-drizzle",
            Self::CloudFog => "cloud-fog",
            Self::CloudHail => "cloud-hail",
            Self::CloudLightning => "cloud-lightning",
            Self::CloudMoon => "cloud-moon",
            Self::CloudMoonRain => "cloud-moon-rain",
            Self::CloudOff => "cloud-off",
            Self::CloudRain => "cloud-rain",
            Self::CloudRainWind => "cloud-rain-wind",
            Self::CloudSnow => "cloud-snow",
            Self::CloudSun => "cloud-sun",
            Self::CloudSunRain => "cloud-sun-rain",
            Self::CloudUpload => "cloud-upload",
            Self::Cloudy => "cloudy",
            Self::Clover => "clover",
            Self::Club => "club",
            Self::Code => "code",
            Self::CodeXml => "code-xml",
            Self::Codepen => "codepen",
            Self::Codesandbox => "codesandbox",
            Self::Coffee => "coffee",
            Self::Cog => "cog",
            Self::Coins => "coins",
            Self::Columns2 => "columns-2",
            Self::Columns3 => "columns-3",
            Self::Columns3Cog => "columns-3-cog",
            Self::Columns4 => "columns-4",
            Self::Combine => "combine",
            Self::Command => "command",
            Self::Compass => "compass",
            Self::Component => "component",
            Self::Computer => "computer",
            Self::ConciergeBell => "concierge-bell",
            Self::Cone => "cone",
            Self::Construction => "construction",
            Self::Contact => "contact",
            Self::ContactRound => "contact-round",
            Self::Container => "container",
            Self::Contrast => "contrast",
            Self::Cookie => "cookie",
            Self::CookingPot => "cooking-pot",
            Self::Copy => "copy",
            Self::CopyCheck => "copy-check",
            Self::CopyMinus => "copy-minus",
            Self::CopyPlus => "copy-plus",
            Self::CopySlash => "copy-slash",
            Self::CopyX => "copy-x",
            Self::Copyleft => "copyleft",
            Self::Copyright => "copyright",
            Self::CornerDownLeft => "corner-down-left",
            Self::CornerDownRight => "corner-down-right",
            Self::CornerLeftDown => "corner-left-down",
            Self::CornerLeftUp => "corner-left-up",
            Self::CornerRightDown => "corner-right-down",
            Self::CornerRightUp => "corner-right-up",
            Self::CornerUpLeft => "corner-up-left",
            Self::CornerUpRight => "corner-up-right",
            Self::Cpu => "cpu",
            Self::CreativeCommons => "creative-commons",
            Self::CreditCard => "credit-card",
            Self::Croissant => "croissant",
            Self::Crop => "crop",
            Self::Cross => "cross",
            Self::Crosshair => "crosshair",
            Self::Crown => "crown",
            Self::Cuboid => "cuboid",
            Self::CupSoda => "cup-soda",
            Self::Currency => "currency",
            Self::Cylinder => "cylinder",
            Self::Dam => "dam",
            Self::Database => "database",
            Self::DatabaseBackup => "database-backup",
            Self::DatabaseZap => "database-zap",
            Self::DecimalsArrowLeft => "decimals-arrow-left",
            Self::DecimalsArrowRight => "decimals-arrow-right",
            Self::Delete => "delete",
            Self::Dessert => "dessert",
            Self::Diameter => "diameter",
            Self::Diamond => "diamond",
            Self::DiamondMinus => "diamond-minus",
            Self::DiamondPercent => "diamond-percent",
            Self::DiamondPlus => "diamond-plus",
            Self::Dice1 => "dice-1",
            Self::Dice2 => "dice-2",
            Self::Dice3 => "dice-3",
            Self::Dice4 => "dice-4",
            Self::Dice5 => "dice-5",
            Self::Dice6 => "dice-6",
            Self::Dices => "dices",
            Self::Diff => "diff",
            Self::Disc => "disc",
            Self::Disc2 => "disc-2",
            Self::Disc3 => "disc-3",
            Self::DiscAlbum => "disc-album",
            Self::Divide => "divide",
            Self::Dna => "dna",
            Self::DnaOff => "dna-off",
            Self::Dock => "dock",
            Self::Dog => "dog",
            Self::DollarSign => "dollar-sign",
            Self::Donut => "donut",
            Self::DoorClosed => "door-closed",
            Self::DoorClosedLocked => "door-closed-locked",
            Self::DoorOpen => "door-open",
            Self::Dot => "dot",
            Self::Download => "download",
            Self::DraftingCompass => "drafting-compass",
            Self::Drama => "drama",
            Self::Dribbble => "dribbble",
            Self::Drill => "drill",
            Self::Drone => "drone",
            Self::Droplet => "droplet",
            Self::DropletOff => "droplet-off",
            Self::Droplets => "droplets",
            Self::Drum => "drum",
            Self::Drumstick => "drumstick",
            Self::Dumbbell => "dumbbell",
            Self::Ear => "ear",
            Self::EarOff => "ear-off",
            Self::Earth => "earth",
            Self::EarthLock => "earth-lock",
            Self::Eclipse => "eclipse",
            Self::Egg => "egg",
            Self::EggFried => "egg-fried",
            Self::EggOff => "egg-off",
            Self::Ellipsis => "ellipsis",
            Self::EllipsisVertical => "ellipsis-vertical",
            Self::Equal => "equal",
            Self::EqualApproximately => "equal-approximately",
            Self::EqualNot => "equal-not",
            Self::Eraser => "eraser",
            Self::EthernetPort => "ethernet-port",
            Self::Euro => "euro",
            Self::Expand => "expand",
            Self::ExternalLink => "external-link",
            Self::Eye => "eye",
            Self::EyeClosed => "eye-closed",
            Self::EyeOff => "eye-off",
            Self::Facebook => "facebook",
            Self::Factory => "factory",
            Self::Fan => "fan",
            Self::FastForward => "fast-forward",
            Self::Feather => "feather",
            Self::Fence => "fence",
            Self::FerrisWheel => "ferris-wheel",
            Self::Figma => "figma",
            Self::File => "file",
            Self::FileArchive => "file-archive",
            Self::FileAudio => "file-audio",
            Self::FileAudio2 => "file-audio-2",
            Self::FileAxis3d => "file-axis-3d",
            Self::FileBadge => "file-badge",
            Self::FileBadge2 => "file-badge-2",
            Self::FileBox => "file-box",
            Self::FileChartColumn => "file-chart-column",
            Self::FileChartColumnIncreasing => "file-chart-column-increasing",
            Self::FileChartLine => "file-chart-line",
            Self::FileChartPie => "file-chart-pie",
            Self::FileCheck => "file-check",
            Self::FileCheck2 => "file-check-2",
            Self::FileClock => "file-clock",
            Self::FileCode => "file-code",
            Self::FileCode2 => "file-code-2",
            Self::FileCog => "file-cog",
            Self::FileDiff => "file-diff",
            Self::FileDigit => "file-digit",
            Self::FileDown => "file-down",
            Self::FileHeart => "file-heart",
            Self::FileImage => "file-image",
            Self::FileInput => "file-input",
            Self::FileJson => "file-json",
            Self::FileJson2 => "file-json-2",
            Self::FileKey => "file-key",
            Self::FileKey2 => "file-key-2",
            Self::FileLock => "file-lock",
            Self::FileLock2 => "file-lock-2",
            Self::FileMinus => "file-minus",
            Self::FileMinus2 => "file-minus-2",
            Self::FileMusic => "file-music",
            Self::FileOutput => "file-output",
            Self::FilePen => "file-pen",
            Self::FilePenLine => "file-pen-line",
            Self::FilePlus => "file-plus",
            Self::FilePlus2 => "file-plus-2",
            Self::FileQuestionMark => "file-question-mark",
            Self::FileScan => "file-scan",
            Self::FileSearch => "file-search",
            Self::FileSearch2 => "file-search-2",
            Self::FileSliders => "file-sliders",
            Self::FileSpreadsheet => "file-spreadsheet",
            Self::FileStack => "file-stack",
            Self::FileSymlink => "file-symlink",
            Self::FileTerminal => "file-terminal",
            Self::FileText => "file-text",
            Self::FileType => "file-type",
            Self::FileType2 => "file-type-2",
            Self::FileUp => "file-up",
            Self::FileUser => "file-user",
            Self::FileVideo => "file-video",
            Self::FileVideo2 => "file-video-2",
            Self::FileVolume => "file-volume",
            Self::FileVolume2 => "file-volume-2",
            Self::FileWarning => "file-warning",
            Self::FileX => "file-x",
            Self::FileX2 => "file-x-2",
            Self::Files => "files",
            Self::Film => "film",
            Self::Fingerprint => "fingerprint",
            Self::FireExtinguisher => "fire-extinguisher",
            Self::Fish => "fish",
            Self::FishOff => "fish-off",
            Self::FishSymbol => "fish-symbol",
            Self::Flag => "flag",
            Self::FlagOff => "flag-off",
            Self::FlagTriangleLeft => "flag-triangle-left",
            Self::FlagTriangleRight => "flag-triangle-right",
            Self::Flame => "flame",
            Self::FlameKindling => "flame-kindling",
            Self::Flashlight => "flashlight",
            Self::FlashlightOff => "flashlight-off",
            Self::FlaskConical => "flask-conical",
            Self::FlaskConicalOff => "flask-conical-off",
            Self::FlaskRound => "flask-round",
            Self::FlipHorizontal => "flip-horizontal",
            Self::FlipHorizontal2 => "flip-horizontal-2",
            Self::FlipVertical => "flip-vertical",
            Self::FlipVertical2 => "flip-vertical-2",
            Self::Flower => "flower",
            Self::Flower2 => "flower-2",
            Self::Focus => "focus",
            Self::FoldHorizontal => "fold-horizontal",
            Self::FoldVertical => "fold-vertical",
            Self::Folder => "folder",
            Self::FolderArchive => "folder-archive",
            Self::FolderCheck => "folder-check",
            Self::FolderClock => "folder-clock",
            Self::FolderClosed => "folder-closed",
            Self::FolderCode => "folder-code",
            Self::FolderCog => "folder-cog",
            Self::FolderDot => "folder-dot",
            Self::FolderDown => "folder-down",
            Self::FolderGit => "folder-git",
            Self::FolderGit2 => "folder-git-2",
            Self::FolderHeart => "folder-heart",
            Self::FolderInput => "folder-input",
            Self::FolderKanban => "folder-kanban",
            Self::FolderKey => "folder-key",
            Self::FolderLock => "folder-lock",
            Self::FolderMinus => "folder-minus",
            Self::FolderOpen => "folder-open",
            Self::FolderOpenDot => "folder-open-dot",
            Self::FolderOutput => "folder-output",
            Self::FolderPen => "folder-pen",
            Self::FolderPlus => "folder-plus",
            Self::FolderRoot => "folder-root",
            Self::FolderSearch => "folder-search",
            Self::FolderSearch2 => "folder-search-2",
            Self::FolderSymlink => "folder-symlink",
            Self::FolderSync => "folder-sync",
            Self::FolderTree => "folder-tree",
            Self::FolderUp => "folder-up",
            Self::FolderX => "folder-x",
            Self::Folders => "folders",
            Self::Footprints => "footprints",
            Self::Forklift => "forklift",
            Self::Forward => "forward",
            Self::Frame => "frame",
            Self::Framer => "framer",
            Self::Frown => "frown",
            Self::Fuel => "fuel",
            Self::Fullscreen => "fullscreen",
            Self::Funnel => "funnel",
            Self::FunnelPlus => "funnel-plus",
            Self::FunnelX => "funnel-x",
            Self::GalleryHorizontal => "gallery-horizontal",
            Self::GalleryHorizontalEnd => "gallery-horizontal-end",
            Self::GalleryThumbnails => "gallery-thumbnails",
            Self::GalleryVertical => "gallery-vertical",
            Self::GalleryVerticalEnd => "gallery-vertical-end",
            Self::Gamepad => "gamepad",
            Self::Gamepad2 => "gamepad-2",
            Self::Gauge => "gauge",
            Self::Gavel => "gavel",
            Self::Gem => "gem",
            Self::GeorgianLari => "georgian-lari",
            Self::Ghost => "ghost",
            Self::Gift => "gift",
            Self::GitBranch => "git-branch",
            Self::GitBranchPlus => "git-branch-plus",
            Self::GitCommitHorizontal => "git-commit-horizontal",
            Self::GitCommitVertical => "git-commit-vertical",
            Self::GitCompare => "git-compare",
            Self::GitCompareArrows => "git-compare-arrows",
            Self::GitFork => "git-fork",
            Self::GitGraph => "git-graph",
            Self::GitMerge => "git-merge",
            Self::GitPullRequest => "git-pull-request",
            Self::GitPullRequestArrow => "git-pull-request-arrow",
            Self::GitPullRequestClosed => "git-pull-request-closed",
            Self::GitPullRequestCreate => "git-pull-request-create",
            Self::GitPullRequestCreateArrow => "git-pull-request-create-arrow",
            Self::GitPullRequestDraft => "git-pull-request-draft",
            Self::Github => "github",
            Self::Gitlab => "gitlab",
            Self::GlassWater => "glass-water",
            Self::Glasses => "glasses",
            Self::Globe => "globe",
            Self::GlobeLock => "globe-lock",
            Self::Goal => "goal",
            Self::Gpu => "gpu",
            Self::Grab => "grab",
            Self::GraduationCap => "graduation-cap",
            Self::Grape => "grape",
            Self::Grid2x2 => "grid-2x2",
            Self::Grid2x2Check => "grid-2x2-check",
            Self::Grid2x2Plus => "grid-2x2-plus",
            Self::Grid2x2X => "grid-2x2-x",
            Self::Grid3x2 => "grid-3x2",
            Self::Grid3x3 => "grid-3x3",
            Self::Grip => "grip",
            Self::GripHorizontal => "grip-horizontal",
            Self::GripVertical => "grip-vertical",
            Self::Group => "group",
            Self::Guitar => "guitar",
            Self::Ham => "ham",
            Self::Hamburger => "hamburger",
            Self::Hammer => "hammer",
            Self::Hand => "hand",
            Self::HandCoins => "hand-coins",
            Self::HandHeart => "hand-heart",
            Self::HandHelping => "hand-helping",
            Self::HandMetal => "hand-metal",
            Self::HandPlatter => "hand-platter",
            Self::Handshake => "handshake",
            Self::HardDrive => "hard-drive",
            Self::HardDriveDownload => "hard-drive-download",
            Self::HardDriveUpload => "hard-drive-upload",
            Self::HardHat => "hard-hat",
            Self::Hash => "hash",
            Self::Haze => "haze",
            Self::HdmiPort => "hdmi-port",
            Self::Heading => "heading",
            Self::Heading1 => "heading-1",
            Self::Heading2 => "heading-2",
            Self::Heading3 => "heading-3",
            Self::Heading4 => "heading-4",
            Self::Heading5 => "heading-5",
            Self::Heading6 => "heading-6",
            Self::HeadphoneOff => "headphone-off",
            Self::Headphones => "headphones",
            Self::Headset => "headset",
            Self::Heart => "heart",
            Self::HeartCrack => "heart-crack",
            Self::HeartHandshake => "heart-handshake",
            Self::HeartMinus => "heart-minus",
            Self::HeartOff => "heart-off",
            Self::HeartPlus => "heart-plus",
            Self::HeartPulse => "heart-pulse",
            Self::Heater => "heater",
            Self::Hexagon => "hexagon",
            Self::Highlighter => "highlighter",
            Self::History => "history",
            Self::Hop => "hop",
            Self::HopOff => "hop-off",
            Self::Hospital => "hospital",
            Self::Hotel => "hotel",
            Self::Hourglass => "hourglass",
            Self::House => "house",
            Self::HousePlug => "house-plug",
            Self::HousePlus => "house-plus",
            Self::HouseWifi => "house-wifi",
            Self::IceCreamBowl => "ice-cream-bowl",
            Self::IceCreamCone => "ice-cream-cone",
            Self::IdCard => "id-card",
            Self::IdCardLanyard => "id-card-lanyard",
            Self::Image => "image",
            Self::ImageDown => "image-down",
            Self::ImageMinus => "image-minus",
            Self::ImageOff => "image-off",
            Self::ImagePlay => "image-play",
            Self::ImagePlus => "image-plus",
            Self::ImageUp => "image-up",
            Self::ImageUpscale => "image-upscale",
            Self::Images => "images",
            Self::Import => "import",
            Self::Inbox => "inbox",
            Self::IndentDecrease => "indent-decrease",
            Self::IndentIncrease => "indent-increase",
            Self::IndianRupee => "indian-rupee",
            Self::Infinity => "infinity",
            Self::Info => "info",
            Self::InspectionPanel => "inspection-panel",
            Self::Instagram => "instagram",
            Self::Italic => "italic",
            Self::IterationCcw => "iteration-ccw",
            Self::IterationCw => "iteration-cw",
            Self::JapaneseYen => "japanese-yen",
            Self::Joystick => "joystick",
            Self::Kanban => "kanban",
            Self::Key => "key",
            Self::KeyRound => "key-round",
            Self::KeySquare => "key-square",
            Self::Keyboard => "keyboard",
            Self::KeyboardMusic => "keyboard-music",
            Self::KeyboardOff => "keyboard-off",
            Self::Lamp => "lamp",
            Self::LampCeiling => "lamp-ceiling",
            Self::LampDesk => "lamp-desk",
            Self::LampFloor => "lamp-floor",
            Self::LampWallDown => "lamp-wall-down",
            Self::LampWallUp => "lamp-wall-up",
            Self::LandPlot => "land-plot",
            Self::Landmark => "landmark",
            Self::Languages => "languages",
            Self::Laptop => "laptop",
            Self::LaptopMinimal => "laptop-minimal",
            Self::LaptopMinimalCheck => "laptop-minimal-check",
            Self::Lasso => "lasso",
            Self::LassoSelect => "lasso-select",
            Self::Laugh => "laugh",
            Self::Layers => "layers",
            Self::Layers2 => "layers-2",
            Self::LayoutDashboard => "layout-dashboard",
            Self::LayoutGrid => "layout-grid",
            Self::LayoutList => "layout-list",
            Self::LayoutPanelLeft => "layout-panel-left",
            Self::LayoutPanelTop => "layout-panel-top",
            Self::LayoutTemplate => "layout-template",
            Self::Leaf => "leaf",
            Self::LeafyGreen => "leafy-green",
            Self::Lectern => "lectern",
            Self::LetterText => "letter-text",
            Self::Library => "library",
            Self::LibraryBig => "library-big",
            Self::LifeBuoy => "life-buoy",
            Self::Ligature => "ligature",
            Self::Lightbulb => "lightbulb",
            Self::LightbulbOff => "lightbulb-off",
            Self::LineSquiggle => "line-squiggle",
            Self::Link => "link",
            Self::Link2 => "link-2",
            Self::Link2Off => "link-2-off",
            Self::Linkedin => "linkedin",
            Self::List => "list",
            Self::ListCheck => "list-check",
            Self::ListChecks => "list-checks",
            Self::ListCollapse => "list-collapse",
            Self::ListEnd => "list-end",
            Self::ListFilter => "list-filter",
            Self::ListFilterPlus => "list-filter-plus",
            Self::ListMinus => "list-minus",
            Self::ListMusic => "list-music",
            Self::ListOrdered => "list-ordered",
            Self::ListPlus => "list-plus",
            Self::ListRestart => "list-restart",
            Self::ListStart => "list-start",
            Self::ListTodo => "list-todo",
            Self::ListTree => "list-tree",
            Self::ListVideo => "list-video",
            Self::ListX => "list-x",
            Self::Loader => "loader",
            Self::LoaderCircle => "loader-circle",
            Self::LoaderPinwheel => "loader-pinwheel",
            Self::Locate => "locate",
            Self::LocateFixed => "locate-fixed",
            Self::LocateOff => "locate-off",
            Self::LocationEdit => "location-edit",
            Self::Lock => "lock",
            Self::LockKeyhole => "lock-keyhole",
            Self::LockKeyholeOpen => "lock-keyhole-open",
            Self::LockOpen => "lock-open",
            Self::LogIn => "log-in",
            Self::LogOut => "log-out",
            Self::Logs => "logs",
            Self::Lollipop => "lollipop",
            Self::Luggage => "luggage",
            Self::Magnet => "magnet",
            Self::Mail => "mail",
            Self::MailCheck => "mail-check",
            Self::MailMinus => "mail-minus",
            Self::MailOpen => "mail-open",
            Self::MailPlus => "mail-plus",
            Self::MailQuestionMark => "mail-question-mark",
            Self::MailSearch => "mail-search",
            Self::MailWarning => "mail-warning",
            Self::MailX => "mail-x",
            Self::Mailbox => "mailbox",
            Self::Mails => "mails",
            Self::Map => "map",
            Self::MapPin => "map-pin",
            Self::MapPinCheck => "map-pin-check",
            Self::MapPinCheckInside => "map-pin-check-inside",
            Self::MapPinHouse => "map-pin-house",
            Self::MapPinMinus => "map-pin-minus",
            Self::MapPinMinusInside => "map-pin-minus-inside",
            Self::MapPinOff => "map-pin-off",
            Self::MapPinPlus => "map-pin-plus",
            Self::MapPinPlusInside => "map-pin-plus-inside",
            Self::MapPinX => "map-pin-x",
            Self::MapPinXInside => "map-pin-x-inside",
            Self::MapPinned => "map-pinned",
            Self::MapPlus => "map-plus",
            Self::Mars => "mars",
            Self::MarsStroke => "mars-stroke",
            Self::Martini => "martini",
            Self::Maximize => "maximize",
            Self::Maximize2 => "maximize-2",
            Self::Medal => "medal",
            Self::Megaphone => "megaphone",
            Self::MegaphoneOff => "megaphone-off",
            Self::Meh => "meh",
            Self::MemoryStick => "memory-stick",
            Self::Menu => "menu",
            Self::Merge => "merge",
            Self::MessageCircle => "message-circle",
            Self::MessageCircleCode => "message-circle-code",
            Self::MessageCircleDashed => "message-circle-dashed",
            Self::MessageCircleHeart => "message-circle-heart",
            Self::MessageCircleMore => "message-circle-more",
            Self::MessageCircleOff => "message-circle-off",
            Self::MessageCirclePlus => "message-circle-plus",
            Self::MessageCircleQuestionMark => "message-circle-question-mark",
            Self::MessageCircleReply => "message-circle-reply",
            Self::MessageCircleWarning => "message-circle-warning",
            Self::MessageCircleX => "message-circle-x",
            Self::MessageSquare => "message-square",
            Self::MessageSquareCode => "message-square-code",
            Self::MessageSquareDashed => "message-square-dashed",
            Self::MessageSquareDiff => "message-square-diff",
            Self::MessageSquareDot => "message-square-dot",
            Self::MessageSquareHeart => "message-square-heart",
            Self::MessageSquareLock => "message-square-lock",
            Self::MessageSquareMore => "message-square-more",
            Self::MessageSquareOff => "message-square-off",
            Self::MessageSquarePlus => "message-square-plus",
            Self::MessageSquareQuote => "message-square-quote",
            Self::MessageSquareReply => "message-square-reply",
            Self::MessageSquareShare => "message-square-share",
            Self::MessageSquareText => "message-square-text",
            Self::MessageSquareWarning => "message-square-warning",
            Self::MessageSquareX => "message-square-x",
            Self::MessagesSquare => "messages-square",
            Self::Mic => "mic",
            Self::MicOff => "mic-off",
            Self::MicVocal => "mic-vocal",
            Self::Microchip => "microchip",
            Self::Microscope => "microscope",
            Self::Microwave => "microwave",
            Self::Milestone => "milestone",
            Self::Milk => "milk",
            Self::MilkOff => "milk-off",
            Self::Minimize => "minimize",
            Self::Minimize2 => "minimize-2",
            Self::Minus => "minus",
            Self::Monitor => "monitor",
            Self::MonitorCheck => "monitor-check",
            Self::MonitorCog => "monitor-cog",
            Self::MonitorDot => "monitor-dot",
            Self::MonitorDown => "monitor-down",
            Self::MonitorOff => "monitor-off",
            Self::MonitorPause => "monitor-pause",
            Self::MonitorPlay => "monitor-play",
            Self::MonitorSmartphone => "monitor-smartphone",
            Self::MonitorSpeaker => "monitor-speaker",
            Self::MonitorStop => "monitor-stop",
            Self::MonitorUp => "monitor-up",
            Self::MonitorX => "monitor-x",
            Self::Moon => "moon",
            Self::MoonStar => "moon-star",
            Self::Mountain => "mountain",
            Self::MountainSnow => "mountain-snow",
            Self::Mouse => "mouse",
            Self::MouseOff => "mouse-off",
            Self::MousePointer => "mouse-pointer",
            Self::MousePointer2 => "mouse-pointer-2",
            Self::MousePointerBan => "mouse-pointer-ban",
            Self::MousePointerClick => "mouse-pointer-click",
            Self::Move => "move",
            Self::Move3d => "move-3d",
            Self::MoveDiagonal => "move-diagonal",
            Self::MoveDiagonal2 => "move-diagonal-2",
            Self::MoveDown => "move-down",
            Self::MoveDownLeft => "move-down-left",
            Self::MoveDownRight => "move-down-right",
            Self::MoveHorizontal => "move-horizontal",
            Self::MoveLeft => "move-left",
            Self::MoveRight => "move-right",
            Self::MoveUp => "move-up",
            Self::MoveUpLeft => "move-up-left",
            Self::MoveUpRight => "move-up-right",
            Self::MoveVertical => "move-vertical",
            Self::Music => "music",
            Self::Music2 => "music-2",
            Self::Music3 => "music-3",
            Self::Music4 => "music-4",
            Self::Navigation => "navigation",
            Self::Navigation2 => "navigation-2",
            Self::Navigation2Off => "navigation-2-off",
            Self::NavigationOff => "navigation-off",
            Self::Network => "network",
            Self::Newspaper => "newspaper",
            Self::Nfc => "nfc",
            Self::NonBinary => "non-binary",
            Self::Notebook => "notebook",
            Self::NotebookPen => "notebook-pen",
            Self::NotebookTabs => "notebook-tabs",
            Self::NotebookText => "notebook-text",
            Self::NotepadText => "notepad-text",
            Self::NotepadTextDashed => "notepad-text-dashed",
            Self::Nut => "nut",
            Self::NutOff => "nut-off",
            Self::Octagon => "octagon",
            Self::OctagonAlert => "octagon-alert",
            Self::OctagonMinus => "octagon-minus",
            Self::OctagonPause => "octagon-pause",
            Self::OctagonX => "octagon-x",
            Self::Omega => "omega",
            Self::Option => "option",
            Self::Orbit => "orbit",
            Self::Origami => "origami",
            Self::Package => "package",
            Self::Package2 => "package-2",
            Self::PackageCheck => "package-check",
            Self::PackageMinus => "package-minus",
            Self::PackageOpen => "package-open",
            Self::PackagePlus => "package-plus",
            Self::PackageSearch => "package-search",
            Self::PackageX => "package-x",
            Self::PaintBucket => "paint-bucket",
            Self::PaintRoller => "paint-roller",
            Self::Paintbrush => "paintbrush",
            Self::PaintbrushVertical => "paintbrush-vertical",
            Self::Palette => "palette",
            Self::Panda => "panda",
            Self::PanelBottom => "panel-bottom",
            Self::PanelBottomClose => "panel-bottom-close",
            Self::PanelBottomDashed => "panel-bottom-dashed",
            Self::PanelBottomOpen => "panel-bottom-open",
            Self::PanelLeft => "panel-left",
            Self::PanelLeftClose => "panel-left-close",
            Self::PanelLeftDashed => "panel-left-dashed",
            Self::PanelLeftOpen => "panel-left-open",
            Self::PanelRight => "panel-right",
            Self::PanelRightClose => "panel-right-close",
            Self::PanelRightDashed => "panel-right-dashed",
            Self::PanelRightOpen => "panel-right-open",
            Self::PanelTop => "panel-top",
            Self::PanelTopClose => "panel-top-close",
            Self::PanelTopDashed => "panel-top-dashed",
            Self::PanelTopOpen => "panel-top-open",
            Self::PanelsLeftBottom => "panels-left-bottom",
            Self::PanelsRightBottom => "panels-right-bottom",
            Self::PanelsTopLeft => "panels-top-left",
            Self::Paperclip => "paperclip",
            Self::Parentheses => "parentheses",
            Self::ParkingMeter => "parking-meter",
            Self::PartyPopper => "party-popper",
            Self::Pause => "pause",
            Self::PawPrint => "paw-print",
            Self::PcCase => "pc-case",
            Self::Pen => "pen",
            Self::PenLine => "pen-line",
            Self::PenOff => "pen-off",
            Self::PenTool => "pen-tool",
            Self::Pencil => "pencil",
            Self::PencilLine => "pencil-line",
            Self::PencilOff => "pencil-off",
            Self::PencilRuler => "pencil-ruler",
            Self::Pentagon => "pentagon",
            Self::Percent => "percent",
            Self::PersonStanding => "person-standing",
            Self::PhilippinePeso => "philippine-peso",
            Self::Phone => "phone",
            Self::PhoneCall => "phone-call",
            Self::PhoneForwarded => "phone-forwarded",
            Self::PhoneIncoming => "phone-incoming",
            Self::PhoneMissed => "phone-missed",
            Self::PhoneOff => "phone-off",
            Self::PhoneOutgoing => "phone-outgoing",
            Self::Pi => "pi",
            Self::Piano => "piano",
            Self::Pickaxe => "pickaxe",
            Self::PictureInPicture => "picture-in-picture",
            Self::PictureInPicture2 => "picture-in-picture-2",
            Self::PiggyBank => "piggy-bank",
            Self::Pilcrow => "pilcrow",
            Self::PilcrowLeft => "pilcrow-left",
            Self::PilcrowRight => "pilcrow-right",
            Self::Pill => "pill",
            Self::PillBottle => "pill-bottle",
            Self::Pin => "pin",
            Self::PinOff => "pin-off",
            Self::Pipette => "pipette",
            Self::Pizza => "pizza",
            Self::Plane => "plane",
            Self::PlaneLanding => "plane-landing",
            Self::PlaneTakeoff => "plane-takeoff",
            Self::Play => "play",
            Self::Plug => "plug",
            Self::Plug2 => "plug-2",
            Self::PlugZap => "plug-zap",
            Self::Plus => "plus",
            Self::Pocket => "pocket",
            Self::PocketKnife => "pocket-knife",
            Self::Podcast => "podcast",
            Self::Pointer => "pointer",
            Self::PointerOff => "pointer-off",
            Self::Popcorn => "popcorn",
            Self::Popsicle => "popsicle",
            Self::PoundSterling => "pound-sterling",
            Self::Power => "power",
            Self::PowerOff => "power-off",
            Self::Presentation => "presentation",
            Self::Printer => "printer",
            Self::PrinterCheck => "printer-check",
            Self::Projector => "projector",
            Self::Proportions => "proportions",
            Self::Puzzle => "puzzle",
            Self::Pyramid => "pyramid",
            Self::QrCode => "qr-code",
            Self::Quote => "quote",
            Self::Rabbit => "rabbit",
            Self::Radar => "radar",
            Self::Radiation => "radiation",
            Self::Radical => "radical",
            Self::Radio => "radio",
            Self::RadioReceiver => "radio-receiver",
            Self::RadioTower => "radio-tower",
            Self::Radius => "radius",
            Self::RailSymbol => "rail-symbol",
            Self::Rainbow => "rainbow",
            Self::Rat => "rat",
            Self::Ratio => "ratio",
            Self::Receipt => "receipt",
            Self::ReceiptCent => "receipt-cent",
            Self::ReceiptEuro => "receipt-euro",
            Self::ReceiptIndianRupee => "receipt-indian-rupee",
            Self::ReceiptJapaneseYen => "receipt-japanese-yen",
            Self::ReceiptPoundSterling => "receipt-pound-sterling",
            Self::ReceiptRussianRuble => "receipt-russian-ruble",
            Self::ReceiptSwissFranc => "receipt-swiss-franc",
            Self::ReceiptText => "receipt-text",
            Self::RectangleCircle => "rectangle-circle",
            Self::RectangleEllipsis => "rectangle-ellipsis",
            Self::RectangleGoggles => "rectangle-goggles",
            Self::RectangleHorizontal => "rectangle-horizontal",
            Self::RectangleVertical => "rectangle-vertical",
            Self::Recycle => "recycle",
            Self::Redo => "redo",
            Self::Redo2 => "redo-2",
            Self::RedoDot => "redo-dot",
            Self::RefreshCcw => "refresh-ccw",
            Self::RefreshCcwDot => "refresh-ccw-dot",
            Self::RefreshCw => "refresh-cw",
            Self::RefreshCwOff => "refresh-cw-off",
            Self::Refrigerator => "refrigerator",
            Self::Regex => "regex",
            Self::RemoveFormatting => "remove-formatting",
            Self::Repeat => "repeat",
            Self::Repeat1 => "repeat-1",
            Self::Repeat2 => "repeat-2",
            Self::Replace => "replace",
            Self::ReplaceAll => "replace-all",
            Self::Reply => "reply",
            Self::ReplyAll => "reply-all",
            Self::Rewind => "rewind",
            Self::Ribbon => "ribbon",
            Self::Rocket => "rocket",
            Self::RockingChair => "rocking-chair",
            Self::RollerCoaster => "roller-coaster",
            Self::Rotate3d => "rotate-3d",
            Self::RotateCcw => "rotate-ccw",
            Self::RotateCcwKey => "rotate-ccw-key",
            Self::RotateCcwSquare => "rotate-ccw-square",
            Self::RotateCw => "rotate-cw",
            Self::RotateCwSquare => "rotate-cw-square",
            Self::Route => "route",
            Self::RouteOff => "route-off",
            Self::Router => "router",
            Self::Rows2 => "rows-2",
            Self::Rows3 => "rows-3",
            Self::Rows4 => "rows-4",
            Self::Rss => "rss",
            Self::Ruler => "ruler",
            Self::RulerDimensionLine => "ruler-dimension-line",
            Self::RussianRuble => "russian-ruble",
            Self::Sailboat => "sailboat",
            Self::Salad => "salad",
            Self::Sandwich => "sandwich",
            Self::Satellite => "satellite",
            Self::SatelliteDish => "satellite-dish",
            Self::SaudiRiyal => "saudi-riyal",
            Self::Save => "save",
            Self::SaveAll => "save-all",
            Self::SaveOff => "save-off",
            Self::Scale => "scale",
            Self::Scale3d => "scale-3d",
            Self::Scaling => "scaling",
            Self::Scan => "scan",
            Self::ScanBarcode => "scan-barcode",
            Self::ScanEye => "scan-eye",
            Self::ScanFace => "scan-face",
            Self::ScanHeart => "scan-heart",
            Self::ScanLine => "scan-line",
            Self::ScanQrCode => "scan-qr-code",
            Self::ScanSearch => "scan-search",
            Self::ScanText => "scan-text",
            Self::School => "school",
            Self::Scissors => "scissors",
            Self::ScissorsLineDashed => "scissors-line-dashed",
            Self::ScreenShare => "screen-share",
            Self::ScreenShareOff => "screen-share-off",
            Self::Scroll => "scroll",
            Self::ScrollText => "scroll-text",
            Self::Search => "search",
            Self::SearchCheck => "search-check",
            Self::SearchCode => "search-code",
            Self::SearchSlash => "search-slash",
            Self::SearchX => "search-x",
            Self::Section => "section",
            Self::Send => "send",
            Self::SendHorizontal => "send-horizontal",
            Self::SendToBack => "send-to-back",
            Self::SeparatorHorizontal => "separator-horizontal",
            Self::SeparatorVertical => "separator-vertical",
            Self::Server => "server",
            Self::ServerCog => "server-cog",
            Self::ServerCrash => "server-crash",
            Self::ServerOff => "server-off",
            Self::Settings => "settings",
            Self::Settings2 => "settings-2",
            Self::Shapes => "shapes",
            Self::Share => "share",
            Self::Share2 => "share-2",
            Self::Sheet => "sheet",
            Self::Shell => "shell",
            Self::Shield => "shield",
            Self::ShieldAlert => "shield-alert",
            Self::ShieldBan => "shield-ban",
            Self::ShieldCheck => "shield-check",
            Self::ShieldEllipsis => "shield-ellipsis",
            Self::ShieldHalf => "shield-half",
            Self::ShieldMinus => "shield-minus",
            Self::ShieldOff => "shield-off",
            Self::ShieldPlus => "shield-plus",
            Self::ShieldQuestionMark => "shield-question-mark",
            Self::ShieldUser => "shield-user",
            Self::ShieldX => "shield-x",
            Self::Ship => "ship",
            Self::ShipWheel => "ship-wheel",
            Self::Shirt => "shirt",
            Self::ShoppingBag => "shopping-bag",
            Self::ShoppingBasket => "shopping-basket",
            Self::ShoppingCart => "shopping-cart",
            Self::Shovel => "shovel",
            Self::ShowerHead => "shower-head",
            Self::Shredder => "shredder",
            Self::Shrimp => "shrimp",
            Self::Shrink => "shrink",
            Self::Shrub => "shrub",
            Self::Shuffle => "shuffle",
            Self::Sigma => "sigma",
            Self::Signal => "signal",
            Self::SignalHigh => "signal-high",
            Self::SignalLow => "signal-low",
            Self::SignalMedium => "signal-medium",
            Self::SignalZero => "signal-zero",
            Self::Signature => "signature",
            Self::Signpost => "signpost",
            Self::SignpostBig => "signpost-big",
            Self::Siren => "siren",
            Self::SkipBack => "skip-back",
            Self::SkipForward => "skip-forward",
            Self::Skull => "skull",
            Self::Slack => "slack",
            Self::Slash => "slash",
            Self::Slice => "slice",
            Self::SlidersHorizontal => "sliders-horizontal",
            Self::SlidersVertical => "sliders-vertical",
            Self::Smartphone => "smartphone",
            Self::SmartphoneCharging => "smartphone-charging",
            Self::SmartphoneNfc => "smartphone-nfc",
            Self::Smile => "smile",
            Self::SmilePlus => "smile-plus",
            Self::Snail => "snail",
            Self::Snowflake => "snowflake",
            Self::SoapDispenserDroplet => "soap-dispenser-droplet",
            Self::Sofa => "sofa",
            Self::Soup => "soup",
            Self::Space => "space",
            Self::Spade => "spade",
            Self::Sparkle => "sparkle",
            Self::Sparkles => "sparkles",
            Self::Speaker => "speaker",
            Self::Speech => "speech",
            Self::SpellCheck => "spell-check",
            Self::SpellCheck2 => "spell-check-2",
            Self::Spline => "spline",
            Self::SplinePointer => "spline-pointer",
            Self::Split => "split",
            Self::Spool => "spool",
            Self::SprayCan => "spray-can",
            Self::Sprout => "sprout",
            Self::Square => "square",
            Self::SquareActivity => "square-activity",
            Self::SquareArrowDown => "square-arrow-down",
            Self::SquareArrowDownLeft => "square-arrow-down-left",
            Self::SquareArrowDownRight => "square-arrow-down-right",
            Self::SquareArrowLeft => "square-arrow-left",
            Self::SquareArrowOutDownLeft => "square-arrow-out-down-left",
            Self::SquareArrowOutDownRight => "square-arrow-out-down-right",
            Self::SquareArrowOutUpLeft => "square-arrow-out-up-left",
            Self::SquareArrowOutUpRight => "square-arrow-out-up-right",
            Self::SquareArrowRight => "square-arrow-right",
            Self::SquareArrowUp => "square-arrow-up",
            Self::SquareArrowUpLeft => "square-arrow-up-left",
            Self::SquareArrowUpRight => "square-arrow-up-right",
            Self::SquareAsterisk => "square-asterisk",
            Self::SquareBottomDashedScissors => "square-bottom-dashed-scissors",
            Self::SquareChartGantt => "square-chart-gantt",
            Self::SquareCheck => "square-check",
            Self::SquareCheckBig => "square-check-big",
            Self::SquareChevronDown => "square-chevron-down",
            Self::SquareChevronLeft => "square-chevron-left",
            Self::SquareChevronRight => "square-chevron-right",
            Self::SquareChevronUp => "square-chevron-up",
            Self::SquareCode => "square-code",
            Self::SquareDashed => "square-dashed",
            Self::SquareDashedBottom => "square-dashed-bottom",
            Self::SquareDashedBottomCode => "square-dashed-bottom-code",
            Self::SquareDashedKanban => "square-dashed-kanban",
            Self::SquareDashedMousePointer => "square-dashed-mouse-pointer",
            Self::SquareDashedTopSolid => "square-dashed-top-solid",
            Self::SquareDivide => "square-divide",
            Self::SquareDot => "square-dot",
            Self::SquareEqual => "square-equal",
            Self::SquareFunction => "square-function",
            Self::SquareKanban => "square-kanban",
            Self::SquareLibrary => "square-library",
            Self::SquareM => "square-m",
            Self::SquareMenu => "square-menu",
            Self::SquareMinus => "square-minus",
            Self::SquareMousePointer => "square-mouse-pointer",
            Self::SquareParking => "square-parking",
            Self::SquareParkingOff => "square-parking-off",
            Self::SquarePen => "square-pen",
            Self::SquarePercent => "square-percent",
            Self::SquarePi => "square-pi",
            Self::SquarePilcrow => "square-pilcrow",
            Self::SquarePlay => "square-play",
            Self::SquarePlus => "square-plus",
            Self::SquarePower => "square-power",
            Self::SquareRadical => "square-radical",
            Self::SquareRoundCorner => "square-round-corner",
            Self::SquareScissors => "square-scissors",
            Self::SquareSigma => "square-sigma",
            Self::SquareSlash => "square-slash",
            Self::SquareSplitHorizontal => "square-split-horizontal",
            Self::SquareSplitVertical => "square-split-vertical",
            Self::SquareSquare => "square-square",
            Self::SquareStack => "square-stack",
            Self::SquareTerminal => "square-terminal",
            Self::SquareUser => "square-user",
            Self::SquareUserRound => "square-user-round",
            Self::SquareX => "square-x",
            Self::SquaresExclude => "squares-exclude",
            Self::SquaresIntersect => "squares-intersect",
            Self::SquaresSubtract => "squares-subtract",
            Self::SquaresUnite => "squares-unite",
            Self::Squircle => "squircle",
            Self::SquircleDashed => "squircle-dashed",
            Self::Squirrel => "squirrel",
            Self::Stamp => "stamp",
            Self::Star => "star",
            Self::StarHalf => "star-half",
            Self::StarOff => "star-off",
            Self::StepBack => "step-back",
            Self::StepForward => "step-forward",
            Self::Stethoscope => "stethoscope",
            Self::Sticker => "sticker",
            Self::StickyNote => "sticky-note",
            Self::Store => "store",
            Self::StretchHorizontal => "stretch-horizontal",
            Self::StretchVertical => "stretch-vertical",
            Self::Strikethrough => "strikethrough",
            Self::Subscript => "subscript",
            Self::Sun => "sun",
            Self::SunDim => "sun-dim",
            Self::SunMedium => "sun-medium",
            Self::SunMoon => "sun-moon",
            Self::SunSnow => "sun-snow",
            Self::Sunrise => "sunrise",
            Self::Sunset => "sunset",
            Self::Superscript => "superscript",
            Self::SwatchBook => "swatch-book",
            Self::SwissFranc => "swiss-franc",
            Self::SwitchCamera => "switch-camera",
            Self::Sword => "sword",
            Self::Swords => "swords",
            Self::Syringe => "syringe",
            Self::Table => "table",
            Self::Table2 => "table-2",
            Self::TableCellsMerge => "table-cells-merge",
            Self::TableCellsSplit => "table-cells-split",
            Self::TableColumnsSplit => "table-columns-split",
            Self::TableOfContents => "table-of-contents",
            Self::TableProperties => "table-properties",
            Self::TableRowsSplit => "table-rows-split",
            Self::Tablet => "tablet",
            Self::TabletSmartphone => "tablet-smartphone",
            Self::Tablets => "tablets",
            Self::Tag => "tag",
            Self::Tags => "tags",
            Self::Tally1 => "tally-1",
            Self::Tally2 => "tally-2",
            Self::Tally3 => "tally-3",
            Self::Tally4 => "tally-4",
            Self::Tally5 => "tally-5",
            Self::Tangent => "tangent",
            Self::Target => "target",
            Self::Telescope => "telescope",
            Self::Tent => "tent",
            Self::TentTree => "tent-tree",
            Self::Terminal => "terminal",
            Self::TestTube => "test-tube",
            Self::TestTubeDiagonal => "test-tube-diagonal",
            Self::TestTubes => "test-tubes",
            Self::Text => "text",
            Self::TextCursor => "text-cursor",
            Self::TextCursorInput => "text-cursor-input",
            Self::TextQuote => "text-quote",
            Self::TextSearch => "text-search",
            Self::TextSelect => "text-select",
            Self::Theater => "theater",
            Self::Thermometer => "thermometer",
            Self::ThermometerSnowflake => "thermometer-snowflake",
            Self::ThermometerSun => "thermometer-sun",
            Self::ThumbsDown => "thumbs-down",
            Self::ThumbsUp => "thumbs-up",
            Self::Ticket => "ticket",
            Self::TicketCheck => "ticket-check",
            Self::TicketMinus => "ticket-minus",
            Self::TicketPercent => "ticket-percent",
            Self::TicketPlus => "ticket-plus",
            Self::TicketSlash => "ticket-slash",
            Self::TicketX => "ticket-x",
            Self::Tickets => "tickets",
            Self::TicketsPlane => "tickets-plane",
            Self::Timer => "timer",
            Self::TimerOff => "timer-off",
            Self::TimerReset => "timer-reset",
            Self::ToggleLeft => "toggle-left",
            Self::ToggleRight => "toggle-right",
            Self::Toilet => "toilet",
            Self::ToolCase => "tool-case",
            Self::Tornado => "tornado",
            Self::Torus => "torus",
            Self::Touchpad => "touchpad",
            Self::TouchpadOff => "touchpad-off",
            Self::TowerControl => "tower-control",
            Self::ToyBrick => "toy-brick",
            Self::Tractor => "tractor",
            Self::TrafficCone => "traffic-cone",
            Self::TrainFront => "train-front",
            Self::TrainFrontTunnel => "train-front-tunnel",
            Self::TrainTrack => "train-track",
            Self::TramFront => "tram-front",
            Self::Transgender => "transgender",
            Self::Trash => "trash",
            Self::Trash2 => "trash-2",
            Self::TreeDeciduous => "tree-deciduous",
            Self::TreePalm => "tree-palm",
            Self::TreePine => "tree-pine",
            Self::Trees => "trees",
            Self::Trello => "trello",
            Self::TrendingDown => "trending-down",
            Self::TrendingUp => "trending-up",
            Self::TrendingUpDown => "trending-up-down",
            Self::Triangle => "triangle",
            Self::TriangleAlert => "triangle-alert",
            Self::TriangleDashed => "triangle-dashed",
            Self::TriangleRight => "triangle-right",
            Self::Trophy => "trophy",
            Self::Truck => "truck",
            Self::TruckElectric => "truck-electric",
            Self::Turtle => "turtle",
            Self::Tv => "tv",
            Self::TvMinimal => "tv-minimal",
            Self::TvMinimalPlay => "tv-minimal-play",
            Self::Twitch => "twitch",
            Self::Twitter => "twitter",
            Self::Type => "type",
            Self::TypeOutline => "type-outline",
            Self::Umbrella => "umbrella",
            Self::UmbrellaOff => "umbrella-off",
            Self::Underline => "underline",
            Self::Undo => "undo",
            Self::Undo2 => "undo-2",
            Self::UndoDot => "undo-dot",
            Self::UnfoldHorizontal => "unfold-horizontal",
            Self::UnfoldVertical => "unfold-vertical",
            Self::Ungroup => "ungroup",
            Self::University => "university",
            Self::Unlink => "unlink",
            Self::Unlink2 => "unlink-2",
            Self::Unplug => "unplug",
            Self::Upload => "upload",
            Self::Usb => "usb",
            Self::User => "user",
            Self::UserCheck => "user-check",
            Self::UserCog => "user-cog",
            Self::UserLock => "user-lock",
            Self::UserMinus => "user-minus",
            Self::UserPen => "user-pen",
            Self::UserPlus => "user-plus",
            Self::UserRound => "user-round",
            Self::UserRoundCheck => "user-round-check",
            Self::UserRoundCog => "user-round-cog",
            Self::UserRoundMinus => "user-round-minus",
            Self::UserRoundPen => "user-round-pen",
            Self::UserRoundPlus => "user-round-plus",
            Self::UserRoundSearch => "user-round-search",
            Self::UserRoundX => "user-round-x",
            Self::UserSearch => "user-search",
            Self::UserX => "user-x",
            Self::Users => "users",
            Self::UsersRound => "users-round",
            Self::Utensils => "utensils",
            Self::UtensilsCrossed => "utensils-crossed",
            Self::UtilityPole => "utility-pole",
            Self::Variable => "variable",
            Self::Vault => "vault",
            Self::VectorSquare => "vector-square",
            Self::Vegan => "vegan",
            Self::VenetianMask => "venetian-mask",
            Self::Venus => "venus",
            Self::VenusAndMars => "venus-and-mars",
            Self::Vibrate => "vibrate",
            Self::VibrateOff => "vibrate-off",
            Self::Video => "video",
            Self::VideoOff => "video-off",
            Self::Videotape => "videotape",
            Self::View => "view",
            Self::Voicemail => "voicemail",
            Self::Volleyball => "volleyball",
            Self::Volume => "volume",
            Self::Volume1 => "volume-1",
            Self::Volume2 => "volume-2",
            Self::VolumeOff => "volume-off",
            Self::VolumeX => "volume-x",
            Self::Vote => "vote",
            Self::Wallet => "wallet",
            Self::WalletCards => "wallet-cards",
            Self::WalletMinimal => "wallet-minimal",
            Self::Wallpaper => "wallpaper",
            Self::Wand => "wand",
            Self::WandSparkles => "wand-sparkles",
            Self::Warehouse => "warehouse",
            Self::WashingMachine => "washing-machine",
            Self::Watch => "watch",
            Self::Waves => "waves",
            Self::WavesLadder => "waves-ladder",
            Self::Waypoints => "waypoints",
            Self::Webcam => "webcam",
            Self::Webhook => "webhook",
            Self::WebhookOff => "webhook-off",
            Self::Weight => "weight",
            Self::Wheat => "wheat",
            Self::WheatOff => "wheat-off",
            Self::WholeWord => "whole-word",
            Self::Wifi => "wifi",
            Self::WifiCog => "wifi-cog",
            Self::WifiHigh => "wifi-high",
            Self::WifiLow => "wifi-low",
            Self::WifiOff => "wifi-off",
            Self::WifiPen => "wifi-pen",
            Self::WifiZero => "wifi-zero",
            Self::Wind => "wind",
            Self::WindArrowDown => "wind-arrow-down",
            Self::Wine => "wine",
            Self::WineOff => "wine-off",
            Self::Workflow => "workflow",
            Self::Worm => "worm",
            Self::WrapText => "wrap-text",
            Self::Wrench => "wrench",
            Self::X => "x",
            Self::Youtube => "youtube",
            Self::Zap => "zap",
            Self::ZapOff => "zap-off",
            Self::ZoomIn => "zoom-in",
            Self::ZoomOut => "zoom-out",
        };
        write!(f, "{name}")
    }
}
