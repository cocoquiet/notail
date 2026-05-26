pub struct Notice {
    id: u32, // wr_id
    title: String,
    sca: u8, // 일반공지, 학사, 장학, 심컴, 첨컴, 인컴, 글솝, 플솝[구.심컴], ICT융합[학부], 대학원, 대학원 계약학과
    author: String,
    article: String,
    created_at: String,
    updated_at: String,
}

impl Notice {
    pub fn new(
        id: u32,
        title: String,
        sca: u8,
        author: String,
        article: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Notice {
            id,
            title,
            sca,
            author,
            article,
            created_at,
            updated_at,
        }
    }
}
