using System;
using System.Linq;
using System.IO;
using System.Collections.Generic;
using System.Text.Json;
using System.Threading.Tasks;

using Meilisearch;
using RestSharp;

namespace Setup;

internal class Program {
    private class Hentai {
        public Int32 id { get; set; }
        public string? title { get; set; }
        public IEnumerable<string>? tags { get; set; }
        public Int16 page { get; set; }
    }

    private static async Task Main(string[] args) {
        var client = new MeilisearchClient("http://localhost:7700");

        await client.CreateIndexAsync("hentai", "id");
        var index = client.Index("hentai");

        await index.UpdateSettingsAsync(new Settings {
            DisplayedAttributes = new String[] { "id" },
            SortableAttributes = new String[] { "id" },
            SearchableAttributes = new String[] { "tags", "title" },
            FilterableAttributes = new String[] { "tags" },
            RankingRules = new String[] {
                "words",
                "id:desc",
                "attribute",
                "proximity",
                "exactness",
                "typo"
            },
            Synonyms = new Dictionary<string, IEnumerable<string>> {
                { "yaoi", new String[] { "males only" } },
                { "yuri", new String[] { "females only" } }
            }
        });
    

        string[] searchables = Directory.GetFiles("./data", "*.json");
 
        var hentais = searchables
            .OrderByDescending(i => i)
            .Select(file => Task.Run(
                () => JsonSerializer.Deserialize<List<Hentai>>(
                    File.ReadAllText(file)
                )
            ))
            .Select(async task => await task)
            .Where(hentai => hentai.Result != null)
            .SelectMany(i => i.Result!);

        var operation = await index.AddDocumentsAsync<Hentai>(hentais);
        await client.WaitForTaskAsync(operation!.Uid, 5 * 60 * 1000);
    }
}