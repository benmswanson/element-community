#[derive(Clone, PartialEq)]
pub struct BookCardData {
    pub title: &'static str,
    pub author: &'static str,
    pub description: &'static str,
    pub cover_url: &'static str,
    pub month: &'static str,
    pub primary_href: Option<&'static str>,
    pub primary_label: Option<&'static str>,
    pub secondary_href: Option<&'static str>,
    pub secondary_label: Option<&'static str>,
}

#[derive(Clone, PartialEq)]
pub struct EventCardData {
    pub title: &'static str,
    pub date: &'static str,
    pub description: &'static str,
    pub location: Option<&'static str>,
    pub image_url: Option<&'static str>,
    pub img_transform: Option<&'static str>,
    pub primary_href: Option<&'static str>,
    pub primary_label: Option<&'static str>,
    pub secondary_href: Option<&'static str>,
    pub secondary_label: Option<&'static str>,
}

#[derive(Clone, PartialEq)]
pub struct TimelineEntry {
    pub when: &'static str,
    pub title: &'static str,
    pub detail: &'static str,
    pub href: Option<&'static str>,
    pub link_label: Option<&'static str>,
    pub image_url: Option<&'static str>,
    pub active: bool,
}

#[derive(Clone, PartialEq)]
pub struct CommunityInfoCard {
    pub label: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub href: Option<&'static str>,
    pub primary: bool,
}

pub const BOOK_CLUB_CURRENT: BookCardData = BookCardData {
    title: "May Book Club",
    author: "Maggie O'Farrell",
    description: "",
    cover_url: "/assets/ham.webp",
    month: "May 28, 2026 · 8:00 PM",
    primary_href: Some("https://partiful.com/e/kHobWBFIKZg7PHB8zQC6?c=At7wGl2m"),
    primary_label: Some("RSVP"),
    secondary_href: Some("https://www.goodreads.com/book/show/43890641-hamnet"),
    secondary_label: Some("Goodreads"),
};

pub const RUN_CLUB_FEATURED: EventCardData = EventCardData {
    title: "Ralle Track Group with Element Run Club",
    date: "Wednesday, May 13 · 7:00 PM",
    description: "",
    location: Some("McCarren Park"),
    image_url: Some("https://ilove.sweatpals.com/api/files/9b735b97-c4a3-400e-ad3c-3306ce357957?variant=l"),
    img_transform: None,
    primary_href: Some("https://sweatpals.com/event/ralle-track-group/2026-05-13"),
    primary_label: Some("Register"),
    secondary_href: None,
    secondary_label: None,
};

pub const COMMUNITY_EVENT_FEATURED: EventCardData = EventCardData {
    title: "Good Saturdays 02",
    date: "Saturday, May 30 · 12:00 PM",
    description: "",
    location: Some("Element Training Club, Grand St, Brooklyn"),
    image_url: Some("https://ilove.sweatpals.com/api/files/2924d997-9787-4afa-9a2a-1b76fdc491c3?variant=l"),
    img_transform: None,
    primary_href: Some("https://sweatpals.com/event/good-saturdays-02-the-workout-presented-by-element-training-club-x-ralle"),
    primary_label: Some("Register"),
    secondary_href: None,
    secondary_label: None,
};

pub const BOOK_CLUB_TIMELINE: [TimelineEntry; 2] = [
    TimelineEntry {
        when: "May 28, 2026",
        title: "May Book Club",
        detail: "Maggie O'Farrell",
        href: Some("https://partiful.com/e/kHobWBFIKZg7PHB8zQC6?c=At7wGl2m"),
        link_label: Some("RSVP"),
        image_url: None,
        active: true,
    },
    TimelineEntry {
        when: "April 30, 2026",
        title: "Headshot",
        detail: "Rita Bullwinkel",
        href: None,
        link_label: None,
        image_url: None,
        active: false,
    },
];

