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
    title: "Headshot",
    author: "Rita Bullwinkel",
    description: "An electrifying novel about a women's boxing tournament in Reno, Nevada, following eight teenage competitors and the lives pressing in around each of them.",
    cover_url: "https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1691077222i/174156218.jpg",
    month: "April 30, 2026 · 8:00 PM",
    primary_href: Some("https://partiful.com/e/Z0UI3U0H7Ooe1XgYdfHr?c=AfUZfvML"),
    primary_label: Some("RSVP"),
    secondary_href: Some("https://www.goodreads.com/book/show/174156218"),
    secondary_label: Some("Goodreads"),
};

pub const RUN_CLUB_FEATURED: EventCardData = EventCardData {
    title: "Wednesday Longer Run",
    date: "Wednesday, April 29 · 6:30 PM",
    description: "Join this week's group run. All paces welcome. We start together and finish together.",
    location: None,
    image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/SMEKSUDCVLPYOFIOJO3CEW3IPCAD63AKQEQ7BARCNUG5GY3WIIS2GMGUMV32UJ6AF2U2CNT257TFUDQKC4VTSXMKGWBVN2NNQKSSI2XP3P2TUZPRYPJG7X7YE5NZZHUIKPPM5LBDQVNJR6DU47SMIZSBSHGHZ33QLQJ4NLYXSFUBU2RDCJYBS2M22UYXSHBGOI======"),
    primary_href: None,
    primary_label: None,
    secondary_href: Some("https://www.strava.com/routes/3484365772880850286"),
    secondary_label: Some("View Route"),
};

pub const COMMUNITY_EVENT_FEATURED: EventCardData = EventCardData {
    title: "Community Meetup | April 25th",
    date: "Friday, April 25",
    description: "Come hang with the Element community. Details on the Partiful page.",
    location: Some("667 Grand St, Brooklyn, NY 11211"),
    image_url: Some("https://firebasestorage.googleapis.com/v0/b/getpartiful.appspot.com/o/external%2Fuser%2FZOk91HF7JyNhpWXc1BhhxGPGWdG2%2Fs1Dk8hckJ9M7lbJJgZu75?alt=media&token=6a22e362-75eb-4f5f-8034-a21ca275389a"),
    primary_href: Some("https://partiful.com/e/eK4W7NufhtWs20oXbeQV"),
    primary_label: Some("RSVP"),
    secondary_href: None,
    secondary_label: None,
};

pub const BOOK_CLUB_TIMELINE: [TimelineEntry; 2] = [
    TimelineEntry {
        when: "April 30, 2026",
        title: "Headshot",
        detail: "Rita Bullwinkel",
        href: None,
        link_label: None,
        image_url: None,
        active: true,
    },
    TimelineEntry {
        when: "May 2026",
        title: "TBD",
        detail: "Suggest the next read.",
        href: Some("https://forms.gle/eQFo1SqXJwRfr3tX6"),
        link_label: Some("Submit a pick"),
        image_url: None,
        active: false,
    },
];

