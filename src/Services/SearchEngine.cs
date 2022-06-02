using Meilisearch;

using Models;

namespace Services;

public interface ISearchEngine {
    Task<IEnumerable<int>> Search(string Keyword, int Batch);
}

public class SearchEngine : ISearchEngine {
    private readonly Meilisearch.Index _Index;
    private readonly static int _BatchSize = 25;
    private readonly static Dictionary<string, string> _Filters = new Dictionary<string, string> {
        { "yuri", "(tags != \"yaoi\") AND (tags != \"yuri or ice\") AND (tags != \"yuuri\") AND (tags != \"males only\")" }
    };

    private static int GetOffset(int batch) => (batch - 1) * _BatchSize;

    public SearchEngine() {
        var client = new MeilisearchClient("http://localhost:7700");
        _Index = client.Index("hentai");
    }

    public async Task<IEnumerable<int>> Search(string Keyword, int Batch) {
        var Filter = _Filters[Keyword] ?? "";

        var results = await _Index.SearchAsync<Hentai>(Keyword, new SearchQuery {
            Filter = Filter,
            Limit = _BatchSize,
            Offset = GetOffset(Batch)
        });

        return results.Hits.Select(x => x.id);
    }
}