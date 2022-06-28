using Microsoft.AspNetCore.Mvc;
using StarBooks.Domain.Core.BooksDataSource;
using StarBooks.Infrastructure.Books;

namespace StarBooks.API.Controllers;

[ApiController]
[Route("api/[controller]")]
public class BooksController : ControllerBase
{
    private readonly IBooksApi _booksApi;
    private readonly BookContext _context;

    public BooksController(IBooksApi booksApi, BookContext context)
    {
        _booksApi = booksApi ?? throw new ArgumentNullException(nameof(booksApi));
        _context = context ?? throw new ArgumentNullException(nameof(context));
    }

    [HttpGet]
    public async Task<BooksSearchDto> Get(
        [FromQuery(Name = "q")] string searchTerm,
        [FromQuery] int maxResults
    )
    {
        var books = await _booksApi.GetBooks(searchTerm, maxResults);

        return books;
    }

    [HttpPost]
    public async Task<IActionResult> Post(
        [FromQuery(Name = "q")] string searchTerm,
        [FromQuery] int maxResults
    )
    {
        var booksDto = await _booksApi.GetBooks(searchTerm, maxResults);
        var books = booksDto.Items.Select(
            b =>
            {
                b.Authors = b.VolumeInfo.Authors;
                b.Categories = b.VolumeInfo.Categories;
                b.Identifiers = b.VolumeInfo.Identifiers;
                return b;
            }
        );

        var bookModels = books.ToList();
        _context.Books.AddRange(bookModels);
        await _context.SaveChangesAsync();

        return Ok(new { Message = "Books added to database", bookModels.Count });
    }
}
