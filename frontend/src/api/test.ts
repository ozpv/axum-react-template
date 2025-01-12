import axios from "axios";

// /test
export const test_api = async (): Promise<String> => {
  const res = await axios
    .get("http://localhost:3000/test", { responseType: "text" })
    .then((res) => {
      if (res.status != 200) {
        return "Something went wrong";
      }
      return res.data;
    });
  return res;
};
