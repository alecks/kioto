import Head from "next/head";
import styles from "../styles/Home.module.css";

export default function Home() {
    return (
        <div className={styles.container}>
            <Head>
                <title>Kioto</title>
            </Head>

            <main className={styles.main}>
                <h1 className={styles.title}>
                    Welcome to <span className={styles.emphasis}>Kioto</span>
                </h1>

                <p className={styles.description}>
                    Extensible calendars and scheduling for Discord.
                </p>

                <div className={styles.grid}>
                    <a href="/book" className={styles.card}>
                        <h3>Documentation &rarr;</h3>
                        <p>Find out how to use Kioto to its max, or even self-host it.</p>
                    </a>

                    <a href="#" className={styles.card}>
                        <h3>Invite &rarr;</h3>
                        <p>Add the bot to your Discord server and get started.</p>
                    </a>

                    <a
                        href="#"
                        className={styles.card}
                    >
                        <h3>Examples &rarr;</h3>
                        <p>Delve into examples of how to schedule tasks with TypeScript.</p>
                    </a>

                    <a
                        href="#"
                        className={styles.card}
                    >
                        <h3>Source &rarr;</h3>
                        <p>Help make Kioto by contributing and reporting issues.</p>
                    </a>
                </div>
            </main>

            <footer className={styles.footer}>
                &copy; 2020&nbsp;<span className={styles.separator}>
                    &bull;
                </span>&nbsp;<a
                    href="https://vercel.com"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    Hosted by <img src="/vercel.svg" alt="Vercel Logo" className={styles.logo} />
                </a>
            </footer>
        </div>
    );
}
