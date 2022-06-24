using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Owned]
public class SearchInfo
{
    [JsonPropertyName("textSnippet")]
    string TextSnippet { get; set; }
}