pub const RUN_CLUB_TIMELINE: [TimelineEntry; 7] = [
    TimelineEntry {
        when: "Saturday, May 9 · 9:00 AM",
        title: "Saturday Morning Run",
        detail: "Today's run",
        href: Some("https://www.strava.com/routes/3476595695882355858"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/OJ2BG6YW5JETPDEGRSS6RUHPVFMPRSFORDDOZN6CMJNFQF7S5SP442ZAFB7YKPKIODEJA42Q7ARRM6XRF5QYPR5UN5UMYWOFL7NZ4NN5GNUD34GEWVFRI2NKTDA6F6JGTDEOENUQAAWBPPI3UL4CHNOK2IOXULUCXNGTRPZVR4AOQ34EP6H5R7KEBJSRAGWNYI======"),
        active: true,
    },
    TimelineEntry {
        when: "Saturday, May 3 · 9:00 AM",
        title: "A Park Named Maria",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3484708461539968178"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/OFIDOJU4DG24FLBVI2BE3AAXF5442KVELW42E53YNN3SCMJPXEDKAYMUEP34UFYFE2WOA6ZH2GEPMT2YDTDDMSCDQI74GD7LL7Q2ZG3ATTNEI55LAATLDVRVXZ4GYUHOHBWSJK3ZVXHEMCWUXPEQHHYN5OY3DWGPI5HQGZMYSZUXSU3Q7IXLUSHUV2RKNVPKOU======"),
        active: false,
    },
    TimelineEntry {
        when: "Wednesday, April 29 · 6:30 PM",
        title: "The K it is",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3484365772880850286"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/SMEKSUDCVLPYOFIOJO3CEW3IPCAD63AKQEQ7BARCNUG5GY3WIIS2GMGUMV32UJ6AF2U2CNT257TFUDQKC4VTSXMKGWBVN2NNQKSSI2XP3P2TUZPRYPJG7X7YE5NZZHUIKPPM5LBDQVNJR6DU47SMIZSBSHGHZ33QLQJ4NLYXSFUBU2RDCJYBS2M22UYXSHBGOI======"),
        active: false,
    },
    TimelineEntry {
        when: "Wednesday, April 22 · 6:30 PM",
        title: "Park Pace",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3469355877184440860"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/5HYO55RACTPLMU3WKTF75UR4OVOZ5GRXPICVCALPNMUVIITQS42TAYKOR4QJOWZDVZZNGBUF4VNDQYOZI4MXSKWTZXPWRDHDD3AK7RAM7UXDVEDUAQW5PVINDSLYFZMVZVTOSVBYVAU6P2OOQ4BMFARRH4TG7TEL3C2R5VIGY4H4O6IJE4GLMV2CV5T72A3TTA======"),
        active: false,
    },
    TimelineEntry {
        when: "Saturday, April 18 · 9:00 AM",
        title: "To The Wharf",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3480270817809452530"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/2P3KS7NLCSFNKIATCIYWJV64CCQVKCLZEPQNWVXGRAXYOW5ARUIAVR6N5FOCLFS3CZVUCH4ZF64GPT3WTACH4WZO22RXVJFPGDY3SZBTFZK6WKQ6RCNLTW6VFZGKPZCHEU7QP5L4YRFJHBDYN2CY2FVH5M5JGJRALYYCI3SO6FM2HINS7NGSAMJ45ZPMWEC674======"),
        active: false,
    },
    TimelineEntry {
        when: "Wednesday, April 15 · 6:30 PM",
        title: "Angler Fish",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3478757606243375166"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/YMO3Q2KK72RKGVLNXKYRUWAH2SY7TORUBLL67WDXUN5DOHB76PFTYJFPP5LYE5MDXJXMTKDI5BMPCXAXCQI6Q2EAR52QZOOAPRLXMQA27LA4IGRBDDRPN7OP7TYBYBR5HC4L57AR6XOW6RY7O2JT7FVLQZBJZIC6ZYPMV4I3GFS62HAOYQKY5S4CBMNWRU3QGE======"),
        active: false,
    },
    TimelineEntry {
        when: "Saturday, April 11 · 9:00 AM",
        title: "Feel the Stern",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3476595695882355858"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/OJ2BG6YW5JETPDEGRSS6RUHPVFMPRSFORDDOZN6CMJNFQF7S5SP442ZAFB7YKPKIODEJA42Q7ARRM6XRF5QYPR5UN5UMYWOFL7NZ4NN5GNUD34GEWVFRI2NKTDA6F6JGTDEOENUQAAWBPPI3UL4CHNOK2IOXULUCXNGTRPZVR4AOQ34EP6H5R7KEBJSRAGWNYI======"),
        active: false,
    },
];

