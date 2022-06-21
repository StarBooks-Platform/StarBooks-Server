using System.Text.Json.Serialization;
using StarBooks.Domain.Books;

namespace StarBooks.Domain.Core.BooksDataSource;

public record BooksSearchDto(
    [property: JsonPropertyName("totalItems")] int TotalItems,
    [property: JsonPropertyName("items")] List<BookModel> Items
);
