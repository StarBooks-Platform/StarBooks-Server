using Refit;

namespace StarBooks.Domain.Core.BooksDataSource;

public interface IBooksApi
{
    [Get("/volumes")]
    Task<BooksSearchDto> GetBooks(
        [Query] [AliasAs("q")] string searchTerm,
        [Query] [AliasAs("maxResults")] int maxResults
    );
}
