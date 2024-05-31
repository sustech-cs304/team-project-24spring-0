import OpenAI from 'openai'

const openAIClient = new OpenAI({
  baseURL: 'https://api.moonshot.cn/v1',
  apiKey: 'sk-7FQIYKkkojavvBhFzapuClQ7B0yUU08PDnRT0L5J3QGv7jEK',
  dangerouslyAllowBrowser: true,
})

export default openAIClient
