import type { Route } from "./+types/home";
import styles from "./home.module.scss";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

export default function Home() {
  let tabTitle: string = "Timetable";

  return (
    <>
      <style>
        {`
        body {
          display: flex;
          width: 100dvw;
          height: 100dvh;
          
          align-items: center;
          justify-content: center;
        }
      `}
      </style>

      <div className={styles.root}>
        <div className={styles.content}>
          <div className={styles.mainContent}>
            <h1 className={styles.header}>{tabTitle}</h1>
          </div>

          <div className={styles.navbar}>
            <p className={styles.active}>text</p>
            <p>text</p>
            <p>text</p>
          </div>
        </div>
      </div>
    </>
  );
}