pub const COMMUNITY_EVENTS_TIMELINE: [TimelineEntry; 8] = [
    TimelineEntry {
        when: "Saturday, May 30",
        title: "Good Saturdays 02",
        detail: "Upcoming",
        href: Some("https://sweatpals.com/event/good-saturdays-02-the-workout-presented-by-element-training-club-x-ralle"),
        link_label: Some("Register"),
        image_url: Some("https://ilove.sweatpals.com/api/files/2924d997-9787-4afa-9a2a-1b76fdc491c3?variant=l"),
        active: true,
    },
    TimelineEntry {
        when: "Saturday, May 9",
        title: "HYROX Half Sim",
        detail: "Past event",
        href: Some("https://docs.google.com/spreadsheets/d/1uqfHxSrWA6c35zo59BU4l4MNw2badp695jgBYMOnhi8/edit?gid=1161720329#gid=1161720329"),
        link_label: Some("View Event"),
        image_url: Some("/assets/sim.jpeg"),
        active: false,
    },
    TimelineEntry {
        when: "Saturday, May 2 · 2:00 PM ET",
        title: "Good Saturdays: The Social",
        detail: "Past event",
        href: Some("https://partiful.com/e/VoAlpsKNThTgyprJaSIU"),
        link_label: Some("View Event"),
        image_url: Some("https://firebasestorage.googleapis.com/v0/b/getpartiful.appspot.com/o/external%2Fuser%2FUHRiYw9aKMeD6W6RofPbz7sjeMx1%2FTIhQ5EdeplVHWC52RZJbm?alt=media&token=abaad7ae-1a79-4421-9c7e-9cf7f30a7e84"),
        active: false,
    },
    TimelineEntry {
        when: "Friday, April 25",
        title: "Community Meetup",
        detail: "Past event",
        href: Some("https://partiful.com/e/eK4W7NufhtWs20oXbeQV"),
        link_label: Some("View Event"),
        image_url: Some("https://firebasestorage.googleapis.com/v0/b/getpartiful.appspot.com/o/external%2Fuser%2FZOk91HF7JyNhpWXc1BhhxGPGWdG2%2Fs1Dk8hckJ9M7lbJJgZu75?alt=media&token=6a22e362-75eb-4f5f-8034-a21ca275389a"),
        active: false,
    },
    TimelineEntry {
        when: "Saturday, April 18",
        title: "HYROX PFT",
        detail: "Past event",
        href: Some("https://withforte.co/events/wdsbGbuxMw6ynjLMrcHj"),
        link_label: Some("View Event"),
        image_url: Some("https://withforte.co/_next/image?url=https%3A%2F%2Ffleetath.s3.us-east-1.amazonaws.com%2Ffile_20260324202010420_29119816.PNG&w=1200&q=75"),
        active: false,
    },
    TimelineEntry {
        when: "Saturday, April 11",
        title: "Run + HYROX + Sculpt Social",
        detail: "Past event",
        href: Some("https://www.rallemovements.com/event-details-registration/ralle-element-training-club-run-hyrox-or-sculpt-social"),
        link_label: Some("View Event"),
        image_url: Some("https://static.wixstatic.com/media/ff0109_8b354e42729e4fa1b96f0402cf77629c~mv2.jpg/v1/fill/w_4125,h_6224,al_c,q_90/ff0109_8b354e42729e4fa1b96f0402cf77629c~mv2.jpg"),
        active: false,
    },
    TimelineEntry {
        when: "Friday, March 14",
        title: "Community Night",
        detail: "Past event",
        href: Some("https://partiful.com/e/MheGKxIXEhqwMXeF1fJh"),
        link_label: Some("View Event"),
        image_url: Some("https://firebasestorage.googleapis.com/v0/b/getpartiful.appspot.com/o/external%2Fuser%2FwLlYiJOlCJMCDD1qK73vHWjmZiA2%2FRAnTWQS9QXRG6aXd7QOPH?alt=media&token=c063cb0d-37b8-4752-b23c-29b4c84552d6"),
        active: false,
    },
    TimelineEntry {
        when: "Wednesday, February 19",
        title: "Run + HYROX Social",
        detail: "Past event",
        href: Some("https://www.rallemovements.com/event-details-registration/ralle-element-training-club-run-hyrox-social?fbclid=PAQ0xDSwQELiRleHRuA2FlbQIxMQBzcnRjBmFwcF9pZA8xMjQwMjQ1NzQyODc0MTQAAafflu5JWBPwhIg5HDZRpvtbnwKAeLQSfcFwTX9HOYSNDeTKuXJfTJcWZ9Gykw_aem_Vvg5jiBZM5klee0MdU_6rg"),
        link_label: Some("View Event"),
        image_url: Some("https://static.wixstatic.com/media/ff0109_239b533ee7924f1d948049546b5a58f8~mv2.jpeg/v1/fill/w_1024,h_768,al_c,q_85/ff0109_239b533ee7924f1d948049546b5a58f8~mv2.jpeg"),
        active: false,
    },
];

