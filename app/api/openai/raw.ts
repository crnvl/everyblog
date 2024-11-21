import { OpenAiRawRequestParams } from "@/misc/apiRoutes";
import { NextApiRequest, NextApiResponse } from "next";
import OpenAI from "openai";
const openai = new OpenAI();

const handler = async (req: NextApiRequest, res: NextApiResponse) => {
    let body: OpenAiRawRequestParams = req.body;

    const completion = await openai.chat.completions.create({
        model: "gpt-3.5-turbo",
        messages: [
            {
                role: "system",
                content: body.prompt,
            }
        ]
    });

    res.status(200).json({ data: completion.choices[0].message });
};
