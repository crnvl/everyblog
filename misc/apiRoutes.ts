import { OpenAiRawRequestParams, OpenAiRawResponseParams } from '@/app/api/openai/raw';
import axios from 'axios';

const api = axios.create();

export const endpoints = {
    openAi: {
        getRawResponse: async (params: OpenAiRawRequestParams): Promise<OpenAiRawResponseParams> => {
            return api.post('/openai/raw', params);
        },
    }
};