pub const BOOK_CLUB_INFO: [CommunityInfoCard; 3] = [
    CommunityInfoCard {
        label: "Join",
        title: "Join the Group",
        description: "Hop into the Goodreads group to see picks, chat, and track your reading.",
        href: Some("https://www.goodreads.com/group/show/7267353-element-book-club"),
        primary: true,
    },
    CommunityInfoCard {
        label: "Read",
        title: "Read the Book",
        description: "Read at your own pace and show up for the conversation when the meetup date arrives.",
        href: Some("https://www.amazon.com/dp/0525657746"),
        primary: false,
    },
    CommunityInfoCard {
        label: "Discuss",
        title: "Meet and Discuss",
        description: "RSVP to the meetup and come talk through the monthly pick together.",
        href: Some("https://partiful.com/e/kHobWBFIKZg7PHB8zQC6?c=At7wGl2m"),
        primary: false,
    },
];

pub const RUN_CLUB_INFO: [CommunityInfoCard; 3] = [
    CommunityInfoCard {
        label: "Strava",
        title: "Follow on Strava",
        description: "See routes, track runs, and follow along with the crew.",
        href: Some("https://www.strava.com/athletes/161795832"),
        primary: true,
    },
    CommunityInfoCard {
        label: "Cadence",
        title: "Every Wed and Sat",
        description: "We run twice a week. Show up and join in without a complicated sign-up flow.",
        href: None,
        primary: false,
    },
    CommunityInfoCard {
        label: "Pace",
        title: "All Paces Welcome",
        description: "Whether you are training for something specific or just getting started, the group stays friendly and accessible.",
        href: None,
        primary: false,
    },
];

pub const COMMUNITY_EVENTS_INFO: [CommunityInfoCard; 3] = [
    CommunityInfoCard {
        label: "Socials",
        title: "Post-workout hangs",
        description: "Happy hours, community hangs, and one-off ways to spend time together outside class.",
        href: None,
        primary: false,
    },
    CommunityInfoCard {
        label: "Fitness Events",
        title: "Open challenges",
        description: "Events, races, and training-adjacent experiences that bring the broader community together.",
        href: None,
        primary: false,
    },
    CommunityInfoCard {
        label: "Bring a Friend",
        title: "Easy to invite people in",
        description: "Most events work well for members and newcomers alike, so it is easy to bring someone along.",
        href: None,
        primary: false,
    },
];
