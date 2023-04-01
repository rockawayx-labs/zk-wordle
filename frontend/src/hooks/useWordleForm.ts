import { createFormContext } from "@mantine/form";

export interface WordleFormValues {
  first: string;
  second: string;
  third: string;
  fourth: string;
  fifth: string;
}

export const [FormProvider, useFormContext, useForm] =
  createFormContext<WordleFormValues>();
