mod embedding;
mod postgres;
mod error;

#[tokio::main]
async fn main() {
    const OLLAMA_URL: &str = "http://localhost:11434/api/embeddings"; // change this to your ollama url

    // connect to database, change this to your database url
    let pool: sqlx::Pool<sqlx::Postgres> = match postgres::connect_db("postgres://postgres:12345@localhost/test").await {
        Ok(pool) => pool,
        Err(err) => {
            println!("No Pool!: {:?}", err);

            return
        }
    };

    //println!("Database Pool: {:?}", pool);

    
    // insert mock data into database
    let text1: &str = "Hi, I bought some headache pills earlier. They worked great! Thank you so much.";
    let body1: String = r#"{"model": "mxbai-embed-large", "prompt": "template"}"#.replace("template", text1);
    let embedding_data1: embedding::EmbeddingResponse = embedding::fetch_embedding(OLLAMA_URL, &body1).await.unwrap(); 
    let text2: &str = "Hello, I had a question about my prescription. The pharmacist was really helpful in explaining how to take it properly. I appreciate the clear instructions.";
    let body2: String = r#"{"model": "mxbai-embed-large", "prompt": "template"}"#.replace("template", text2);
    let embedding_data2: embedding::EmbeddingResponse = embedding::fetch_embedding(OLLAMA_URL, &body2).await.unwrap(); 
    let text3: &str = "I came in looking for another brand of headache pills. I was able to find one with less than half the cost. I would recommend them to anyone.";
    let body3: String = r#"{"model": "mxbai-embed-large", "prompt": "template"}"#.replace("template", text3);
    let embedding_data3: embedding::EmbeddingResponse = embedding::fetch_embedding(OLLAMA_URL, &body3).await.unwrap(); 
    let text4: &str = "I purchased some vitamins, and when I got home, I noticed that they were close to their expiration date. It would be helpful if the staff checked the dates more regularly.";
    let body4: String = r#"{"model": "mxbai-embed-large", "prompt": "template"}"#.replace("template", text4);
    let embedding_data4: embedding::EmbeddingResponse = embedding::fetch_embedding(OLLAMA_URL, &body4).await.unwrap(); 
    let text5: &str = "I had some questions about a new medication, and the pharmacist took the time to explain everything to me in detail. I feel much more confident about taking it now.";
    let body5: String = r#"{"model": "mxbai-embed-large", "prompt": "template"}"#.replace("template", text5);
    let embedding_data5: embedding::EmbeddingResponse = embedding::fetch_embedding(OLLAMA_URL, &body5).await.unwrap(); 
    let mut query: String = "INSERT INTO reviews (text, embedding) VALUES (".to_owned();
    query = query + "'" + text1 + "', '[" + embedding_data1.embedding.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",").as_str() + "]'),";
    query = query + "('" + text2 + "', '[" + embedding_data2.embedding.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",").as_str() + "]'),";
    query = query + "('" + text3 + "', '[" + embedding_data3.embedding.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",").as_str() + "]'),";
    query = query + "('" + text4 + "', '[" + embedding_data4.embedding.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",").as_str() + "]'),";
    query = query + "('" + text5 + "', '[" + embedding_data5.embedding.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",").as_str() + "]');";
    embedding::insert_embedding(&pool, &query).await.unwrap();
    

    // get embeddings data from ollama
    let prompt: &str = "I had some questions about a new medication, and the pharmacist took the time to explain everything to me in detail. I feel much more confident about taking it now.";
    let body: String = r#"{"model": "mxbai-embed-large", "prompt": "template"}"#.replace("template", prompt);
    let embedding_data: embedding::EmbeddingResponse = match embedding::fetch_embedding(OLLAMA_URL, &body).await {
        Ok(embedding) => { 
            embedding
        }
        Err(err) => {
            println!("No Embedding!: {:?}", err);
            
            embedding::EmbeddingResponse { embedding: vec![] }
        }
    };

    //println!("{:?}", embedding_data.embedding);


    // get cosine similarity
    let mut query: String = "SELECT text, 1 - (embedding <=> '[".to_owned();
    query = query + embedding_data.embedding.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",").as_str();
    query = query + "]') AS cosine_similarity FROM reviews ORDER BY cosine_similarity DESC;";
  
    let cosine_similarity: Vec<embedding::Feedback> = match embedding::query_embedding(&pool, &query).await {
        Ok(cosine_similarity) => { 
            cosine_similarity
        }
        Err(err) => {
            println!("No Reviews!: {:?}", err);
              
            vec![]
        }
    };
  
    println!("{:?}", cosine_similarity);
}
  