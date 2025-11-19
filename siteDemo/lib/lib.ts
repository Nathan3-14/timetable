import fs from "fs";

export function readJsonFile(filePath: string): void {
  const jsonString: string = fs.readFile(
    filePath,
    "utf8",
    (err, fileContent) => {
      if (err) {
        console.log(`error reading file: ${err}`);
        return err;
      }
      return fileContent;
    },
  );

  console.log(jsonString);
}
