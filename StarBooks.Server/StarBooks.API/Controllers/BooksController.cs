using Microsoft.AspNetCore.Mvc;
using StarBooks.Domain.Core.BooksDataSource;

namespace StarBooks.API.Controllers;

[ApiController]
[Route("api/[controller]")]
public class BooksController: ControllerBase
{
    private readonly IBooksApi _booksApi;
    
    public BooksController(IBooksApi booksApi)
    {
        _booksApi = booksApi ?? throw new ArgumentNullException(nameof(booksApi));
    }
    
    [HttpGet]
    public async Task<BooksSearchDto> Get([FromQuery(Name = "q")] string searchTerm)
    {
        var books = await _booksApi.GetBooks(searchTerm);
        
        return books;
    }
}