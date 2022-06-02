namespace Models;

public class Hentai {
    public Int32 id { get; set; }
    public string? title { get; set; }
    public IEnumerable<string>? tags { get; set; }
    public Int16 page { get; set; }
}
