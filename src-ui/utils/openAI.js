import OpenAI from "openai";

const openAIClient = new OpenAI({
    baseURL: "https://api.moonshot.cn/v1",
    apiKey: "",
    dangerouslyAllowBrowser: true
});

export default openAIClient;