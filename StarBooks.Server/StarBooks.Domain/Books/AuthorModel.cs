using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using StarBooks.Domain.Core.Converters;

namespace StarBooks.Domain.Books;

[Table("authors"), JsonConverter(typeof(AuthorConverter))]
public class AuthorModel
{
    [Key]
    public int Id { get; set; }
    
    public string FirstName { get; set; }
    public string LastName { get; set; }
    
    public List<BookModel> Books { get; set; }
    
    [NotMapped]
    public string FullName => $"{FirstName} {LastName}";

    public override string ToString()
    {
        return FullName;
    }
    
    public static AuthorModel Parse(string author)
    {
        var parts = author.Split(' ');
        return new AuthorModel
        {
            FirstName = parts[0],
            LastName = parts[1]
        };
    }
}