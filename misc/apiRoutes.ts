import axios from 'axios';

const api = axios.create();

export const endpoints = {
    openAi: {
        getRawResponse: async (params: OpenAiRawRequestParams) => {
            return api.post('/openai/raw', params);
        },
    }
};

export interface OpenAiRawRequestParams {
    prompt: string;
}