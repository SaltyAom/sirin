using Microsoft.AspNetCore.Mvc;

namespace Controllers;

[ApiController]
[Route("/")]
public class BaseController : ControllerBase {
    [HttpGet]
    public string Get() => "Sirin";
}