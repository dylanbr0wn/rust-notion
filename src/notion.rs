pub(crate) mod types;

pub struct NotionClient {
    client: reqwest::Client,
}

impl NotionClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn notion_request(
        &self,
        my_body: types::PageBody,
    ) -> Result<types::PageResponse, reqwest::Error> {
        let url = "https://www.notion.so/api/v3/loadPageChunk";
        let response = self
            .client
            .post(url)
            .json(&my_body)
            .header("Accept", "application/json")
            .send()
            .await?;
        let body = response.json::<types::PageResponse>().await?;

        Ok(body)
    }

    pub async fn get_page(&self, page_id: &str) -> Result<types::PageResponse, reqwest::Error> {
        let my_page = types::PageBody {
            limit: 100,
            chunkNumber: 0,
            verticalColumns: false,
            pageId: page_id.to_string(),
            cursor: types::Cursor { stack: vec![] },
        };

        // println!("{}", serde_json::to_string(&my_page).unwrap());
        self.notion_request(my_page).await

        // match  {
        //     Ok(body) => body.recordMap.block,
        //     Err(e) => self.handle_request_error(e),
        // }
    }

    // pub fn parse_page(&self, block: HashMap<String, types::Block>, page_id: &str) {
    //     let mut keys: Vec<String>;

    //     loop {
    //         keys = block.keys().map(|x| x.to_string()).collect();

    //         let pending_blocks = keys.iter().flat_map(|id| {
    //             let blockdata = block.get(id).unwrap();
    //             let content = blockdata.value.content.as_ref();
    //             let value: Vec<String>;
    //             match content {
    //                 Some(v) if blockdata.value.kind == "page" && id != page_id => value = vec![],
    //                 Some(v) => value = v.to_vec(),
    //                 None => value = vec![],
    //             }

    //             if value.is_empty() {
    //                 value
    //             } else {
    //                 value
    //                     .iter()
    //                     .filter(|id| block.contains_key(*id))
    //                     .map(|x| x.to_string())
    //                     .collect()
    //             }

    //             // let text = value.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    //             // text
    //         });
    //     }
    // }
}

// let client = reqwest::Client::new();
