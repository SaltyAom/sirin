using Services;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddScoped<ISearchEngine, SearchEngine>();

builder.Services.AddControllers();

var app = builder.Build();

app.MapControllers();

app.Run();
