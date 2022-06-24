using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Owned]
public class VolumeInfo
{
    [JsonPropertyName("title")]
    public string Title { get; set; }

    [JsonPropertyName("authors"), NotMapped]
    public List<AuthorModel> Authors { get; set; }

    [JsonPropertyName("publisher")]
    public string Publisher { get; set; }

    [JsonPropertyName("publishedDate")]
    public string PublishedDate { get; set; }

    [JsonPropertyName("description")]
    public string Description { get; set; }

    [JsonPropertyName("industryIdentifiers"), NotMapped]
    public List<IndustryIdentifierModel> Identifiers { get; set; }

    [JsonPropertyName("pageCount")]
    public int PageCount { get; set; }

    [JsonPropertyName("printType")]
    public string PrintType { get; set; }

    [JsonPropertyName("categories"), NotMapped]
    public List<CategoryModel> Categories { get; set; }

    [JsonPropertyName("averageRating")]
    public double AverageRating { get; set; }

    [JsonPropertyName("ratingsCount")]
    public int RatingsCount { get; set; }

    [JsonPropertyName("imageLinks")]
    public ImageLinks ImageLinks { get; set; }

    [JsonPropertyName("language")]
    public string Language { get; set; }

    [JsonPropertyName("previewLink")]
    public string PreviewLink { get; set; }

    [JsonPropertyName("InfoLink")]
    public string InfoLink { get; set; }
}