pub const RUN_CLUB_TIMELINE: [TimelineEntry; 6] = [
    TimelineEntry {
        when: "Wednesday, April 22 · 6:30 PM",
        title: "Wednesday Longer Run",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3469355877184440860"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/5HYO55RACTPLMU3WKTF75UR4OVOZ5GRXPICVCALPNMUVIITQS42TAYKOR4QJOWZDVZZNGBUF4VNDQYOZI4MXSKWTZXPWRDHDD3AK7RAM7UXDVEDUAQW5PVINDSLYFZMVZVTOSVBYVAU6P2OOQ4BMFARRH4TG7TEL3C2R5VIGY4H4O6IJE4GLMV2CV5T72A3TTA======"),
        active: false,
    },
    TimelineEntry {
        when: "Saturday, April 18 · 9:00 AM",
        title: "Saturday Morning Run",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3480270817809452530"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/2P3KS7NLCSFNKIATCIYWJV64CCQVKCLZEPQNWVXGRAXYOW5ARUIAVR6N5FOCLFS3CZVUCH4ZF64GPT3WTACH4WZO22RXVJFPGDY3SZBTFZK6WKQ6RCNLTW6VFZGKPZCHEU7QP5L4YRFJHBDYN2CY2FVH5M5JGJRALYYCI3SO6FM2HINS7NGSAMJ45ZPMWEC674======"),
        active: false,
    },
    TimelineEntry {
        when: "Wednesday, April 15 · 6:30 PM",
        title: "Wednesday Longer Run",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3478757606243375166"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/YMO3Q2KK72RKGVLNXKYRUWAH2SY7TORUBLL67WDXUN5DOHB76PFTYJFPP5LYE5MDXJXMTKDI5BMPCXAXCQI6Q2EAR52QZOOAPRLXMQA27LA4IGRBDDRPN7OP7TYBYBR5HC4L57AR6XOW6RY7O2JT7FVLQZBJZIC6ZYPMV4I3GFS62HAOYQKY5S4CBMNWRU3QGE======"),
        active: false,
    },
    TimelineEntry {
        when: "Saturday, April 11 · 9:00 AM",
        title: "Saturday Morning Run",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3476595695882355858"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/OJ2BG6YW5JETPDEGRSS6RUHPVFMPRSFORDDOZN6CMJNFQF7S5SP442ZAFB7YKPKIODEJA42Q7ARRM6XRF5QYPR5UN5UMYWOFL7NZ4NN5GNUD34GEWVFRI2NKTDA6F6JGTDEOENUQAAWBPPI3UL4CHNOK2IOXULUCXNGTRPZVR4AOQ34EP6H5R7KEBJSRAGWNYI======"),
        active: false,
    },
    TimelineEntry {
        when: "Wednesday, April 8 · 6:30 PM",
        title: "Wednesday Longer Run",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3476735579330903130"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/7QY2CPBLZZCBIPWTPLJIGORUHQP46QRN44B5Y273NANB6TH6RSXRFUTXSDACPSLTUBE7FSNWDLZYWHEBD6M4332NH767D6TMSZTL6CRA6DYAONETPY4WFCYV26MAX4OV7MQZEQD4IQXJZ7V3INEZUK7IT6V227DUKTLKOUJOU7INX6WZ2C67X32USIPUL7W5FY======"),
        active: false,
    },
    TimelineEntry {
        when: "Saturday, April 4 · 9:00 AM",
        title: "Saturday Morning Run",
        detail: "Past run",
        href: Some("https://www.strava.com/routes/3475160646034726692"),
        link_label: Some("View Route"),
        image_url: Some("https://d3o5xota0a1fcr.cloudfront.net/v6/maps/BJNOM7WQ7M6OU6ZJZUSUDZ7BF6ESHANZ3QJSZCFS6ZID6K3TZ37O43J43S5TG36NP3EHBDWKKXZKBUZLLH2FZZIXO5O4BVBJV453EDZO3B2U4UB7XCMGDRPUNJMTFLJJLSPTHAHXAHMF6KOIAOPQAXK4LHSAJMUQ2JHM6YJTU7UMQVCUBX7SDSUVLAR42725WM======"),
        active: false,
    },
];

pub const COMMUNITY_EVENTS_TIMELINE: [TimelineEntry; 6] = [
    TimelineEntry {
        when: "Saturday, May 2 · 2:00 PM ET",
        title: "Good Saturdays: The Social",
        detail: "Upcoming",
        href: Some("https://partiful.com/e/VoAlpsKNThTgyprJaSIU"),
        link_label: Some("RSVP"),
        image_url: Some("https://firebasestorage.googleapis.com/v0/b/getpartiful.appspot.com/o/external%2Fuser%2FUHRiYw9aKMeD6W6RofPbz7sjeMx1%2FTIhQ5EdeplVHWC52RZJbm?alt=media&token=abaad7ae-1a79-4421-9c7e-9cf7f30a7e84"),
        active: true,
    },
    TimelineEntry {
        when: "Friday, April 25",
        title: "Community Meetup",
        detail: "Upcoming",
        href: Some("https://partiful.com/e/eK4W7NufhtWs20oXbeQV"),
        link_label: Some("RSVP"),
        image_url: Some("https://firebasestorage.googleapis.com/v0/b/getpartiful.appspot.com/o/external%2Fuser%2FZOk91HF7JyNhpWXc1BhhxGPGWdG2%2Fs1Dk8hckJ9M7lbJJgZu75?alt=media&token=6a22e362-75eb-4f5f-8034-a21ca275389a"),
        active: true,
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
        href: Some("https://www.amazon.com/Headshot-Novel-Rita-Bullwinkel/dp/0593654129/ref=sr_1_1?crid=2N3LASO74LGJ0&dib=eyJ2IjoiMSJ9.RD3JcqgbdTOwTE51qHLMkZAlUbG1xjeraIG3g9A_HNKz0_3kRe5w_8-iBrzGVXu-mkHGTOORebyKBX0y0TGg43fklHJlEQImpjh_9Nn_Kc4.ohSsz7HmFfDu8aW96Brx9KnlwOwmX0O1iBkGAiUOglo&dib_tag=se&keywords=headshot+rita+bullwinkel&qid=1776451187&sprefix=headshot+rita+%2Caps%2C139&sr=8-1"),
        primary: false,
    },
    CommunityInfoCard {
        label: "Discuss",
        title: "Meet and Discuss",
        description: "RSVP to the meetup and come talk through the monthly pick together.",
        href: Some("https://partiful.com/e/Z0UI3U0H7Ooe1XgYdfHr"),
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
