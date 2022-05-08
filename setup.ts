import { MeiliSearch } from 'meilisearch'
import type { Index, EnqueuedTask } from 'meilisearch'

import fetch from 'isomorphic-unfetch'

import { readFile } from 'fs/promises'
import { resolve } from 'path'
import { path as root } from 'app-root-path'

import { exec } from 'child_process'

export interface Hentai {
    id: number
    title: String
    tags: string[]
    page: number
}

export interface MeiliSearchStatus {
    status: 'available' | string
}

const ping = async () => {
    exec('./meilisearch')

    let resolve: () => void

    const ready = new Promise<void>((r) => {
        resolve = r
    })

    const ping = setInterval(async () => {
        try {
            const status: MeiliSearchStatus = await fetch(
                'http://localhost:7700/health'
            ).then((r) => r.json())

            if (status?.status === 'available') resolve()
        } catch (_) {}
    }, 100)

    await ready
    clearInterval(ping)
}

const createClient = async () => {
    await ping()

    const client = new MeiliSearch({ host: 'http://0.0.0.0:7700' })

    let index: Index<Hentai>
    try {
        index = await client.getIndex('hentai')
    } catch (_) {
        await client.createIndex('hentai', {
            primaryKey: 'id'
        })

        index = client.index('hentai')

        await index.updateSettings({
            sortableAttributes: ['id'],
            searchableAttributes: ['tags', 'title']
        })

        await index.updateRankingRules([
            'exactness',
            'attribute',
            'words',
            'typo',
            'proximity',
            'sort',
            'id:desc'
        ])
    }

    const importing: Promise<EnqueuedTask>[] = []

    // Index from newest to oldest
    for (let i = 20; i > 1; i--) {
        importing.push(
            new Promise<EnqueuedTask>(async (done) => {
                const file = await readFile(
                    resolve(root, `./data/searchable${i}.json`),
                    {
                        encoding: 'utf-8'
                    }
                )

                done(await index.addDocuments(JSON.parse(file)))
            })
        )
    }

    const tasks = await Promise.all(importing)

    await client.waitForTasks(
        tasks.map(({ uid }) => uid),
        {
            intervalMs: 10 * 1000,
            timeOutMs: 5 * 60 * 1000
        }
    )

    console.log('Done Indexing')

    process.exit(0)
}

createClient()
