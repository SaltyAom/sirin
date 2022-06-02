using Microsoft.AspNetCore.Mvc;

using Services;

namespace Controllers;

[ApiController]
[Route("/search")]
public class SearchController : ControllerBase {
    private readonly ISearchEngine _searchEngine;

    public SearchController(ISearchEngine SearchEngine) {
        _searchEngine = SearchEngine;
    }

    [HttpGet]
    public String Searchable() => "Searchable";

    [HttpGet("{Query}/{Batch:int}")]
    public async Task<IActionResult> Search(string Query, int Batch) {
        var results = await _searchEngine.Search(Query, Batch);

        return Ok(results);
    }
}
