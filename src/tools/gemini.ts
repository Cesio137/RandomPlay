import {
    EnhancedGenerateContentResponse,
    GoogleGenerativeAI,
    HarmBlockThreshold,
    HarmCategory,
    SafetySetting,
} from "@google/generative-ai";

const gemAI = new GoogleGenerativeAI(process.env.GEMINI_API_KEY);
const threshold = HarmBlockThreshold.BLOCK_NONE;
const safetySettings: SafetySetting[] = [
    { category: HarmCategory.HARM_CATEGORY_DANGEROUS_CONTENT, threshold },
    { category: HarmCategory.HARM_CATEGORY_HARASSMENT, threshold },
    { category: HarmCategory.HARM_CATEGORY_HATE_SPEECH, threshold },
    { category: HarmCategory.HARM_CATEGORY_SEXUALLY_EXPLICIT, threshold },
];

export const gemini = {
    text: gemAI.getGenerativeModel({ model: "gemini-1.5-flash", safetySettings }),
    getText(response: EnhancedGenerateContentResponse) {
        try {
            return { success: true, text: response.text() };
        } catch (error) {
            return { success: false, error };
        }
    },
};
